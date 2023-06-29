<h1 align="center">GQL - Git Query Language</h1></br>

<p align="center">
<img src="media/gql_logo.svg" width="20%" height="20%"/>
</p>

<p align="center">
  <img alt="Release" src="https://github.com/AmrDeveloper/GQL/actions/workflows/release.yaml/badge.svg">
  <img alt="Docs" src="https://github.com/AmrDeveloper/GQL/actions/workflows/docs.yaml/badge.svg">
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
SELECT name, count(name) AS commit_num FROM commits GROUP BY name ORDER BY commit_num DES LIMIT 10
SELECT commit_count FROM branches WHERE commit_count BETWEEN 0 .. 10

SELECT * FROM refs WHERE type = "branch"
SELECT * FROM refs WHERE ORDER BY type

SELECT * FROM commits
SELECT name, email FROM commits
SELECT name, email FROM commits ORDER BY name DES
SELECT name, email FROM commits WHERE name contains "gmail" ORDER BY name
SELECT * FROM commits WHERE name.lower = "amrdeveloper"
SELECT name FROM commits GROUP By name
SELECT name FROM commits GROUP By name having name = "AmrDeveloper"

SELECT * FROM branches
SELECT * FROM branches WHERE ishead = "true"
SELECT * FROM branches WHERE name ends_with "master"
SELECT * FROM branches WHERE name contains "origin"

SELECT * FROM tags
SELECT * FROM tags OFFSET 1 LIMIT 1
```

---

## Documentation:

  - [Full Documentation](https://amrdeveloper.github.io/GQL/)
  - [Install or Build](docs/setup.md)
  - [Tables](docs/structure/tables.md)
  - [Types](docs/structure/types.md)
  - [Statements](docs/statement)
  - [Expressions](docs/expression)
  - [Transformations](docs/function/transformations.md)
  - [Aggregations](docs/function/aggregations.md)

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
