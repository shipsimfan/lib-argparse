use proc_macro_util::proc_macro_function;

mod action_wrapper;
mod description;
mod flag_name;
mod options_type;

proc_macro_function!(
    /// Creates a constant time argument [`Parser`]
    ///
    /// The format for this macro is as follows:
    /// ```
    /// parser! { PARSER_NAME -> OptionsType
    ///     name
    ///     description,
    ///     terminal
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
    ///  - `terminal` is an optional expression which is the terminal argument of the parser
    ///  - `flags` is the optional list of flags, each one as an expression
    parser::Parser
);

proc_macro_function!(
    /// Creates a help flag
    ///
    /// The format for this macro is as follows:
    /// ```
    /// help_flag!(short_name, long_name)
    /// ```    
    /// where:
    ///  - `short_name` is an optional literal which is the name following the short prefix
    ///  - `long_name` is an optional literal which is the name following the long prefix. The
    ///     preceding comma is treated as part of the literal for identifying if there is a long
    ///     name. In order to specify just a long name use `help_flag!(, long_name)` and to specify
    ///     just a short name use `help_flag!(short_name)`.
    help_flag::HelpFlag
);

proc_macro_function!(
    /// Creates a version flag
    ///
    /// The format for this macro is as follows:
    /// ```
    /// version_flag!(short_name, long_name version)
    /// ```    
    /// where:
    ///  - `short_name` is an optional literal which is the name following the short prefix
    ///  - `long_name` is an optional literal which is the name following the long prefix. The
    ///     preceding comma is treated as part of the literal for identifying if there is a long
    ///     name. In order to specify just a long name use `version_flag!(, long_name)` and to
    ///     specify just a short name use `version_flag!(short_name)`.
    ///  - `version` is an expression whose result must implement [`std::fmt::Display`] and will be
    ///     the text displayed when this flag is matched. The version is not automatically
    ///     included. The environment variable containing the version is `CARGO_PKG_VERSION`.
    version_flag::VersionFlag
);

proc_macro_function!(
    /// Creates a simple flag
    ///
    /// The format for this macro is as follows:
    /// ```
    /// simple_flag!(short_name, long_name count*hint missing description |options: Options, mut parameters|? action)
    /// ```    
    /// where:
    ///  - `short_name` is an optional literal which is the name following the short prefix
    ///  - `long_name` is an optional literal which is the name following the long prefix. The
    ///     preceding comma is treated as part of the literal for identifying if there is a long
    ///     name. In order to specify just a long name use `simple_flag!(, long_name)` and to
    ///     specify just a short name use `simple_flag!(short_name)`.
    ///  - `count` is an optional literal which is the number of parameters to accept. If this is
    ///     included, both `hint` and `missing` are required as well.
    ///  - `hint` is an optional literal which is the hint displayed in the help. If this is
    ///     included, both `count` and `missing` are required as well.
    ///  - `missing` is an optional expression which is the message displayed if the parameters are
    ///     missing. If this is included, both `hint` and `count` are required as well.
    ///  - `description` is a literal which is the description displayed in the help.
    ///  - `options` is an identifier for the developer provided options variable in the action.
    ///  - `Options` is an optional type that specifies the options type when it is ambiguous to
    ///     the compiler. The preceding colon indicates the presence of this field. A `&mut` is not
    ///     nescessary and is added by the macro.
    ///  - `mut` is an optional `mut` keyword to make the parameters mutable.
    ///  - `parameters` is an identifier for the parameters variable in the action.
    ///  - `?` is optional and if present, indicates that the action might return an error,
    ///     otherwise the action return is wrapped in [`Result::Ok`] and the return type is set to
    ///     [`Infallible`].
    ///  - `action` is an expression which is the action itself.
    simple_flag::SimpleFlag
);

