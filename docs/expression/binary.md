A binary expression contains two operands separated by one operator

### Arithmetic Expression

Used to perform arithmetic operators on number types.

- `+` Addition.
- `-` Subtraction.
- `*` Multiplication.
- `/` Division.
- `%` Modulus.

### Comparison Expression
- `=` used to check if field equal to expected value.
- `!` used to check if field not equal to expected value.
- `>` used to check if field greater than expected value.
- `>=` used to check if field greater than or equals expected value.
- `<` used to check if field less than expected value.
- `<=` used to check if field less than or equals expected value.

---

### String checks Expression
- `contains` used to check that field contains value.
- `starts_with` used to check that field starts with value.
- `ends_with` used to check that field ends with value.
- `matches` used to check that field matches regex format.

---

### Logical Expressions

- `|` or `or`: used to calculate or between two booleans,
- `&` or `and`: used to calculate and between two booleans,
- `^` or `xor`: used to calculate xor between two booleans,

---

### Between Expression
Used to check if value is between range start and end included

```SQL
SELECT commit_count FROM branches WHERE commit_count BETWEEN 2 .. 30000
```