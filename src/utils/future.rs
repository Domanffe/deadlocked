use std::{
    future::Future,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
    thread,
};

thread_local! {
    static LOCAL_WAKER: Waker = {
        let signal = Signal(thread::current());
        Waker::from(Arc::new(signal))
    };
}

struct Signal(thread::Thread);

impl Wake for Signal {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.unpark();
    }
}

pub trait FutureExt: Future {
    fn block(self) -> Self::Output
    where
        Self: Sized,
    {
        let mut fut = std::pin::pin!(self);

        LOCAL_WAKER.with(|waker| {
            let mut context = Context::from_waker(waker);

            loop {
                match fut.as_mut().poll(&mut context) {
                    Poll::Pending => thread::park(),
                    Poll::Ready(item) => break item,
                }
            }
        })
    }
}

impl<F: Future + ?Sized> FutureExt for F {}

#[cfg(test)]
mod test {
    use super::FutureExt;

    #[test]
    fn block() {
        const VALUE: i32 = 5;

        async fn work() -> i32 {
            VALUE
        }

        assert_eq!(work().block(), VALUE);
    }
}
