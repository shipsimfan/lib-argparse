//! Library for parsing command line arguments

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(try_trait_v2)]
#![feature(os_str_display)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(box_into_inner)]
#![feature(maybe_uninit_uninit_array)]

mod argument;
mod command;
mod default_display;
mod error;
mod flag;
mod positional;

pub use argument::{
    ArgsOsSource, ArgsSource, Argument, ArgumentSource, OsStrArgument, StrArgument,
};
pub use command::Command;
pub use default_display::DefaultDisplay;
pub use error::{Error, InvalidAddressError, InvalidCharError, InvalidNumberError, Result};
pub use flag::{Flag, FlagInfo};
pub use macros::Command;
pub use positional::{Positional, PositionalInfo, PositionalResult};
