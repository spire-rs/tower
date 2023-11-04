#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

#[cfg(feature = "batch")]
#[cfg_attr(docsrs, doc(cfg(feature = "batch")))]
mod batch;
#[cfg(feature = "limit")]
#[cfg_attr(docsrs, doc(cfg(feature = "limit")))]
mod limit;
#[cfg(feature = "report")]
#[cfg_attr(docsrs, doc(cfg(feature = "report")))]
mod report;
#[cfg(feature = "trace")]
#[cfg_attr(docsrs, doc(cfg(feature = "trace")))]
mod trace;

#[cfg(feature = "util")]
#[cfg_attr(docsrs, doc(cfg(feature = "util")))]
mod builder;
#[cfg(feature = "util")]
pub use builder::*;

mod cluster;
mod rayon;
#[cfg(feature = "limit")]
#[cfg_attr(docsrs, doc(cfg(feature = "limit")))]
pub mod limit;
