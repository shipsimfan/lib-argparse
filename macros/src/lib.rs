use proc_macro_util::proc_macro_function;

proc_macro_function!(
    /// Creates a constant time argument [`Parser`]
    ///
    /// The format for this macro is as follows:
    /// ```
    /// parser! { PARSER_NAME -> OptionsType
    ///     name
    ///     description
    ///     [
    ///         flags,*
    ///     ]
    /// }
    /// ```
    /// where:
    ///  - `PARSER_NAME` is the name of the resulting constant
    ///  - `OptionsType` is the type options type the parser modifies
    ///  - `name` is a literal representing the name of the program
    ///  - `description` is an optional literal describing the use of the program
    ///  - `flags` is the optional list of flags, each one as an expression
    parser::Parser
);

proc_macro_function!(
    /// Creates a help flag
    ///
    /// The format for this macro is as follows:
    /// ```
    /// help_flag!(short_name, long_name noexit)
    /// ```    
    /// where:
    ///  - `short_name` is an optional literal which is the name following the short prefix
    ///  - `long_name` is an optional literal which is the name following the long prefix. The
    ///     preceding comma is treated as part of the literal for identifying if there is a long
    ///     name. In order to specify just a long name use `help_flag!(, long_name)` and to specify
    ///     just a short name use `help_flag!(short_name)`.
    ///  - `noexit` is optional and if present, sets this help flag to not exit after displaying
    ///     the help.
    help_flag::HelpFlag
);
