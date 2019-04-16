macro_rules! benchmark_suite {
    ($rt:expr) => {
        #[runtime::bench($rt)]
        async fn smoke() {}

        #[runtime::bench($rt)]
        async fn notify_self() {
            use futures::future::Future;
            use futures::task::{Context, Poll};
            use std::pin::Pin;

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
                        w.wake();
                        Poll::Pending
                    }
                }
            }

            let tasks = (0..300)
                .map(|_| {
                    runtime::spawn(async {
                        await!(Task { depth: 0 });
                    })
                })
                .collect::<Vec<_>>();

            for task in tasks {
                await!(task);
            }
        }

        #[runtime::bench($rt)]
        async fn spawn_many() {
            let tasks = (0..25_000)
                .map(|_| runtime::spawn(async {}))
                .collect::<Vec<_>>();

            for task in tasks {
                await!(task);
            }
        }

        #[runtime::bench($rt)]
        async fn poll_reactor() {
            use futures::compat::Compat01As03;
            use futures::future::FutureExt;
            use futures01::{future, Async};
            use tokio::reactor::Registration;

            let tasks = (0..300)
                .map(|_| {
                    runtime::spawn(async {
                        let (r, s) = mio::Registration::new2();
                        let registration = Registration::new();
                        registration.register(&r).unwrap();

                        let mut depth = 0;
                        let mut capture = Some(r);

                        runtime::spawn(
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
        }
    };
}
