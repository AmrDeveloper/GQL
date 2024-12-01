The GitQL Query engine is designed to work as a set of decoupled libraries which gives you
the flexibility to extend or replace any part of the query execution journey.

## GitQL Engine Architecture

``` mermaid
graph LR
  A[SQL Query] --> B[Tokenizer];
  B --> C[Parser and Type Checker];
  C --> D[Execution Engine];
  D --> E[Output]
  F[Schema] --> C;
  G[Data Provider] --> D;
  H[Standard library] --> D;
```

## GitQL SDK Components

| Component    |                  Description                  |                      Install |
| ------------ | :-------------------------------------------: | ---------------------------: |
| gitql-core   |                Core components                |   `cargo install gitql-core` |
| gitql-std    |          Standard library functions           |    `cargo install gitql-std` |
| gitql-cli    | CLI components like args parser, cli reporter |    `cargo install gitql-cli` |
| gitql-ast    |   structures components such as AST, Types    |    `cargo install gitql-ast` |
| gitql-parser |      Parser and Type checker components       | `cargo install gitql-parser` |
| gitql-engine |          Execution engine component           | `cargo install gitql-engine` |

---

## Using the GitQL SDk to extend the components

As you will see building your own query language for specific need using the GitQL gives you the ability to customize every part of the engine such as operators, types, schema, functions ...etc.

- [Customize the Data Schema](schema.md).
- [Customize the Data Provider](provider.md).
- [Customize the Standard library](functions.md).
- [Customize the Type system](types.md).
- [Customize the Value system](values.md).

---

## Example of product that build on top of GitQL

- [ClangQL](https://github.com/AmrDeveloper/ClangQL):
To run SQL query on C/C++ Code.

- [FileQL](https://github.com/AmrDeveloper/FileQL):
To run SQL query on the file system.

- [PyQL](https://github.com/AmrDeveloper/PyQL):
To run SQL query on Python Code.

Feel free to add your product too, everyone is welcome to join.
