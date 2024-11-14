//! Library for parsing command line arguments

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod argument;
mod error;

pub use argument::{Argument, FromArgument, OsStrArgument, StrArgument};
pub use error::Error;

mod messages {
    #![allow(missing_docs)]
    i18n::include_fluent!("fluent");
}
