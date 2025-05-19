//! Library for parsing command line arguments

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(try_trait_v2)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(box_into_inner)]
#![feature(maybe_uninit_uninit_array_transpose)]

mod argument;
mod command;
mod default_display;
mod error;
mod flag;
mod flag_group;
mod positional;

pub use argument::{
    ArgsOsSource, ArgsSource, Argument, ArgumentSource, OsStrArgument, StrArgument,
};
pub use command::Command;
pub use default_display::DefaultDisplay;
pub use error::{
    Error, InvalidAddressError, InvalidCharError, InvalidLengthError, InvalidNumberError, Result,
    UnexpectedError,
};
pub use flag::{Flag, FlagInfo};
pub use flag_group::FlagGroup;
pub use macros::{Command, FlagGroup};
pub use positional::{Positional, PositionalInfo, PositionalResult};
