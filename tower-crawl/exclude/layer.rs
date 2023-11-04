use tower::Layer;

use crate::exclude::store::Store;
use crate::exclude::Exclude;

/// Conditionally dispatches requests as specified by
/// the retrieved `robots.txt` file and the [`Policy`].
///
/// This [`Layer`] produces instances of the [`Exclude`] service with
/// the given [`Policy`].
///
/// [`Request`]: http::Request
/// [`Policy`]: crate::exclude::policy::Policy
#[derive(Debug, Clone)]
pub struct ExcludeLayer<P> {
    store: Store,
    policy: P,
}

impl<P> ExcludeLayer<P> {
    /// Returns a new layer that produces [`Exclude`] services with the given [`Policy`].
    ///
    /// [`Policy`]: crate::exclude::Policy
    pub fn new(policy: P, store: Store) -> Self {
        Self { policy, store }
    }
}

impl<S, P> Layer<S> for ExcludeLayer<P>
where
    P: Clone,
{
    type Service = Exclude<S, P>;

    fn layer(&self, inner: S) -> Self::Service {
        Exclude::new(inner, self.policy.clone(), self.store.clone())
    }
}
