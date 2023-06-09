<h1 align="center">GQL - Git Query Language</h1></br>

<p align="center">
<img src="media/gql_logo.svg" width="20%" height="20%"/>
</p>

<p align="center">
GQL is a query language with a syntax very similar to SQL with a tiny engine to perform queries on .git files instance of database files, the engine executes the query on the fly without the need to create database files or convert .git files into any other format.
</p>

<p align="center">
  <img src="media/gql_demo.gif" alt="animated" width="100%"/>
</p>

---

### Samples

```sql
select * from commits
select name, email from commits
select name, email from commits order by name
select name, email from commits where name contains "gmail" order by name
select * from commits where name.lower = "amrdeveloper"

select * from branches
select * from branches where ishead = "true"
select * from branches where name ends_with "master"
select * from branches where name contains "origin"

select * from tags
select * from tags offset 1 limit 1
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

## Sorting
To sort the result you need to use `order by` keyword followed by field name.

---

### Limit
limit statement take n as integer to limit the result number.

---

### Offset
offset statement take n as integer to ignore the first n result.

---

### Transformations
Transofmations are functions with 0 arguments used to apply transformation on values

- `lower` convert the value to be lower case.
- `upper` convert the value to be upper case.
- `trim` remote leading and trailing whitespace.
- `length` return the length as a string.

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
