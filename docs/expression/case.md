Case expression is similar to Switch Expression in many languages, it's return the value of the first branch that has condition evaluated to true, if not branch found it will return the default value

```sql
SELECT name FROM branches WHERE (CASE WHERE isRemote THEN 1 ELSE 0 END) > 0
```