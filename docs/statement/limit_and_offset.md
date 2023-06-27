The `LIMIT` statement used to limit the number of end result

```sql
SELECT * FROM commits LIMIT 10
SELECT * FROM branches LIMIT 15
```

The `OFFSET` statement specifies how many rows to skip at the beginning of the result set

```sql
SELECT * FROM commits OFFSET 10
SELECT * FROM branches OFFSET 15
```

You can mix the offset and limit statements

```sql
SELECT * FROM commits OFFSET 10 LIMIT 10
SELECT * FROM branches OFFSET 15 LIMIT 15
```