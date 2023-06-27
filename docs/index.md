<h1 align="center">GQL - Git Query Language</h1></br>

<p align="center">
<img src="assets/gql_logo.svg" width="20%" height="20%"/>
</p>

<p align="center">
  <img alt="GitHub release" src="https://img.shields.io/github/v/release/amrdeveloper/gql">
  <img alt="GitHub issues" src="https://img.shields.io/github/issues/amrdeveloper/gql">
  <img alt="GitHub" src="https://img.shields.io/github/license/amrdeveloper/gql">
  <img alt="GitHub all releases" src="https://img.shields.io/github/downloads/amrdeveloper/gql/total">
</p>

GQL is a query language with a syntax very similar to SQL with a tiny engine to perform queries on .git files instance of database files, the engine executes the query on the fly without the need to create database files or convert .git files into any other format, note that all Keywords in GQL are case-insensitive similar to SQL.

### Samples

``` sql
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