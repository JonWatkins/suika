use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::Duration;

pub async fn delay(dur: Duration) {
    struct Delay {
        when: std::time::Instant,
        waker: Option<Arc<Mutex<Option<Waker>>>>,
        completed: bool,
    }

    impl Future for Delay {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.completed {
                panic!("`async fn` resumed after completion");
            }

            if let Some(waker) = &self.waker {
                let mut waker = waker.lock().unwrap();
                if !waker.as_ref().unwrap().will_wake(cx.waker()) {
                    *waker = Some(cx.waker().clone());
                }
            } else {
                let when = self.when;
                let waker = Arc::new(Mutex::new(Some(cx.waker().clone())));
                self.waker = Some(waker.clone());

                thread::spawn(move || {
                    let now = std::time::Instant::now();
                    if now < when {
                        thread::sleep(when - now);
                    }
                    let waker = waker.lock().unwrap();
                    if let Some(waker) = &*waker {
                        waker.wake_by_ref();
                    }
                });
            }

            if std::time::Instant::now() >= self.when {
                self.completed = true;
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    let future = Delay {
        when: std::time::Instant::now() + dur,
        waker: None,
        completed: false,
    };

    future.await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};

    fn block_on<F: Future>(future: F) -> F::Output {
        use std::task::{RawWaker, RawWakerVTable};

        fn no_op(_: *const ()) {}
        fn clone_waker(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }

        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_waker, no_op, no_op, no_op);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
        let mut cx = Context::from_waker(&waker);
        let mut future = Box::pin(future);

        loop {
            match future.as_mut().poll(&mut cx) {
                Poll::Ready(val) => return val,
                Poll::Pending => thread::yield_now(),
            }
        }
    }

    #[test]
    fn test_delay_function_short_delay() {
        let start = Instant::now();
        let delay_duration = Duration::from_millis(1);
        block_on(delay(delay_duration));
        let elapsed = start.elapsed();
        assert!(elapsed >= delay_duration);
    }

    #[test]
    fn test_delay_function_long_delay() {
        let start = Instant::now();
        let delay_duration = Duration::from_millis(500);
        block_on(delay(delay_duration));
        let elapsed = start.elapsed();
        assert!(elapsed >= delay_duration);
    }
}
