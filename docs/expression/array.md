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

### Slice Expression

Slice expression can be used to return a slice from array from `[start:end`.

```sql
SELECT [1, 2, 3][1:2];
SELECT [[1, 2, 3], [4, 5, 6], [7, 8, 9]][1:2];
```

Slice expression can be used also with 1 as start range to return a slice from array from 1 to the end.

```sql
SELECT [1, 2, 3][:2];
SELECT [[1, 2, 3], [4, 5, 6], [7, 8, 9]][:3];
```

Slice expression can be used also with start only range to return a slice from array from start to length of array.

```sql
SELECT [1, 2, 3][1:];
SELECT [[1, 2, 3], [4, 5, 6], [7, 8, 9]][2:];
```