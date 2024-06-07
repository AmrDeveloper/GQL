### Array value Expression

Array expression can be created using the `ARRAY` keyword followed by a list of expression between `[` and `]`.

```sql
SELECT ARRAY[1, 2, 3];
SELECT ARRAY[ARRAY[1, 2, 3], ARRAY[4, 5, 6], ARRAY[7, 8, 9]];
```

Or you can write the list of expressions directly with `[` and `]`

```sql
SELECT [1, 2, 3];
SELECT [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
```