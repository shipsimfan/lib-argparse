# ToDo List
 1. Add tests (42/90%)
     1. Unit tests
     2. Integration tests
 2. Add terminal arguments
     1. Add commands
         1. Add command struct
         2. Add commands struct (List of command that implements terminal argument)
     2. Add commands macro
     3. Add positionals
         1. Add positional trait
         2. Add simple positional (one or more arguments -> action)
         3. Add parsing positional (one argument -> FromStr/From<String>/From<&str>/From<OsString>/From<&OsStr> -> action | error_map)
     4. Add positional macros
         1. Add simple positional macro
         2. Add parsing positional macro