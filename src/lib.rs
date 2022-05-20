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



mod codeblock;
mod cowstr;

pub mod read;

pub mod directory;

pub mod dispatch;



#[cfg(test)]
mod tests {}
