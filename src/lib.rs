//! # lib-argparse
//! Library for parsing command line arguments
//!
//! ## Concepts
//! **TODO**
//!
//! ## Usage
//! **TODO**

#![deny(missing_docs)]
#![feature(const_trait_impl)]

mod error;
mod flag_argument;
mod parser;

pub use error::{Error, Result};
pub use flag_argument::{
    FlagArgument, FlagClass, HelpFlagArgument, ParsingFlagArgument, SimpleFlagArgument,
    VersionFlagArgument,
};
pub use macros::{help_flag, parser};
pub use parser::Parser;
