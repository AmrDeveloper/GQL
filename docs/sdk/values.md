Now we created a new Type called `IntPairType` that can be used as Function parameter, return type or column type, but we need a custom value that can represent this type, it's almost the same concept, so lets start creating the `IntPairValue`.

```rust linenums="1"
use gitql_ast::types::base::DataType;

use super::base::Value;

#[derive(Clone)]
pub struct IntPairValue {
    pub first: i64,
    pub second: i64,
}

impl Value for IntPairValue {

    /// Define the literal representation for our new Value
    fn literal(&self) -> String {
        format!("({}, {})", self.first, self.second)
    }

    /// Define how to check equality between this value and other
    fn equals(&self, other: &Box<dyn Value>) -> bool {
        if let Some(other_int_pair) = other.as_any().downcast_ref::<IntPairValue>() {
            return self.first == other_int_pair.first 
                && self.second == other_int_pair.second;
        }
        false
    }

    /// You can define how to order between IntPair values or None to disable ordering
    fn compare(&self, other: &Box<dyn Value>) -> Option<Ordering> {
        None
    }

    fn data_type(&self) -> Box<dyn DataType> {
        Box::new(IntPairType)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    /// As we allowed `+` between IntPair types in `can_perform_add_op_with` 
    /// We need also to define how this operator will work
    fn perform_add_op(&self, other: &Box<dyn Value>) -> Result<Box<dyn Value>, String> {
        if let Some(other_int) = other.as_any().downcast_ref::<IntPairValue>() {
            let first = self.first + other_int.first;
            let second = self.second + other_int.second;
            return Ok(Box::new(IntPairValue { first, second }));
        }

        /// Write your exception message
        Err("Unexpected type to perform `+` with".to_string())
    }
}
```

---

### Creating Function to construct IntPairValue

```rust linenums="1"
fn new_int_pair(values: &[Box<dyn Value>]) -> Box<dyn Value> {
    let first = values[0].as_int().unwrap();
    let second = values[1].as_int().unwrap();
    Ok(Box::new(IntPairValue { first, second }))
}
```

### Register this function signature and implementation

```rust linenums="1"
// Append the function implementation
let mut std_functions = standard_functions().to_owned();
std_functions.insert("new_int_pair", new_int_pair);

// Append the function signature
let mut std_signatures = standard_function_signatures().to_owned();
std_signatures.insert(
    "new_int_pair",
    Signature {
        // Take two Integers values
        parameters: vec![Box::new(IntType), Box::new(IntType)],
        // Return IntPair Value
        return_type: Box::new(IntPairValue),
    }
);
```

After connecting everything together in the next step, you can perform query like this

```sql
SELECT new_int_pair(1, 2) + new_int_pair(3, 4);
```

And got result like `(4, 6)`.

### Going forward

This is just a quick example of how to create your own types, but you can create any type you want, even Data structures like Map and allow
index operator for it so you can write 

```sql
SELECT map["key"]
```