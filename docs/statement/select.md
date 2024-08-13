### Select Statement

The `SELECT` statement is used to query data from a single table

For example to select all fields from commits table.

```sql
SELECT * FROM commits
```

Or Selecting just title and message

```sql
SELECT title message FROM commits
```

You can use Aggregation function in the select statement to perform function on all data until the current one

```sql
SELECT count(author_name) FROM commits
```

You can alias the column name only in this query by using `as` keyword for example

```sql
SELECT title as tt FROM commits
SELECT name, commit_count, max(commit_count) AS max_count message FROM branches
```

---

### Distinct option

You can select unique rows only using the `distinct` keyword for example,

```sql
SELECT DISTINCT title AS tt FROM commits
```

---

### Distinct On option

You can select rows with unique fields using the `distinct on` keyword with one or more field for example,

```sql
SELECT DISTINCT ON (author_name) title AS tt FROM commits
```

### Joins

You can perform one or more JOIN to join two tables together, you can use one of four different join types,
which are Inner, Cross, Left and Right outer JOINS and also filter by on predicate condition.

```sql
SELECT COUNT() FROM tags JOIN branches
SELECT COUNT() FROM tags LEFT JOIN branches ON commit_count > 1
SELECT COUNT() FROM tags RIGHT JOIN branches ON commit_count > 1
```

### Select ... INTO

You can export the query result into external file using the syntax `INTO OUTFILE <File> <options>`

```sql
SELECT name FROM branches INTO OUTFILE "branches.txt"
```

You can format the output result with options for example

```sql
SELECT * FROM branches INT OUTFILE "branches" FIELDS TERMINATED "," LINES TERMINATED "\n" ENCLOSED "|"
```

If you want to just dump the data without any format you can use `INTO DUMPFILE`

```sql
SELECT * FROM branches INTO DUMPFILE "braches.txt"
```
