GitQL has support for global variables with syntax inspired by MySQL

### Declare variable with value

```sql
SET @one = 1
SET @STRING = "GitQL"
```

### Use the variable
You can use the variable like any other symbol using the name

```sql
SELECT @one
```