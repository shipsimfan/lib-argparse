# ToDo
 1. Implement basic building blocks
   1. Arguments and argument sources
     1. Implement `FromArgument`
       1. String
       2. OsString
       3. PathBuf
       4. Box
       5. Option
       6. Cell, RefCell, maybe OnceCell and LazyCell
       7. Tuples
       8. Bool
       9. Char
       10. Numbers
       11. Array
       12. Vec, VecDeque, LinkedList, HashSet, BTreeSet, BinaryHeap
       13. IpAddr, SocketAddr
       14. Rc, Arc
       15. Atomics
       16. Mutex, RwLock
     2. Add `ArgumentSource` trait
     3. Implement `ArgumentSource`
   2. Errors
   3. Commands and basic parsing
     1. Positionals
     2. Flags
     3. Sub-commands
   4. Version flag
   5. Help flag
 2. Add macros