Now after creating our own `Schema`, `DataProvider` and `Standard function`, most of the cases can be handled 
at this level, but what if you want to store the values in other type than `Int`, `Float`, `Text` ...etc, 
for example you want to create a type called `Song` or `Video` and then you can run a query asking 
what is the longest video in each directory? or what if you want to create a function that take a TreeNode and return number children nodes?

---

To allow this level of customization in type system we need to make it easy to integrate with the parser, type checker 
and execution engine? we can't support every possible type in the engine.

But .... What if we support 0 types 🤔 .... what if the types are not primitives in the engine but defined in the Standard library or the SDK?

---

### Moving the Types from the Engine to the SDK level

This idea is inspired by `Chris Lattner` design in Mojo and Swift programming languages, 
by default the Engine has some built in predefined types like `Integer`, `Float`, `Boolean` ...etc, it know very well 
what operators can work with them, how can cast them, this can work well and programming languages gives you the ability to 
compose types and create Structured data types like `List`, `File`, `GamePlayer` ...etc, but what if we create a way to define your own type, 
and define how the Engine or the Compiler can deal with it, define what operators work with it, in this case we can define `Integer`, `Float`
as part of the Standard library or in our case the SDK, and that also gives the SDK user the power to built any type he want to make it integrated
well in all part of the Query Engine.

Lets for example say we want to create type called `IntPair` and define attributes and operators for it,

### Creating a custom DataType

```rust linenums="1"
use std::any::Any;

use super::base::DataType;

#[derive(Clone)]
pub struct IntPairType;

impl DataType for IntPairType {

    /// Define the literal representation for our new type
    fn literal(&self) -> String {
        "IntPair".to_string()
    }

    /// Define how to compare this type with others
    fn equals(&self, other: &Box<dyn DataType>) -> bool {
        let int_pair_type: Box<dyn DataType> = Box::new(self.clone());
        other.is_any() || other.is_int() || other.is_variant_contains(&int_pair_type)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    /// Allow using the `+` operator between two IntPair's
    fn can_perform_add_op_with(&self) -> Vec<Box<dyn DataType>> {
        vec![Box::new(IntPairType)]
    }

    /// Define that the result of IntPair + IntPair will be another IntPair
    fn add_op_result_type(&self, _other: &Box<dyn DataType>) -> Box<dyn DataType> {
        Box::new(IntPairType)
    }

    /// Define any other operators like -, *, %, >, ...etc
}
```

Now if we create a new Function with Signature that accept IntPair and we pass Int, it will report an error, but now we created a Type but to create a Value with this type we need to create a Custom Value too ([Creating the IntPairValue as Custom value](values.md)).