proc_macro_function!(
    /// Creates a parsing flag
    ///
    /// The format for this macro is as follows:
    /// ```
    /// parsing_flag!(short_name, long_name hint missing description |options: Options, mut item: Type|? action)
    /// ```    
    /// where:
    ///  - `short_name` is an optional literal which is the name following the short prefix
    ///  - `long_name` is an optional literal which is the name following the long prefix. The
    ///     preceding comma is treated as part of the literal for identifying if there is a long
    ///     name. In order to specify just a long name use `parsing_flag!(, long_name)` and to
    ///     specify just a short name use `parsing_flag!(short_name)`.
    ///  - `hint` is a literal which is the hint displayed in the help.
    ///  - `missing` is an expression which is the message displayed if the parameter is missing.
    ///  - `description` is a literal which is the description displayed in the help.
    ///  - `options` is an identifier for the developer provided options variable in the action.
    ///  - `Options` is an optional type that specifies the options type when it is ambiguous to
    ///     the compiler. The preceding colon indicates the presence of this field. A `&mut` is not
    ///     nescessary and is added by the macro.
    ///  - `mut` is an optional `mut` keyword to make the item mutable.
    ///  - `item` is an identifier for the item variable in the action.
    ///  - `Type` is the type that the parsing flag parses to. It must implement [`std::str::FromStr`]
    ///  - `?` is optional and if present, indicates that the action might return an error,
    ///     otherwise the action return is wrapped in [`Result::Ok`] and the return type is set to
    ///     [`Infallible`].
    ///  - `action` is an expression which is the action itself.
    parsing_flag::ParsingFlag
);

proc_macro_function!(
    /// Creates a list of commands
    ///
    /// The format for this macro is as follows:
    /// ```
    /// commands! [
    ///     commands,*
    /// ]
    /// ```
    /// where:
    ///  - `commands` is the optional list of commands, each one as an expression
    commands::Commands
);

proc_macro_function!(
    /// Creates a command
    ///
    /// The format for this macro is as follows:
    /// ```
    /// command! {
    ///     name
    ///     description
    ///     |options: Options|? action
    ///     {
    ///         parser_name
    ///         terminal
    ///         [
    ///             flags,*
    ///         ]
    ///     }
    /// }
    /// ```
    /// where:
    ///  - `name` is a literal which is the name of the command
    ///  - `description` is a literal which is the description of the command
    ///  - `options` is an identifier for the developer provided options variable in the action
    ///  - `Options` is an optional type that specifies the options type when it is ambiguous to
    ///     the compiler. The preceding colon indicates the presence of this field. A `&mut` is not
    ///     nescessary and is added by the macro.
    ///  - `?` is optional and if present, indicates that the action might return an error,
    ///     otherwise the action return is wrapped in [`Result::Ok`] and the return type is set to
    ///     [`Infallible`].
    ///  - `action` is an expression which is the action itself.
    ///  - `parser_name` is a literal which is the name of the resulting parser for this command
    ///  - `terminal` is an optional expression which is the terminal argument of the resulting parser
    ///  - `flags` is the optional list of flags for the resulting parser, each one as an expression
    command::Command
);

proc_macro_function!(
    /// Creates a list of positional arguments
    ///
    /// The format for this macro is as follows:
    /// ```
    /// positionals! [
    ///     positionals,*
    /// ]
    /// ```
    /// where:
    ///  - `positionals` is the optional list of positionals, each one as an expression
    positionals::Positionals
);

proc_macro_function!(
    /// Creates a simple positional argument
    ///
    /// The format for this macro is as follows:
    /// ```
    /// simple_positional!(name count*hint description |options: Options, index, mut parameters|? action)
    /// ```    
    /// where:
    ///  - `name` is a literal which is the name of this positional argument.
    ///  - `count` is a literal which is the number of parameters to accept.
    ///  - `hint` is a literal which is the hint displayed in the help.
    ///  - `description` is a literal which is the description displayed in the help.
    ///  - `options` is an identifier for the developer provided options variable in the action.
    ///  - `Options` is an optional type that specifies the options type when it is ambiguous to
    ///     the compiler. The preceding colon indicates the presence of this field. A `&mut` is not
    ///     nescessary and is added by the macro.
    ///  - `index` is an identifier for the index of the parameter.
    ///  - `mut` is an optional `mut` keyword to make the parameters mutable.
    ///  - `parameters` is an identifier for the parameters variable in the action.
    ///  - `?` is optional and if present, indicates that the action might return an error,
    ///     otherwise the action return is wrapped in [`Result::Ok`] and the return type is set to
    ///     [`Infallible`].
    ///  - `action` is an expression which is the action itself.
    simple_positional::SimplePositional
);
