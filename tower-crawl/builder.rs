use tower::{layer::util::Stack, ServiceBuilder};

mod sealed {
    #[allow(unreachable_pub)]
    pub trait Sealed<T> {}
}

/// Extension trait for the `tower::`[`ServiceBuilder`] for adding middlewares.
pub trait ServiceBuilderExt<L>: sealed::Sealed<L> + Sized {
    /// Conditionally dispatches requests to the inner service according to
    /// the retrieved `robots.txt` file and the provided [`Policy`].
    ///
    /// This wraps the inner service with an instance of the [`Exclude`]
    /// middleware.
    ///
    /// [`Exclude`]: crate::exclude::Exclude
    /// [`Policy`]: crate::exclude::Policy
    #[cfg(feature = "exclude")]
    #[cfg_attr(docsrs, doc(cfg(feature = "exclude")))]
    fn exclude_pages<P>(
        self,
        policy: P,
        store: crate::exclude::store::Store,
    ) -> ServiceBuilder<Stack<super::exclude::ExcludeLayer<P>, L>>;

    /// Populates the task queue with addresses discovered in the retrieved
    /// sitemaps according to the provided [`Policy`].
    ///
    /// This wraps the inner service with an instance of the [`Include`]
    /// middleware.
    ///
    /// [`Include`]: crate::include::Include
    /// [`Policy`]: crate::include::Policy
    #[cfg(feature = "include")]
    #[cfg_attr(docsrs, doc(cfg(feature = "include")))]
    fn include_pages<P>(
        self,
        policy: P,
        store: crate::exclude::store::Store,
    ) -> ServiceBuilder<Stack<super::include::IncludeLayer<P>, L>>;
}

impl<L> sealed::Sealed<L> for ServiceBuilder<L> {}

impl<L> ServiceBuilderExt<L> for ServiceBuilder<L> {
    #[cfg(feature = "exclude")]
    fn exclude_pages<P>(
        self,
        policy: P,
        store: crate::exclude::store::Store,
    ) -> ServiceBuilder<Stack<super::exclude::ExcludeLayer<P>, L>> {
        self.layer(super::exclude::ExcludeLayer::new(policy, store))
    }

    #[cfg(feature = "include")]
    fn include_pages<P>(
        self,
        policy: P,
        store: crate::exclude::store::Store,
    ) -> ServiceBuilder<Stack<crate::include::IncludeLayer<P>, L>> {
        self.layer(super::include::IncludeLayer::new(policy, store))
    }
}
