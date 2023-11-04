mod layer;

#[derive(Debug, Clone)]
pub struct InBatch<S> {
    inner: S,
    batch: usize,
}

impl<S> InBatch<S> {
    /// Returns a new [`InBatch`] service wrapping `inner` with the given [`Policy`].
    pub fn new(inner: S, batch: usize) -> Self {
        Self { inner, batch }
    }
}
