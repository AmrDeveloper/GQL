An aggregate function in GQL performs a calculation on multiple values and returns a single value

### Aggregation `max`
Accept field name to calculate the maximum value of it for all elements until the current one

```sql
SELECT name, commit_count, max(commit_count) FROM branches;
```

### Aggregation `min`
Accept field name to calculate the minimum value of it for all elements until the current one

```sql
SELECT name, commit_count, min(commit_count) FROM branches;
```

### Aggregation `sum`
The function sum() is an aggregate function that returns the sum of items in a group

```sql
SELECT name, sum(insertions) FROM diffs GROUP BY name;
```

### Aggregation `avg`
The function avg() is an aggregate function that returns the average values of items in a group

```sql
SELECT name, avg(insertions) FROM commits GROUP BY name;
```

### Aggregation `count`
The function count() is an aggregate function that returns the number of items in a group

```sql
SELECT name, max(name) FROM commits GROUP BY name;
```

### Aggregation `group_concat`
The function group_concat() is an aggregate function that returns a string with concatenated non-NULL value from a group

```sql
SELECT GROUP_CONCAT(name, "-", email) FROM commits GROUP BY name;
```

### Aggregation `bool_and`
The function bool_and() is an aggregate function that returns true if all input values are true, otherwise false

```sql
SELECT bool_and(is_remote) FROM branches;
```

### Aggregation `bool_or`
The function bool_or() is an aggregate function that returns true if at least one input value is true, otherwise false

```sql
SELECT bool_or(is_remote) FROM branches;
```

### Aggregation `bit_and`
The function bit_and() is an aggregate function that returns the bitwise AND of all non-null input values, or null if none

```sql
SELECT bit_and(commits_count) FROM branches;
```