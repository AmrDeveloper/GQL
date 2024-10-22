By default the `gitql-std` component contains the standard functions and aggregations functions inspired by
SQLite, MySQL, PostgreSQL, MSSQL ...etc.

This can be more than enough in most cases, but maybe you want to create a special functions related to your data,
or your custom type.

### Creating a custom function

To create a new function you need to provide a name, signature (What are parameters types and return type) and the actual function implementation, the SDK expects a Map with type `HashMap<&'static str, Signature>` to map function name to the signature, and another map of type `&'static HashMap<&'static str, Function>` to map function name to the actual implementation.

By default you can got those two maps with all standard functions like this

```rust linenums="1"
let std_functions = standard_functions().to_owned();
let std_signatures = standard_function_signatures().to_owned();
```

You can remove, replace or insert in those maps, lets take an example of adding new function

Lets start by the function implementation, it should take an Array of Values as arguments, and return Value,
so our function will take two parameters, file path and extension, and return true if this path end with this extension

```rust linenums="1"
fn is_file_has_extension(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    // Get the first argument
    let file_path = values[0].as_text().unwrap();
    // Get the other argument
    let extension = values[1].as_text().unwrap();
    // Check if path end with this extension
    let is_true   = file_path.ends_with(&extension);
    // Return result
    Box::new(BoolValue { value : is_true })
}
```


After implementing our new function let append it to our clone of the standard functions

```rust linenums="1"
// Append the function implementation
let mut std_functions = standard_functions().to_owned();
std_functions.insert("is_file_has_extension", is_file_has_extension);

// Append the function signature
let mut std_signatures = standard_function_signatures().to_owned();
std_signatures.insert(
    "is_file_has_extension",
    Signature {
        // Take two Text values
        parameters: vec![Box::new(TextType), Box::new(TextType)],
        // Return Bool value
        return_type: Box::new(BoolType),
    }
);
```

---

> **_NOTE:_**  You can remove functions, or even create a new empty map with only your functions.

> **_NOTE:_**  The same concepts works with Aggregations functions.

> **_NOTE:_**  Later you will see how to create function with your own types.