The `Do` works in a similar way to the SELECT statement, but without returning a result set

For example to select all fields from commits table.

```sql
DO SLEEP(10);
```

Where DO Cannot be Used

We can’t use DO everywhere that we can use SELECT. For example we can’t do the following:

```sql
DO * FROM branches;
```