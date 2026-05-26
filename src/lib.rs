#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_cfg))]

mod inc_dec_exts;

pub use inc_dec_exts::*;

mod macros;

pub use macros::*;

mod traits;

pub use traits::*;

mod int_inc_dec_exts;

pub use inc_dec_exts::*;

#[cfg(test)]
mod tests;
