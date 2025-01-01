use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

pub struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
    executor: Sender<Arc<Task>>,
    active_tasks: Arc<Mutex<usize>>,
    completed: Mutex<bool>,
}

impl Task {
    pub fn spawn<F>(future: F, sender: &Sender<Arc<Task>>, active_tasks: Arc<Mutex<usize>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(),
            active_tasks,
            completed: Mutex::new(false),
        });

        sender.send(task).unwrap();
    }

    pub fn poll(self: Arc<Self>) {
        let mut completed = self.completed.lock().unwrap();
        if *completed {
            return;
        }

        let waker = self.clone().into_waker();
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.lock().unwrap();
        if let Poll::Ready(()) = future.as_mut().poll(&mut cx) {
            *self.active_tasks.lock().unwrap() -= 1;
            *completed = true;
        } else {
            let _ = self.executor.send(self.clone());
        }
    }

    fn into_waker(self: Arc<Self>) -> Waker {
        Waker::from(Arc::new(WakerImpl { task: self }))
    }
}

struct WakerImpl {
    task: Arc<Task>,
}

impl std::task::Wake for WakerImpl {
    fn wake(self: Arc<Self>) {
        let _ = self.task.executor.send(self.task.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;
    use crate::r#async::delay;
    use std::time::Duration;
    use std::thread;

    #[test]
    fn test_task_poll() {
        let (sender, _receiver) = channel::<Arc<Task>>();
        let active_tasks = Arc::new(Mutex::new(1));

        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(async {
                let delay_duration = Duration::from_millis(100);
                delay(delay_duration).await;
            })),
            executor: sender.clone(),
            active_tasks: Arc::clone(&active_tasks),
            completed: Mutex::new(false),
        });

        while !*task.completed.lock().unwrap() {
            let task_clone = Arc::clone(&task);
            task_clone.poll();
            thread::sleep(Duration::from_millis(10));
        }

        assert_eq!(*task.completed.lock().unwrap(), true);
        assert_eq!(*task.active_tasks.lock().unwrap(), 0);
    }
}
