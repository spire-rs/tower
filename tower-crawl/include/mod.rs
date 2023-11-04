use http::Request;
use std::task::{Context, Poll};

use tower::Service;

use crate::exclude::store::Store;
use crate::include::future::ResponseFuture;
pub use crate::include::layer::IncludeLayer;
pub use crate::include::policy::Policy;

pub mod future;
mod layer;
mod policy;
mod store;

/// Populates the task queue with addresses discovered in the retrieved
/// sitemaps according to the provided [`Policy`].
///
/// [`Include`]: crate::include::Include
/// [`Policy`]: crate::include::Policy
#[derive(Debug, Clone)]
pub struct Include<S, P> {
    inner: S,
    policy: P,
    store: Store,
}

impl<S, P> Include<S, P> {
    /// Returns a new [`Include`] service wrapping `inner` with the given [`Policy`].
    pub fn new(inner: S, policy: P, store: Store) -> Self {
        Self {
            inner,
            policy,
            store,
        }
    }

    /// Gets a reference to the inner service.
    pub fn get_ref(&self) -> &S {
        &self.inner
    }

    /// Gets a mutable reference to the inner service.
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.inner
    }

    /// Consumes `self`, returning the inner service.
    pub fn into_inner(self) -> S {
        self.inner
    }

    /// Gets a copy of the applied policy.
    pub fn policy(&self) -> P
    where
        P: Clone,
    {
        self.policy.clone()
    }
}

impl<S, P, Body> Service<Request<Body>> for Include<S, P>
where
    S: Service<Request<Body>>,
    P: Policy + Clone,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<S, P, Body>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        ResponseFuture::new(
            self.inner.call(req),
            self.policy.clone(),
            self.store.clone(),
        )
    }
}

#[cfg(feature = "load")]
#[cfg_attr(docsrs, doc(cfg(feature = "load")))]
impl<S, P> tower::load::Load for Include<S, P>
where
    S: tower::load::Load,
{
    type Metric = S::Metric;

    fn load(&self) -> Self::Metric {
        self.inner.load()
    }
}
