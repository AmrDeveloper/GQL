<h1 align="center">GQL - Git Query Language</h1></br>

<p align="center">
<img src="media/gql_logo.svg" width="20%" height="20%"/>
</p>

<p align="center">
  <img alt="GitHub release" src="https://img.shields.io/github/v/release/amrdeveloper/gql">
  <img alt="GitHub issues" src="https://img.shields.io/github/issues/amrdeveloper/gql">
  <img alt="GitHub" src="https://img.shields.io/github/license/amrdeveloper/gql">
  <img alt="GitHub all releases" src="https://img.shields.io/github/downloads/amrdeveloper/gql/total">
</p>

<p align="center">
GQL is a query language with a syntax very similar to SQL with a tiny engine to perform queries on .git files instance of database files, the engine executes the query on the fly without the need to create database files or convert .git files into any other format, note that all Keywords in GQL are case-insensitive similar to SQL.
</p>

<p align="center">
  <img src="media/gql_demo.gif" alt="animated" width="100%"/>
</p>

---

### Samples

```sql
SELECT * FROM refs WHERE type = "branch"
SELECT * FROM refs WHERE ORDER BY type

SELECT * FROM commits
SELECT name, email FROM commits
SELECT name, email FROM commits ORDER BY name DES
SELECT name, email FROM commits WHERE name contains "gmail" ORDER BY name
SELECT * FROM commits WHERE name.lower = "amrdeveloper"

SELECT * FROM branches
SELECT * FROM branches WHERE ishead = "true"
SELECT * FROM branches WHERE name ends_with "master"
SELECT * FROM branches WHERE name contains "origin"

SELECT * FROM tags
SELECT * FROM tags OFFSET 1 LIMIT 1
```

### Build and run
To build and run GQL you need to have rust installed on your system and then use run command with folder that contains .git files

```
cargo run <repository_path>
```

---
## Documentation:

### Select from
Select keyword used to select all of some fields from specific table

---

### Tables and Fields
- refs { name, full_name, type }
- commits { name, email, title, message, time }
- branches { name, ishead, isremote }
- tags { name }

---

### String Comparisons
- `=` used to check if field equal to expected value.
- `!` used to check if field not equal to expected value.
- `>` used to check if field greater than expected value.
- `>=` used to check if field greater than or equals expected value.
- `<` used to check if field less than expected value.
- `<=` used to check if field less than or equals expected value.

---

### String checks
- `contains` used to check that field contains value.
- `starts_with` used to check that field starts with value.
- `ends_with` used to check that field ends with value.
- `matches` used to check that field matches regex format.

---

## Logical Expressions

- `|` or `or`: used to calculate or between two booleans,
- `&` or `and`: used to calculate and between two booleans,
- `^` or `xor`: used to calculate xor between two booleans,

---

## Unary Expressions

- `!`: used as prefix for expression to perform bang expression

---

## Group Expressions

Group expression is an expresion inside `(` and `)` used to give high precedence for expression.

---

## Boolean Expressions

- `true`, `false`

---

## Sorting
To sort the result you need to use `order by` keyword followed by field name,
by default it will be in Ascending order, if you want Descending you need to add `DES` like SQL after field name

---

### Limit
limit statement take n as integer to limit the result number.

---

### Offset
offset statement take n as integer to ignore the first n result.

---

### Transformations
Transformations are functions with 0 arguments used to apply transformation on values

- `lower` convert the value to be lower case.
- `upper` convert the value to be upper case.
- `trim` remote leading and trailing whitespace.
- `length` return the length as a string.

---

### Name Alias

You can rename a column temporarily by giving another name, which is known as ALIAS,
Renaming is a temporary change and the actual column name does not change

```sql
SELECT name as branch_name from branches
```

---

### License
```
MIT License

Copyright (c) 2023 Amr Hesham

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
