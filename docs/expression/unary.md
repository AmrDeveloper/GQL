The unary expression is an expression the prefixed with operators

### Not Expression
The logical NOT ( ! ) operator takes truth to falsity and vice versa. It is typically used with boolean

```sql
SELECT * FROM branches WHERE !is_remote
SELECT * FROM branches WHERE !is_head
```