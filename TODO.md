# ToDo List
 1. Add adjustable usage ("%f" for flags, "%t" for terminal, "%[0-9]+" for command list)
 2. Add flag arguments
     1. Add adjustable flags header
     2. Add help flag (with no-exit)
         1. Add help generator
     3. Add version flag (with no-exit)
     4. Add flag macros
         1. Add help flag macro
         2. Add version flag macro
         3. Add simple flag macro
         4. Add parsing flag macro
 3. Add terminal arguments
     1. Add terminal argument trait
     2. Add terminal argument parsing
     3. Update help generator for terminal arguments
     4. Add terminal arguments to parser macro
     5. Add commands
         1. Add command struct
         2. Add commands struct (List of command that implements terminal argument)
     6. Add commands macro
     7. Add positionals
         1. Add positional trait
         2. Add simple positional (one or more arguments -> action)
         3. Add parsing positional (one argument -> FromStr/From<String>/From<&str>/From<OsString>/From<&OsStr> -> action | error_map)
     8. Add positional macros
         1. Add simple positional macro
         2. Add parsing positional macro