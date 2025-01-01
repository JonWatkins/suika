use crate::task::Task;
use std::future::Future;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
pub struct SimpleAsync {
    scheduled: Arc<Mutex<Receiver<Arc<Task>>>>,
    sender: Sender<Arc<Task>>,
    stop_sender: Sender<()>,
    stop_receiver: Arc<Mutex<Receiver<()>>>,
    active_tasks: Arc<Mutex<usize>>,
}

impl SimpleAsync {
    pub fn new() -> SimpleAsync {
        let (sender, scheduled) = mpsc::channel::<Arc<Task>>();
        let (stop_sender, stop_receiver) = mpsc::channel();
        SimpleAsync {
            scheduled: Arc::new(Mutex::new(scheduled)),
            sender,
            stop_sender,
            stop_receiver: Arc::new(Mutex::new(stop_receiver)),
            active_tasks: Arc::new(Mutex::new(0)),
        }
    }

    pub fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        *self.active_tasks.lock().unwrap() += 1;
        Task::spawn(future, &self.sender, Arc::clone(&self.active_tasks));
    }

    pub fn run(&self) {
        loop {
            let stop_receiver = self.stop_receiver.lock().unwrap();
            if let Ok(_) = stop_receiver.try_recv() {
                break;
            }
            drop(stop_receiver);

            let task_option = self.scheduled.lock().unwrap().try_recv();
            match task_option {
                Ok(task) => {
                    task.poll();
                }
                Err(mpsc::TryRecvError::Empty) => {
                    thread::yield_now();
                }
                Err(mpsc::TryRecvError::Disconnected) => {
                    break;
                }
            }
        }
        println!("Executor stopped.");
    }

    pub fn stop(&self) {
        let _ = self.stop_sender.send(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::delay;
    use std::sync::mpsc::channel;
    use std::time::Duration;

    #[test]
    fn test_simple_async_no_tasks() {
        let simple_async = Arc::new(SimpleAsync::new());

        let simple_async_thread = Arc::clone(&simple_async);
        let handle = std::thread::spawn(move || {
            simple_async_thread.run();
        });

        simple_async.stop();
        handle.join().unwrap();
    }

    #[test]
    fn test_simple_async_multiple_tasks() {
        let simple_async = Arc::new(SimpleAsync::new());
        let (tx, rx) = channel();

        for i in 0..5 {
            let tx_clone = tx.clone();
            simple_async.spawn(async move {
                delay(Duration::from_millis(50 * i)).await;
                tx_clone.send(i).unwrap();
                println!("Task {} completed", i);
            });
        }

        let simple_async_thread = Arc::clone(&simple_async);
        let handle = std::thread::spawn(move || {
            simple_async_thread.run();
        });

        let mut results = vec![];
        for _ in 0..5 {
            results.push(rx.recv().unwrap());
        }

        simple_async.stop();
        handle.join().unwrap();

        results.sort();
        assert_eq!(results, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_simple_async_nested_delays() {
        let simple_async = Arc::new(SimpleAsync::new());
        let (tx, rx) = channel();

        simple_async.spawn(async move {
            delay(Duration::from_millis(100)).await;
            println!("First delay completed");
            delay(Duration::from_millis(100)).await;
            println!("Second delay completed");
            tx.send("nested delays completed").unwrap();
        });

        let simple_async_thread = Arc::clone(&simple_async);
        let handle = std::thread::spawn(move || {
            simple_async_thread.run();
        });

        let result = rx.recv().unwrap();

        simple_async.stop();
        handle.join().unwrap();

        assert_eq!(result, "nested delays completed");
    }
}
