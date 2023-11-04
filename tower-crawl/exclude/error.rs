//! Error types for the [`Exclude`] middleware.
//!
//! [`Exclude`]: crate::exclude::Exclude

#[derive(Debug, Default, thiserror::Error)]
#[error("access disallowed by the `robots.txt` file")]
pub struct Denied {
    patten: Option<String>,
}

impl Denied {
    /// Creates a new [`Denied`] error.
    pub fn new() -> Self {
        Self::default()
    }
}
