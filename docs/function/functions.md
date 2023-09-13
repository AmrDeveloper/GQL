An function in GitQL accept one or more value and return value,
note that all functions names are case-insensitive.

### String functions

| Name       | Paramter     | Return | Description                                                         |
| ---------- | ------------ | ------ | ------------------------------------------------------------------- |
| LOWER      | Text         | Text   | Return Text in lower case.                                          |
| UPPER      | Text         | Text   | Return Text in upper case.                                          |
| REVERSE    | Text         | Text   | Return a reversed string.                                           |
| TRIM       | Text         | Text   | Removes leading and trailing spaces from a string.                  |
| LTRIM      | Text         | Text   | Removes leading spaces from a string.                               |
| RTRIM      | Text         | Text   | Removes trailing spaces from a string.                              |
| LEN        | Text         | Number | Return the length of this string.                                   |
| REPLICATE  | Text, Number | Text   | Return repeated a string a specified number of times.               |
| SPACE      | Number       | Text   | Returns a string of the specified number of space characters.       |
| ASCII      | Text         | Number | Returns the ASCII value for the specific character.                 |
| LEFT       | Text, Number | Text   | Extracts a number of characters from a string (starting from left). |
| DATALENGTH | Text         | Number | Returns the number of bytes used to represent an expression.        |
| CHAR       | Number       | Text   | Returns the character based on the ASCII code.                      |
| REPLACE    | Text, Text, Text | Text  | Replaces all occurrences of a substring within a string, with a new substring. |
| SUBSTRING  | Text, Number, Number | Text | Extracts some characters from a string.                       |
| STUFF      | Text, Number, Number, Text | Deletes a part of a string and then inserts another part into the string, starting at a specified position. |
| RIGHT      | Text, Number | Text   | Extracts a number of characters from a string (starting from right).|
### String functions samples

```sql
SELECT * FROM commits where LOWER(name) = "amrdeveloper"
SELECT * FROM commits where UPPER(name) = "AMRDEVELOPER"
SELECT * FROM commits where REVERSE(name) = "repolevedrma"
SELECT * FROM commits where TRIM(name) = ""
SELECT * FROM commits where LEN(name) > 0
SELECT * FROM commits where name = SPACE(5)
SELECT name, ASCII(name) AS firstCharAscii FROM commits
SELECT LEFT("AmrDeveloper", 3) AS extract
SELECT DATALENGTH("AmrDeveloper") as bytelength
SELECT CHAR(345) AS code
SELECT REPLACE("ABC ABC ABC", "a", "c") as replacedText
SELECT name, SUBSTRING(name, 1, 5) AS extract FROM commits
SELECT STUFF("GQL tutorial!", 13, 1, " is fun!")
SELECT RIGHT("AmrDeveloper", 3) AS extract
```

### Date functions

| Name              | Paramter | Return   | Description                                               |
| ----------------- | -------- | -------- | --------------------------------------------------------- |
| CURRENT_TIME      |          | Time     | Return current time in `HH-MM-SS` format.                 |
| CURRENT_DATE      |          | Date     | Return current date in `YYYY-MM-DD` format.               |
| CURRENT_TIMESTAMP |          | DateTime | Return current date time in `YYYY-MM-DD HH-MM-SS` format. |


### Date functions samples

```sql
SELECT CURRENT_TIME()
SELECT CURRENT_DATE()
SELECT CURRENT_TIMESTAMP()
```