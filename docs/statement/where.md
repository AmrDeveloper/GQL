The `WHERE` statement is used to filter the data by one or more conditions

For example to select all commits for a specific username

```sql
SELECT * FROM commits where name = "AmrDeveloper"
SELECT * FROM commits WHERE name contains "gmail"
SELECT * FROM branches WHERE is_head = "true"
SELECT * FROM branches WHERE name ends_with "master"
SELECT * FROM branches WHERE name contains "origin"
``` 

You can add Unary and Binary expressions, but you can use Aggregation functions inside the Where statement, because it calculated after the group by statement.