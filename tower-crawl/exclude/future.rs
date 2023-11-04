//! Future types for the [`Exclude`] middleware.
//!
//! [`Exclude`]: crate::exclude::Exclude

use std::future::Future;
use std::pin::Pin;
use std::task::{ready, Context, Poll};

use http::{Method, Request, StatusCode, Uri};
use pin_project_lite::pin_project;
use tower::{BoxError, Service};

use crate::exclude::error::Denied;
use crate::exclude::policy::Policy;
use crate::exclude::store::{Status, Store};

pin_project! {
    /// Internal state of the [`ResponseFuture`].
    #[project = StateProj]
    #[derive(Debug)]
    enum State<F, Body> {
        /// This [`ResponseFuture`] has not accessed the store yet or
        /// the other [`ResponseFuture`] has already locked it.
        Waiting { request: Option<Request<Body>> },
        /// This [`ResponseFuture`] has locked the store for the `robots.txt` retrieval.
        /// with the [`future`]
        Locking { #[pin] future: F, request: Option<Request<Body>> },
        /// This [`ResponseFuture`] has already retrieved and checked
        /// the `robots.txt` file, the request is processed now with
        /// the `future`.
        Working { #[pin] future: F },
    }
}

pin_project! {
    /// The [`Future`] returned by a [`Exclude`] service.
    ///
    /// [`Exclude`]: crate::exclude::Exclude
    #[project = ResponseFutureProj]
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ResponseFuture<S, P, Body>
    where
        S: Service<Request<Body>>,
    {
        #[pin] state: State<S::Future, Body>,
        inner: S,
        policy: P,
        store: Store,
    }

    impl<S, P, Body> PinnedDrop for ResponseFuture<S, P, Body>
    where
        S: Service<Request<Body>>,
    {
        fn drop(this: Pin<&mut Self>) {
            let mut this = this.project();
            let mut state = this.state.as_mut().project();
            if let StateProj::Locking { request, .. } = state {
                let request = request.as_ref().expect("should be present");
                this.store.cancel(request.uri());
            }

            // The `PinnedDrop` cannot be called the 2nd time,
            // therefore there is no need to reset the `State`.
        }
    }
}

impl<S, P, Body> ResponseFuture<S, P, Body>
where
    S: Service<Request<Body>>,
    P: Policy<Body>,
{
    pub(crate) fn new(req: Request<Body>, mut inner: S, policy: P, store: Store) -> Self {
        let state = if policy.is_checked(&req) {
            let request = Some(req);
            State::Waiting { request }
        } else {
            let future = inner.call(req);
            State::Working { future }
        };

        Self {
            state,
            inner,
            store,
            policy,
        }
    }
}

impl<S, P, Body> Future for ResponseFuture<S, P, Body>
where
    S: Service<Request<Body>>,
    S::Error: Into<BoxError>,
    P: Policy<Body>,
    Body: Default,
{
    type Output = Result<S::Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        loop {
            match this.state.as_mut().project() {
                StateProj::Waiting { request } => {
                    let uri = request.as_ref().expect("should be present").uri();
                    match this.store.access(uri) {
                        Status::Unknown => {
                            let request = request.take().expect("should be present");
                            this.state.set(State::Working {
                                future: this.inner.call(request),
                            });
                        },

                        Status::Locking(robots) => {
                            let request = request.take().expect("should be present");
                            let request0 = create_request(robots);
                            this.state.set( State::Locking {
                                future: this.inner.call(request0),
                                request: Some(request),
                            });
                        }

                        Status::Waiting => return Poll::Pending,
                        Status::Ready(robots) => {
                            let path = uri.path_and_query().map(|path| path.as_str());
                            if path.is_some_and(|u| !robots.is_relative_allowed(u)) {
                                return Poll::Ready(Err(Denied::new().into()));
                            }

                            let request = request.take().expect("should be present");
                            this.state.set(State::Working {
                                future: this.inner.call(request),
                            });
                        }
                    }
                }
                StateProj::Locking { request, future } => {
                    let poll: Result<S::Response, S::Error> = ready!(future.poll(cx));
                    let uri = request.as_ref().expect("should be present").uri();

                    // TODO: Convert into robots.txt.
                    // this.store.store(uri, StatusCode, &[])

                    let request = request.take().expect("should be present");
                    this.state.set(State::Waiting {
                        request: Some(request),
                    });
                }
                StateProj::Working { future } => {
                    let poll: Result<S::Response, S::Error> = ready!(future.poll(cx));
                    return Poll::Ready(poll.map_err(Into::into));
                }
            }
        }
    }
}

fn create_request<Body>(address: Uri) -> Request<Body>
where
    Body: Default,
{
    Request::builder()
        .method(Method::GET)
        .uri(address)
        .body(Body::default())
        .expect("should be contractible")
}
