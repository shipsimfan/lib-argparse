//! Macros for lib-argparse

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]
#![feature(proc_macro_diagnostic)]

mod command;
mod flag_group;
mod positional;

proc_macro_util::proc_macro_derive!(
    /// Derives the `Command` trait for a structure, allowing it to parse arguments
    ///
    /// Fields can be treated as either flags, flag groups, or positionals. Adding a `flag`
    /// attribute to a field cause it to be a flag. Adding a `flag_group` attribute to a field will
    /// cause it to be a flag group. All fields without a `flag` or `flag_group` attribute will be
    /// treated as a positional, parsed in the order they appear.
    Command (command, flag, arg, flag_group) -> command::generate
);

proc_macro_util::proc_macro_derive!(
    /// Derives the `FlagGroup` trait for a structure
    ///
    /// Fields can have a `flag` attribute attached to give extra information about the flag. See
    /// documentation for `Command` derive for details on the `flag` attribute.
    ///
    /// Fields can have a `flag_group` attribute attached to mark it as a child flag group. This
    /// cannot be marked at the same time as `flag`.
    FlagGroup (flag, flag_group) -> flag_group::generate
);

proc_macro_util::proc_macro_derive!(
    /// Derives the `Positional` trait for an enum with variants that have no fields or only a
    /// single tuple field.
    ///
    /// All variants will be match on a modified version of its name. The name is converted to
    /// lower-case and dashes are inserted when an upper-case letter follows a lower-case letter.
    ///
    /// Variants with a single tuple field will be treated as sub-commands rather than a simple
    /// match, and the type associated will be used as a `Command`.
    Positional -> positional::generate
);
