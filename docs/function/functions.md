An function in GitQL accept one or more value and return value,
note that all functions names are case-insensitive.

### String functions
| Name      | Paramter     | Return | Description                                                   |
| --------- | ------------ | ------ | ------------------------------------------------------------- |
| LOWER     | Text         | Text   | Return Text in lower case                                     |
| UPPER     | Text         | Text   | Return Text in upper case                                     |
| REVERSE   | Text         | Text   | Return a reversed string                                      |
| TRIM      | Text         | Text   | Return string without start or end whitespaces                |
| LEN       | Text         | Number | Return the length of this string                              |
| REPLICATE | Text, Number | Text   | Return repeated a string a specified number of times.         |
| SPACE     | Number       | Text   | Returns a string of the specified number of space characters. |

### String functions samples

```sql
SELECT * FROM commits where LOWER(name) = "amrdeveloper"
SELECT * FROM commits where UPPER(name) = "AMRDEVELOPER"
SELECT * FROM commits where REVERSE(name) = "repolevedrma"
SELECT * FROM commits where TRIM(name) = ""
SELECT * FROM commits where LEN(name) > 0
SELECT * FROM commits where name = SPACE(5)
```