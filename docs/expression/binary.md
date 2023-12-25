A binary expression contains two operands separated by one operator

### Arithmetic Expression

Used to perform arithmetic operators on number types.

- `+` Addition.
- `-` Subtraction.
- `*` Multiplication.
- `/` Division.
- `%` Modulus.

---

### Comparison Expression
- `=` used to check if two values are equals.
- `!=` or `<>` used to check if two values are not equals.
- `>` used to check value greater than other value.
- `>=` used to check if value is greater than or equals than other value
- `<` used to check if value is less than than other value.
- `<=` used to check if value is less than or equals than other value.
- `<=>` Returns 1 rather than NULL if both operands are NULL, and 0 rather than NULL if one operand is NULL.
---

### Like Expression
The `LIKE` operator is used for searching for a specified pattern in a string.

```sql
SELECT "Git Query Language" LIKE "G%"
SELECT "Git Query Language" LIKE "%e"
SELECT "Git Query Language" LIKE "%Query%"
SELECT "10 usd" LIKE "[0-9]* usd"
```

---

### Glob Expression
The `GLOB` operator is similar to `LIKE` but uses the Unix file globing syntax for its wildcards. Also, `GLOB` is case sensitive, unlike `LIKE`.

```sql
SELECT "Git Query Language" GLOB "Git*"
```

---

### Logical Expressions

- `||` or `or`: used to calculate logical or between two booleans,
- `&&` or `and`: used to calculate logical and between two booleans,
- `^` or `xor`: used to calculate logical xor between two booleans,

---

### Bitwise Expressions

- `|`: used to calculate bitwise or between two numbers,
- `&`: used to calculate bitwise and between two numbers,
- `<<`: used to calculate bitwise right shift between two numbers,
- `>>`: used to calculate bitwise left shift between two numbers,
 
---

### Between Expression
Used to check if value is between range start and end included

```SQL
SELECT commit_count FROM branches WHERE commit_count BETWEEN 2 .. 30000
```

---

### Is Null Expression
Returns true if value is null, can used with `NOT` keyword to return if true if not null

```SQL
SELECT 1 IS NULL
SELECT 1 IS NOT NULL
```

---

### In Expression
Returns true if any one or more values are equal to the argument

```SQL
SELECT "One" IN ("One", "Two", "Three")
```