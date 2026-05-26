#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_cfg))]

mod inc_dec;

pub use inc_dec::*;

mod macros;

pub use macros::*;

mod traits;

pub use traits::*;

#[cfg(test)]
mod tests;
