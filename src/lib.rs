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
#![feature(const_mut_refs)]

mod error;
mod flag_argument;
mod parser;
mod terminal_argument;

pub use error::{Error, Result};
pub use flag_argument::{
    ConfigFlagArgument, FlagArgument, FlagClass, HelpFlagArgument, ParsingFlagArgument,
    SimpleFlagArgument, VersionFlagArgument,
};
pub use macros::{
    command, commands, config_flag, help_flag, parser, parsing_flag, parsing_positional,
    positionals, simple_flag, simple_positional, version_flag,
};
pub use parser::Parser;
pub use terminal_argument::{
    Command, Commands, ParsingPositionalArgument, PositionalArgument, PositionalTerminalArgument,
    SimplePositionalArgument, TerminalArgument,
};
