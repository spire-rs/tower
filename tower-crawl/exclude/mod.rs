use std::task::{Context, Poll};

use http::Request;
use tower::{BoxError, Service};

use crate::exclude::future::ResponseFuture;
pub use crate::exclude::layer::ExcludeLayer;
#[doc(inline)]
pub use crate::exclude::policy::Policy;
use crate::exclude::store::Store;

pub mod error;
pub mod future;
mod layer;
pub mod policy;
pub mod store;

/// Conditionally dispatches requests as specified by
/// the retrieved `robots.txt` file and the [`Policy`].
///
/// [`Exclude`]: crate::exclude::Exclude
/// [`Policy`]: crate::exclude::policy::Policy
#[derive(Debug, Clone)]
pub struct Exclude<S, P> {
    inner: S,
    policy: P,
    store: Store,
}

impl<S, P> Exclude<S, P> {
    /// Returns a new [`Exclude`] service wrapping `inner` with the given [`Policy`].
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

impl<S, P, Body> Service<Request<Body>> for Exclude<S, P>
where
    S: Service<Request<Body>> + Clone,
    S::Error: Into<BoxError>,
    P: Policy<Body> + Clone,
    Body: Default,
{
    type Response = S::Response;
    type Error = BoxError;
    type Future = ResponseFuture<S, P, Body>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        ResponseFuture::new(
            req,
            self.inner.clone(),
            self.policy.clone(),
            self.store.clone(),
        )
    }
}

#[cfg(feature = "load")]
#[cfg_attr(docsrs, doc(cfg(feature = "load")))]
impl<S, P> tower::load::Load for Exclude<S, P>
where
    S: tower::load::Load,
{
    type Metric = S::Metric;

    fn load(&self) -> Self::Metric {
        self.inner.load()
    }
}
