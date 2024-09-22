The `ORDER BY` Statement used to order the result-set in ascending or descending order by one or more arguments.

```sql
SELECT author_name, author_email FROM commits ORDER BY author_name
SELECT author_name, author_email FROM commits ORDER BY author_name, author_email
SELECT author_name, author_email FROM commits ORDER BY author_email, commit_id ASC
SELECT author_name, author_email FROM commits ORDER BY author_name DESC
SELECT author_name, author_email FROM commits ORDER BY author_name, LEN(author_name)
```

The `ORDER BY` Statement with `USING <operator>` syntax inspired by PostgreSQL

```sql
SELECT author_name, author_email FROM commits ORDER BY author_email, commit_id USING <
SELECT author_name, author_email FROM commits ORDER BY author_name USING >
```