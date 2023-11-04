//! TODO: Desc.

use http::uri::Scheme;
use http::Request;
use robotxt::{Robots, ALL_UAS};

/// TODO: Desc.
pub trait Policy<Body> {
    /// Returns true if the [`Request`]'s [`URI`] should be matched to
    /// the retrieved `robots.txt` file.
    ///
    /// [`URI`]: http::Uri
    fn is_checked(&self, req: &Request<Body>) -> bool {
        if let Some(scheme) = req.uri().scheme() {
            [Scheme::HTTP, Scheme::HTTPS].contains(scheme)
        } else {
            false
        }
    }

    /// Returns true if the [`Request`] is allowed to be sent.
    fn is_allowed(&self, req: &Request<Body>, is_allowed: bool) -> bool;
}

impl<T, Body> Policy<Body> for T
where
    T: Fn(&Request<Body>, bool) -> bool,
{
    fn is_allowed(&self, req: &Request<Body>, is_allowed: bool) -> bool {
        (self)(req, is_allowed)
    }
}

// Strictly follows the `robots.txt` file with a list of exceptions.
#[derive(Debug, Clone, Default)]
pub struct ExceptBuilder {
    allow: Vec<String>,
    disallow: Vec<String>,
}

impl ExceptBuilder {
    /// Creates a new [`Except`] policy builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// TODO: Desc.
    pub fn allow(mut self, path: &str) -> Self {
        self.allow.push(path.to_string());
        self
    }

    /// TODO: Desc.
    pub fn disallow(mut self, path: &str) -> Self {
        self.disallow.push(path.to_string());
        self
    }

    /// TODO: Desc.
    pub fn build(self) -> Except {
        let builder = Robots::builder()
            .group([ALL_UAS], |mut builder| {
                for rule in self.allow.iter() {
                    builder = builder.allow(rule);
                }

                for rule in self.allow.iter() {
                    builder = builder.disallow(rule);
                }

                builder
            })
            .parse(ALL_UAS);

        Except::new(builder)
    }
}

/// Strictly follows the `robots.txt` file with a list of exceptions.
#[derive(Debug, Clone)]
pub struct Except {
    inner: Robots,
}

impl Except {
    /// Creates a new [`Except`] policy.
    pub fn new(inner: Robots) -> Self {
        Self { inner }
    }

    /// Creates a new [`Except`] policy builder.
    pub fn builder() -> ExceptBuilder {
        ExceptBuilder::default()
    }
}

impl<Body> Policy<Body> for Except {
    fn is_allowed(&self, req: &Request<Body>, is_allowed: bool) -> bool {
        let path = req.uri().path_and_query();
        path.map(|x| x.as_str())
            .and_then(|x| self.inner.try_is_relative_allowed(x))
            .unwrap_or(is_allowed)
    }
}

impl From<Robots> for Except {
    fn from(value: Robots) -> Self {
        Self::new(value)
    }
}
