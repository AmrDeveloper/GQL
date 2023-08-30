The unary expression is an expression the prefixed with operators

### Prefix Unary Expression
- `!` takes truth to falsity and vice versa. It is typically used with boolean

```sql
SELECT * FROM branches WHERE !is_remote
SELECT * FROM branches WHERE !is_head
```

- `-` negates the value of the operand.

```sql
SELECT * FROM branches WHERE commit_count > -1
```