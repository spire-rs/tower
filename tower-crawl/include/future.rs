//! Future types for the [`Include`] middleware.
//!
//! [`Include`]: crate::include::Include

use http::Request;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::exclude::store::Store;
use pin_project_lite::pin_project;
use tower::Service;

pin_project! {
    /// The [`Future`] returned by a [`Include`] service.
    ///
    /// [`Include`]: crate::exclude::Include
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ResponseFuture<S, P, Body>
    where
        S: Service<Request<Body>>,
    {
        #[pin] future: S::Future,
        policy: P,
        store: Store,
    }
}

impl<S, P, Body> ResponseFuture<S, P, Body>
where
    S: Service<Request<Body>>,
{
    pub(crate) fn new(future: S::Future, policy: P, store: Store) -> Self {
        Self {
            future,
            policy,
            store,
        }
    }
}

impl<S, P, Body> Future for ResponseFuture<S, P, Body>
where
    S: Service<Request<Body>>,
    S::Future: Future<Output = Result<S::Response, S::Error>>,
{
    type Output = Result<S::Response, S::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        this.future.poll(cx)
    }
}
