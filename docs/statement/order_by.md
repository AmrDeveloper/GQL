The `ORDER BY` Statement used to order the result-set in ascending or descending order by one or more argument.

```sql
SELECT name, email FROM commits ORDER BY name
SELECT name, email FROM commits ORDER BY name, email
SELECT name, email FROM commits ORDER BY email, commit_id ASC
SELECT name, email FROM commits ORDER BY name DESC
SELECT name, email FROM commits ORDER BY name, LEN(name)
SELECT name, email FROM commits ORDER BY (cASE WHEN (email contains "gmail") THEN 1 ELSE 0 END) DESC
```