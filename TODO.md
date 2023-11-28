# ToDo List
 1. Add flag arguments
     1. Add flag macros
         1. Add simple flag macro
         2. Add parsing flag macro
 2. Add terminal arguments
     1. Add terminal argument trait
     2. Add terminal argument to help generation
     3. Add terminal argument parsing
     4. Update help generator for terminal arguments
     5. Add terminal arguments to parser macro
     6. Add commands
         1. Add command struct
         2. Add commands struct (List of command that implements terminal argument)
     7. Add commands macro
     8. Add positionals
         1. Add positional trait
         2. Add simple positional (one or more arguments -> action)
         3. Add parsing positional (one argument -> FromStr/From<String>/From<&str>/From<OsString>/From<&OsStr> -> action | error_map)
     9. Add positional macros
         1. Add simple positional macro
         2. Add parsing positional macro