//! Library for parsing command line arguments

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod argument;
mod error;

mod messages;

pub use argument::{Argument, OsStrArgument, StrArgument};
pub use error::Error;
