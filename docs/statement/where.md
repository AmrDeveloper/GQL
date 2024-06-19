The `WHERE` statement is used to filter the data by one or more conditions

For example to select all commits for a specific username

```sql
SELECT * FROM commits where author_name = "AmrDeveloper"
SELECT * FROM branches WHERE is_head = "true"
``` 

You can add Unary and Binary expressions, but you can use Aggregation functions inside the Where statement, because it calculated after the group by statement.