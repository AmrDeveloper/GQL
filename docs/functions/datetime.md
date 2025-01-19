### Date, Time and DateTime operators

| Operator | Arguments       | Description                    |
| -------- | --------------- | ------------------------------ |
| +        | (Date, Integer) | Add a number of days to a date |

### Date, Time and DateTime functions

| Name              | Parameters                | Return   | Description                                                                   |
| ----------------- | ------------------------- | -------- | ----------------------------------------------------------------------------- |
| Date              | DateTime                  | Date     | Extracts the date part from a datetime expression.                            |
| CURRENT_TIME      |                           | Time     | Return current time in `HH:MM:SS` format.                                     |
| CURRENT_DATE      |                           | Date     | Return current date in `YYYY-MM-DD` format.                                   |
| CURRENT_TIMESTAMP |                           | DateTime | Return current date time in `YYYY-MM-DD HH:MM:SS` format.                     |
| MAKEDATE          | Integer, Integer          | Date     | Create and return a date based on a year and a number of days.                |
| MAKETIME          | Integer, Integer, Integer | Time     | Create and return a time value based on an hour, minute, and second value.    |
| NOW               |                           | DateTime | Return current date time in `YYYY-MM-DD HH:MM:SS` format.                     |
| Day               | Date                      | Integer  | Returns the index of the day (1 to 31) in the date.                           |
| DAYNAME           | Date                      | Text     | Returns the name of the day given a timestamp.                                |
| MONTHNAME         | Date                      | Text     | Returns the name of the month given a timestamp.                              |
| HOUR              | DateTime                  | Integer  | Returns the hour part of a datetime.                                          |
| MINUTE            | DateTime                  | Integer  | Returns the minute part of a datetime.                                        |
| ISDATE            | Any                       | Boolean  | Return TRUE if the argument type is Date.                                     |
| DAYOFWEEK         | Date                      | Integer  | Returns the day of the week for a given date (a number from 1 to 7)           |
| DAYOFMONTH        | Date                      | Integer  | Returns the day of the month for a given date (a number from 1 to 31)         |
| DAYOFYEAR         | Date                      | Integer  | Returns the day of the year for a given date (a number from 1 to 366)         |
| WEEKOFYEAR        | Date                      | Integer  | Returns the week number for a given date (a number from 1 to 53).             |
| QUARTER           | Date                      | Integer  | Returns the quarter of the year for a given date value (a number from 1 to 4) |
| YEAR              | Date                      | Integer  | Returns the year part of the date                                             |
| MONTH             | Date                      | Integer  | Returns the month part of the date (a number from 1 to 12)                    |
| WEEKDAY           | Date                      | Integer  | Returns the weekday number of the date (from 0 monday to 6 sunday)            |
| TO_DAYS           | Date                      | Integer  | Returns the number of days between a date and date "0000-00-00"               |
| LAST_DAY          | Date                      | Date     | Returns the last day of the month for a given date                            |
| YEARWEEK          | Date                      | Text     | Returns the year and week number (a number from 0 to 53) for a given date     |
