#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

// Middleware.

#[cfg(feature = "exclude")]
#[cfg_attr(docsrs, doc(cfg(feature = "exclude")))]
pub mod exclude;
#[cfg(feature = "include")]
#[cfg_attr(docsrs, doc(cfg(feature = "include")))]
pub mod include;

// Builder.

#[cfg(feature = "util")]
#[cfg_attr(docsrs, doc(cfg(feature = "util")))]
mod builder;
#[cfg(feature = "util")]
pub use builder::*;
