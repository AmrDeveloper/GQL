The `ORDER BY` keyword is used to sort the result-set in ascending or descending order.

```sql
SELECT name, email FROM commits ORDER BY name
SELECT name, email FROM commits ORDER BY name ASC
SELECT name, email FROM commits ORDER BY name DESC
SELECT name, email FROM commits ORDER BY LEN(name)
SELECT name, email FROM commits ORDER BY (cASE WHEN (email contains "gmail") THEN 1 ELSE 0 END) DESC
```