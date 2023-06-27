An transformation function in GQL performs a transformation on single value and returns a single value

### String lower
Transform String value to lower case

```sql
SELECT * FROM commits where name.lower = "amrdeveloper"
```

### String upper
Transform String value to upper case

```sql
SELECT * FROM commits where name.upper = "AMRDEVELOPER"
```

### String trim
Transform String value to String with removes whitespace from the start and eng of it

```sql
SELECT * FROM commits where name.trim = ""
```

### String length
Transform String value to the length of it

```sql
SELECT * FROM commits where name.length > 0
```
