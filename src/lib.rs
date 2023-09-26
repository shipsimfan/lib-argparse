//! # lib-argparse
//!
//! Library for parsing arguments
//!
//! ## Concept
//!
//! Argument parsing is performed in a tree. Each node of the tree is a `Parser`.
//! `Parser`s can have any number of `FlagArgument`s and one `TerminalArgument`.
//!
//! ### Flag Arguments
//!
//! `FlagArgument`s are arguments that start with '-' or '--'
//! A `FlagArgument` can be one of the following:
//!  - `HelpFlag`
//!  - `ActionFlag`
//!  - `ValueFlag`
//!
//! #### Help Flag
//! A `HelpFlag` generates and displays a help for the current parser and exits the program when matched.
//!
//! #### Action Flag
//! An `ActionFlag` runs a function on matching.
//!
//! The function can take one of the following as parameters:
//!  - `()`
//!  - `(&str)`
//!  - `(String)`
//!  - `(Vec<String>)`
//!
//! The function can return one of the following:
//!  - `()`
//!  - `Result<(), E>` where `E` is any type
//!
//! #### Value Flag
//! A `ValueFlag` will construct a value using a `ValueParser` and pass it to a function.
//! The `ValueParser` is given access to the argument stream so it can be variable length.
//! The function can return the same types as an action parser.
//!
//! ### Terminal Arguments
//! `TerminalArgument`s are how the tree of `Parser`s is formed.
//! A `TerminalArgument` can be one of the following:
//!  - `None`
//!  - `Command`
//!  - `Positionals`
//!
//! #### Commands
//! `Command`s form the branches of the tree.
//! `Command`s are made of an unordered list of strings representing commands.
//! Each command leads to a `Parser` which continues parsing the remaining arguments.
//!
//! #### Positionals
//! `Positionals` form the leaves of the tree.
//! `Positionals` are made from an ordered list of `PositionalParser`s.

mod flag;
mod parser;

pub use flag::{ActionFlag, FlagArgument, FlagKind};
pub use parser::Parser;
