#![doc = include_str!("../README.md")] 

#![cfg_attr(docsrs, feature(doc_cfg))]

//#![cfg_attr(feature = "no_std", no_std)]

mod inc_dec_exts;

pub use inc_dec_exts::*;

mod macros;

pub use macros::*;

mod traits;

pub use traits::*;

mod int_inc_dec_exts;

pub use inc_dec_exts::*;

#[cfg(feature = "num")]
mod num;

#[cfg(test)]
mod inc_dec_ext_tests;

#[cfg(test)]
mod int_inc_dec_ext_tests;
