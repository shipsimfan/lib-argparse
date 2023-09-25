//! # lib-argparse
//!
//! Library for parsing arguments
//!
//! ## Concept
//!
//! Argument parsing is performed in a tree. Each node of the tree is a `Parser`.
//! `Parser`s can have any number of `NamedArgument`s and one `TerminalArgument`.
//!
//! ### Named Arguments
//!
//! `NamedArgument`s are arguments that start with '-' or '--'
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
