#![feature(test, async_await, await_macro)]

extern crate test;

mod baseline {
    use futures::executor;
    use futures::prelude::*;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    #[bench]
    fn smoke(b: &mut test::Bencher) {
        b.iter(|| {
            executor::block_on(async {
                juliex::spawn(async move {});
            })
        });
    }

    #[bench]
    fn notify_self(b: &mut test::Bencher) {
        b.iter(|| {
            executor::block_on(async {
                struct Task {
                    depth: usize,
                }

                impl Future for Task {
                    type Output = ();

                    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                        self.depth += 1;

                        if self.depth == 300 {
                            Poll::Ready(())
                        } else {
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        }
                    }
                }

                let tasks = (0..300)
                    .map(|_| {
                        spawn(async move {
                            await!(Task { depth: 0 });
                        })
                    })
                    .collect::<Vec<_>>();

                for task in tasks {
                    await!(task);
                }
            })
        });
    }

    #[bench]
    fn spawn_many(b: &mut test::Bencher) {
        b.iter(|| {
            executor::block_on(async {
                let tasks = (0..25_000).map(|_| spawn(async {})).collect::<Vec<_>>();

                for task in tasks {
                    await!(task);
                }
            })
        });
    }

    #[bench]
    fn poll_reactor(b: &mut test::Bencher) {
        use futures::compat::Compat01As03;
        use futures::future::FutureExt;
        use futures01::{future, Async};
        use tokio::reactor::Registration;
        b.iter(|| {
            executor::block_on(async {
                let tasks = (0..300)
                    .map(|_| {
                        spawn(async {
                            let (r, s) = mio::Registration::new2();
                            let registration = Registration::new();
                            registration.register(&r).unwrap();

                            let mut depth = 0;
                            let mut capture = Some(r);

                            spawn(
                                Compat01As03::new(future::poll_fn(move || loop {
                                    if registration.poll_read_ready().unwrap().is_ready() {
                                        depth += 1;
                                        if depth == 300 {
                                            capture.take().unwrap();
                                            return Ok(Async::Ready(()));
                                        }
                                    } else {
                                        s.set_readiness(mio::Ready::readable()).unwrap();
                                        return Ok(Async::NotReady);
                                    }
                                }))
                                .map(|_: Result<(), ()>| ()),
                            )
                        })
                    })
                    .collect::<Vec<_>>();

                for task in tasks {
                    await!(task);
                }
            })
        });
    }

    /// Spawn function for juliex to get back a handle
    pub fn spawn<F, T>(fut: F) -> JoinHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = futures::channel::oneshot::channel();

        let fut = async move {
            let t = await!(fut);
            let _ = tx.send(t);
        };

        juliex::spawn(fut);
        JoinHandle { rx }
    }

    /// Handle returned from Juliex.
    // We should patch Juliex to support this natively, and be more efficient on channel use.
    #[derive(Debug)]
    pub struct JoinHandle<T> {
        pub(crate) rx: futures::channel::oneshot::Receiver<T>,
    }

    impl<T> Future for JoinHandle<T> {
        type Output = T;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match self.rx.poll_unpin(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(Ok(t)) => Poll::Ready(t),
                Poll::Ready(Err(_)) => panic!(), // TODO: Is this OK? Print a better error message?
            }
        }
    }
}
