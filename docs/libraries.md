Git query language now has different architecture as a set of crates, 
the goal is to able to use part of it in your program and allow you to extend the features

### Libraries
- gitql-cli: Contains the command line interface components.
- gitql-ast: Contains the abstract syntax tree nodes.
- gitql-parser: Contains the parser code.
- gitql-engine: Contains the execution engine code.