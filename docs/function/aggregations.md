An aggregate function in GQL performs a calculation on multiple values and returns a single value

### Aggregation `max`
Accept field name to calculate the maximum value of it for all elements until the current one

```sql
SELECT name, commit_count, max(commit_count) FROM branches
```

### Aggregation `min`
Accept field name to calculate the minimum value of it for all elements until the current one

```sql
SELECT name, commit_count, min(commit_count) FROM branches
```

### Aggregation `sum`
The function sum() is an aggregate function that returns the sum of items in a group

```sql
SELECT name, sum(insertions) FROM diffs GROUP BY name
```

### Aggregation `avg`
The function avg() is an aggregate function that returns the average values of items in a group

```sql
SELECT name, avg(insertions) FROM commits GROUP BY name
```

### Aggregation `count`
The function count() is an aggregate function that returns the number of items in a group

```sql
SELECT name, max(name) FROM commits GROUP BY name
```