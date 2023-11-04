use std::num::NonZeroUsize;

#[derive(Debug, Clone, Default)]
pub struct DepthLimitLayer {
    depth: Option<NonZeroUsize>,
}

impl DepthLimitLayer {
    /// Returns a new layer that produces [`DepthLimit`] services with the given depth.
    ///
    pub fn new(depth: usize) -> Self {
        let depth = NonZeroUsize::new(depth).expect("should not be zero");
        Self { depth: Some(depth) }
    }
}
