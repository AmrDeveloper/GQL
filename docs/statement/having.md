The `HAVING` statement is very similar to `WHERE` expect that it evaluated after the `GROUP BY` statement

```sql
SELECT * FROM commits GROUP BY author_name HAVING author_name = "AmrDeveloper"
SELECT * FROM branches GROUP BY name HAVING is_head = "true"
``` 