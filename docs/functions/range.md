### Range operators

| Operator | Arguments            | Description |
| -------- | -------------------- | ----------- |
| @>       | (Range<T>, T)        | Contains    |
| <@       | (T, Range<T>)        | Contain by  |
| &&       | (Range<T>, Range<T>) | Overlap     |

### Range functions

| Name      | Parameters         | Return          | Description                                          |
| --------- | ------------------ | --------------- | ---------------------------------------------------- |
| INT4RANGE | Integer, Integer   | Range(Integer)  | Create a Range of integer type with start and end.   |
| DATERANGE | Date, Date         | Range(Date)     | Create a Range of date type with start and end.      |
| TSRANGE   | DateTime, DateTime | Range(DateTime) | Create a Range of date time type with start and end. |
| ISEMPTY   | Range              | Boolean         | Return true of this range is empty.                  |