use tower::Layer;

use crate::exclude::store::Store;
use crate::include::Include;

/// Populates the task queue with addresses discovered in the retrieved
/// sitemaps according to the provided [`Policy`].
///
/// This [`Layer`] produces instances of the [`Include`] service with
/// the given [`Policy`].
///
/// [`Policy`]: crate::include::Policy
#[derive(Debug, Clone)]
pub struct IncludeLayer<P> {
    store: Store,
    policy: P,
}

impl<P> IncludeLayer<P> {
    /// Returns a new layer that produces [`IncludeLayer`] services with the given [`Policy`].
    ///
    /// [`Policy`]: crate::include::Policy
    pub fn new(policy: P, store: Store) -> Self {
        Self { policy, store }
    }
}

impl<S, P> Layer<S> for IncludeLayer<P>
where
    P: Clone,
{
    type Service = Include<S, P>;

    fn layer(&self, inner: S) -> Self::Service {
        Include::new(inner, self.policy.clone(), self.store.clone())
    }
}
