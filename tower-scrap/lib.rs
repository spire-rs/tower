#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

mod browser;
mod driver;

mod parse;
mod reduce;

#[cfg(feature = "util")]
#[cfg_attr(docsrs, doc(cfg(feature = "util")))]
mod builder;
#[cfg(feature = "util")]
pub use builder::*;
