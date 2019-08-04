#![feature(test, async_await)]

extern crate test;

mod baseline {
    use futures::executor;
    use futures::future::RemoteHandle;
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
                            Task { depth: 0 }.await;
                        })
                    })
                    .collect::<Vec<_>>();

                for task in tasks {
                    task.await;
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
                    task.await;
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
                    task.await;
                }
            })
        });
    }

    /// Spawn function for juliex to get back a handle
    pub fn spawn<F, T>(fut: F) -> RemoteHandle<T>
    where
        F: Future<Output = T> + Send + 'static,
        T: Send + 'static,
    {
        let (fut, handle) = fut.remote_handle();

        juliex::spawn(fut);

        handle
    }
}
