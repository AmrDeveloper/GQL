An aggregate function in GQL performs a calculation on multiple values and returns a single value

| Name         | Parameters | Return  | Description                                                       |
| ------------ | ---------- | ------- | ----------------------------------------------------------------- |
| MAX          | ANY        | Any     | Return maximum value of it for all elements until the current one |
| MIN          | ANY        | Any     | Return minimum value of it for all elements until the current one |
| SUM          | Number     | Number  | Return the sum of items in a group.                               |
| AVG          | Number     | Number  | Return the average of items in a group                            |
| COUNT        | ANY?       | Any     | Return the number of items in a group                             |
| GROUP_CONCAT | ...Any     | Text    | Return string with concatenated non-NULL value from a group       |
| BOOL_AND     | Boolean    | Boolean | Return true if all input values are true, otherwise false         |
| BOOL_OR      | Boolean    | Boolean | Return true if at least one input value is true, otherwise false  |
| BIT_AND      | Integer    | Integer | Return bitwise AND of all non-null input values, or null if none  |
| BIT_OR       | Integer    | Integer | Return bitwise OR of all non-null input values, or null if none   |
| BIT_XOR      | Integer    | Integer | Return bitwise XOR of all non-null input values, or null if none  |
