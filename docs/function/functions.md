An function in GitQL accept one or more value and return value,
note that all functions names are case-insensitive.

### String functions
| Name      | Paramter     | Return | Description                                           |
| --------- | ------------ | ------ | ----------------------------------------------------- |
| LOWER     | Text         | Text   | Return Text in lower case                             |
| UPPER     | Text         | Text   | Return Text in upper case                             |
| REVERSE   | Text         | Text   | Return a reversed string                              |
| TRIM      | Text         | Text   | Return string without start or end whitespaces        |
| LEN       | Text         | Number | Return the length of this string                      |
| REPLICATE | Text, Number | Number | Return repeated a string a specified number of times. |

### String functions samples

```sql
SELECT * FROM commits where name.lower = "amrdeveloper"
SELECT * FROM commits where name.upper = "AMRDEVELOPER"
SELECT * FROM commits where name.reverse = "repolevedrma"
SELECT * FROM commits where name.trim = ""
SELECT * FROM commits where name.len > 0
```