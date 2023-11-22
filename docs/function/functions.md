An function in GitQL accept one or more value and return value,
note that all functions names are case-insensitive.

### String functions

| Name       | Paramters                    | Return  | Description                                                                                                                                                          |
| ---------- | ---------------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| LOWER      | Text                         | Text    | Return Text in lower case.                                                                                                                                           |
| UPPER      | Text                         | Text    | Return Text in upper case.                                                                                                                                           |
| REVERSE    | Text                         | Text    | Return a reversed string.                                                                                                                                            |
| TRIM       | Text                         | Text    | Removes leading and trailing spaces from a string.                                                                                                                   |
| LTRIM      | Text                         | Text    | Removes leading spaces from a string.                                                                                                                                |
| RTRIM      | Text                         | Text    | Removes trailing spaces from a string.                                                                                                                               |
| LEN        | Text                         | Integer | Return the length of this string.                                                                                                                                    |
| REPLICATE  | Text, Integer                | Text    | Return repeated a string a specified number of times.                                                                                                                |
| SPACE      | Integer                      | Text    | Returns a string of the specified number of space characters.                                                                                                        |
| ASCII      | Text                         | Integer | Returns the ASCII value for the specific character.                                                                                                                  |
| LEFT       | Text, Integer                | Text    | Extracts a number of characters from a string (starting from left).                                                                                                  |
| DATALENGTH | Text                         | Integer | Returns the number of bytes used to represent an expression.                                                                                                         |
| CHAR       | Integer                      | Text    | Returns the character based on the ASCII code.                                                                                                                       |
| NCHAR      | Integer                      | Text    | Returns the character based on the ASCII code.                                                                                                                       |
| REPLACE    | Text, Text, Text             | Text    | Replaces all occurrences of a substring within a string, with a new substring.                                                                                       |
| SUBSTRING  | Text, Integer, Integer       | Text    | Extracts some characters from a string.                                                                                                                              |
| STUFF      | Text, Integer, Integer, Text | Text    | Deletes a part of a string and then inserts another part into the string, starting at a specified position.                                                          |
| RIGHT      | Text, Integer                | Text    | Extracts a number of characters from a string (starting from right).                                                                                                 |
| TRANSLATE  | Text, Text, Text,            | Text    | Returns the string from the first argument after the characters specified in the second argument are translated into the characters specified in the third argument. |
| SOUNDEX    | Text                         | Text    | Returns a four-character code to evaluate the similarity of two expressions.                                                                                         |
| CONCAT     | Text, Text                   | Text    | Adds two or more strings together.                                                                                                                                   |
| UNICODE    | Text                         | Integer | Return an integer value (the Unicode value), for the first character of the input expression.                                                                        |

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
SELECT TRANSLATE("Amr[Dev]{eloper}", "[]{}", "()()")
SELECT SOUNDEX("AmrDeveloper") as code
SELECT CONCAT("amrdeveloper", ".github.io")
SELECT UNICODE("AmrDeveloper")
```

### Date functions

| Name              | Paramters        | Return   | Description                                                     |
| ----------------- | ---------------- | -------- | --------------------------------------------------------------- |
| CURRENT_TIME      |                  | Time     | Return current time in `HH-MM-SS` format.                       |
| CURRENT_DATE      |                  | Date     | Return current date in `YYYY-MM-DD` format.                     |
| CURRENT_TIMESTAMP |                  | DateTime | Return current date time in `YYYY-MM-DD HH-MM-SS` format.       |
| MAKEDATE          | Integer, Integer | Date     | Create and return a date based on  a year and a number of days. |
| NOW               |                  | DateTime | Return current date time in `YYYY-MM-DD HH-MMSS` format.        |

### Date functions samples

```sql
SELECT CURRENT_TIME()
SELECT CURRENT_DATE()
SELECT CURRENT_TIMESTAMP()
SELECT MAKEDATE(2023, 12)
SELECT NOW()
```

### Numeric Functions

| Name   | Paramters | Return  | Description                                                                  |
| ------ | --------- | ------- | ---------------------------------------------------------------------------- |
| PI     |           | Float   | Return the value of PI.                                                      |
| FLOOR  | Float     | Integer | Returns the largest integer value that is smaller than or equal to a number. |
| ROUND  | Float     | Integer | Returns the nearest integer value.                                           |
| SQUARE | Integer   | Integer | Returns the square of an integer value.                                      |
| ABS    | Integer   | Integer | Returns the absolute value of an integer value.                              |
| SIN    | Float     | Float   | Returns the sine of a number.                                                |

### Numeric functions samples

```sql
SELECT PI()
SELECT FLOOR(1.6)
SELECT ROUND(1.5)
SELECT SQUARE(64)
SELECT ABS(-1)
SELECT SIN(2.0)
```

### General functions

| Name      | Paramters | Return  | Description                                 |
| --------- | --------- | ------- | ------------------------------------------- |
| ISNULL    | ANY       | Boolean | Return TRUE if the argument type is null.   |
| ISNUMERIC | ANY       | Boolean | Return TRUE if the argument type is number. |
| TYPEOF    | ANY       | Text    | Return the argument type name.              |

```sql
SELECT ISNULL(null), ISNULL(1)
SELECT ISNUMERIC(null), ISNUMERIC(1), ISNUMERIC(1.1), ISNUMERIC(false)
SELECT TYPEOF(""), TYPEOF(1), TYPEOF(null)
```