#![doc = include_str!("../README.md")]
#![warn(clippy::missing_docs_in_private_items)]
#![warn(missing_docs)] // Duplicate, but this can report the missing earlier
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(rustdoc::all)]
#![allow(rustdoc::missing_doc_code_examples)]
#![feature(generic_associated_types)]
#![feature(never_type)]
#![feature(doc_cfg)]
#![feature(extend_one)]



mod codeblock;
mod cowstr;

pub mod directory;
pub mod dispatch;
pub mod read;

pub mod write_blocks;
#[doc(inline)]
pub use write_blocks::{write_blocks, write_blocks_errless};



#[cfg(test)]
mod tests {}
