<h1 align="center">GQL - Git Query Language</h1></br>

<p align="center">
<img src="assets/gql_logo.svg" width="20%" height="20%"/>
</p>

<p align="center">
  <img alt="Crates.io" src="https://img.shields.io/crates/v/gitql?style=flat-square">
  <img alt="Release" src="https://github.com/AmrDeveloper/GQL/actions/workflows/release.yaml/badge.svg">
  <img alt="Docs" src="https://github.com/AmrDeveloper/GQL/actions/workflows/docs.yaml/badge.svg">
  <img alt="GitHub release" src="https://img.shields.io/github/v/release/amrdeveloper/gql">
  <img alt="GitHub issues" src="https://img.shields.io/github/issues/amrdeveloper/gql">
  <img alt="GitHub" src="https://img.shields.io/github/license/amrdeveloper/gql">
  <img alt="GitHub all releases" src="https://img.shields.io/github/downloads/amrdeveloper/gql/total">
</p>

GQL is a query language with a syntax very similar to SQL with a tiny engine to perform queries on .git files instance of database files, the engine executes the query on the fly without the need to create database files or convert .git files into any other format, note that all Keywords in GQL are case-insensitive similar to SQL.

### Samples

``` sql
SELECT 1
SELECT 1 + 2
SELECT LEN("Git Query Language")
SELECT "One" IN ("One", "Two", "Three")
SELECT "Git Query Language" LIKE "%Query%"

SET @arr = [1, 2, 3];
SELECT [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
SELECT @arr[1], @arr[2], @arr[3], ARRAY_LENGTH(@arr);
SELECT @arr[1:2], @arr[2:], @arr[:2];

SELECT DISTINCT title AS tt FROM commits
SELECT name, COUNT(name) AS commit_num FROM commits GROUP BY name, author_email ORDER BY commit_num DESC LIMIT 10
SELECT commit_count FROM branches WHERE commit_count BETWEEN 0 .. 10

SELECT * FROM refs WHERE type = "branch"
SELECT * FROM refs ORDER BY type

SELECT * FROM commits
SELECT author_name, author_email FROM commits
SELECT author_name, author_email FROM commits ORDER BY name DESC, author_email ASC
SELECT author_name, author_email FROM commits WHERE name LIKE "%gmail%" ORDER BY name
SELECT * FROM commits WHERE LOWER(name) = "amrdeveloper"
SELECT author_name FROM commits GROUP By name
SELECT author_name FROM commits GROUP By name having name = "AmrDeveloper"

SELECT * FROM branches
SELECT * FROM branches WHERE is_head = true
SELECT name, LEN(name) FROM branches

SELECT * FROM tags
SELECT * FROM tags OFFSET 1 LIMIT 1
```