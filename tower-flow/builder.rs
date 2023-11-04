use tower::{layer::util::Stack, ServiceBuilder};

mod sealed {
    #[allow(unreachable_pub)]
    pub trait Sealed<T> {}
}

/// Extension trait for the `tower::`[`ServiceBuilder`] for adding
/// middleware from `tower-flow`.
pub trait ServiceBuilderExt<L>: sealed::Sealed<L> + Sized {
    /// Setup a dynamic delay policy before the request execution.
    ///
    /// This wraps the inner service with an instance of the [`DynLimit`]
    /// middleware.
    ///
    /// [`DynLimit`]: crate::limit::DynLimit
    /// [`Policy`]: crate::limit::Policy
    #[cfg(feature = "limit")]
    #[cfg_attr(docsrs, doc(cfg(feature = "limit")))]
    fn dynamic_limit<P>(
        self,
        policy: P,
    ) -> ServiceBuilder<Stack<super::limit::DynLimitLayer<P>, L>>;
}

impl<L> sealed::Sealed<L> for ServiceBuilder<L> {}

impl<L> ServiceBuilderExt<L> for ServiceBuilder<L> {
    #[cfg(feature = "limit")]
    fn dynamic_limit<P>(
        self,
        policy: P,
    ) -> ServiceBuilder<Stack<crate::limit::DynLimitLayer<P>, L>> {
        self.layer(super::limit::DynLimitLayer::new(policy))
    }
}
