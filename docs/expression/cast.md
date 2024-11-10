### Cast expression

In GitQL there are two types of casting, Explicit and Implicit casting

#### Implicit Casting

Implicit casting is performed without the need from you to write cast operator or function for example

```sql
SELECT True = 't'
```

In this case the engine perform implicit cast from Text 't' to become boolean 'true' so it's end up with

```sql
SELECT True = True
```

The same is performed when you write Date, Time or DateTime as String and pass it to function that accept Date.

#### Explicit Casting

Implicit casting can handle some cases when the value is const and has specific pattern, but in some cases you want for example
to cast Float to Int or Int to Float after the value is evaluated or provided from real data, in this case you need to explicit ask the engine
to case this value to another type, for example

```SQL
SELECT CAST(commits_count AS Real);
```

Instead of using the above syntax, we can also use the following condensed syntax:

```SQL
SELECT commits_count::Real;
```