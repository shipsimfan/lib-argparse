//! Macros for lib-argparse

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

mod command;

proc_macro_util::proc_macro_derive!(
    /// Derives the `Command` trait for a structure, allowing it to parse arguments
    ///
    /// Fields can be treated as either flags or positionals. Adding a `flag` attribute to a field
    /// cause it to be a flag. All fields without a `flag` attribute will be treated as a
    /// positional, parsed in the order they appear.
    Command (command, flag, arg) -> command::generate
);
