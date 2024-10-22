The GitQL parser and engine should be aware of what kind of data it should deal with, so it can provide
a clean error messages, and apply the right operators, this information defined in a structure way in place called
the `Schema`, and this schema contains

- What tables you have.
- What are the columns in each tables and what are their types.

```rust
pub struct Schema {
    pub tables_fields_names: HashMap<&'static str, Vec<&'static str>>,
    pub tables_fields_types: HashMap<&'static str, Box<dyn DataType>>,
}
```

So for your custom purpose you need to define your own schema, let take an example of a simple file system,
so you have a table called `files`, and this table has two columns, `file_name` as Text (aka String), and `is_directory` as Boolean.

### Define the columns types

```rust linenums="1"
pub fn tables_fields_types() -> HashMap<&'static str, Box<dyn DataType>> {
    let mut map: HashMap<&'static str, Box<dyn DataType>> = HashMap::new();
    map.insert("file_name", Box::new(TextType));
    map.insert("is_directory", Box::new(BoolType));
    map
}
```

### Define the table name and his columns

```rust linenums="1"
pub fn tables_fields_names() -> &'static HashMap<&'static str, Vec<&'static str>> {
    static HASHMAP: OnceLock<HashMap<&'static str, Vec<&'static str>>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert("files", vec!["file_name", "is_directory"]);
        map
    );
}
```

### Create a schema object with those information

```rust linenums="1"
let schema = Schema {
    tables_fields_names: tables_fields_names().to_owned(),
    tables_fields_types: tables_fields_types().to_owned(),
};
```

Later this schema instance with the standard library will used to create the environment

```rust linenums="1"
let mut env = Environment::new(schema);
```