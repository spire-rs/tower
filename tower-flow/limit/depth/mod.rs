use std::num::NonZeroUsize;
use std::task::{Context, Poll};

use tower::Service;

pub mod error;
pub mod future;
mod layer;

/// Filters out requests with a recursion [`depth`] exceeding the limit
///
/// [`depth`]: Depth
#[derive(Debug, Clone)]
pub struct DepthLimit<S> {
    inner: S,
    depth: NonZeroUsize,
}

impl<S> DepthLimit<S> {
    /// Returns a new [`DepthLimit`] service wrapping `inner` with the given depth limit.
    pub fn new(inner: S, depth: NonZeroUsize) -> Self {
        Self { inner, depth }
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
}

impl<S, Req> Service<Req> for DepthLimit<Req>
where
    S: Service<Req>,
    Req: Depth,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        todo!()
    }
}

pub trait Depth {
    fn depth(&self) -> usize;
}
