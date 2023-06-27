The `GROUP BY` statement groups rows that have the same values into summary rows, like "find the number of commits for each username or email".

```SQL
SELECT * FROM commits GROUP BY name
```