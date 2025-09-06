### Array operators

| Operator | Arguments            | Description |
| -------- | -------------------- | ----------- |
| @>       | (Array<T>, T)        | Contains    |
| <@       | (T, Array<T>)        | Contain by  |
| &&       | (Array<T>, Array<T>) | Overlap     |

### Array functions

| Name            | Parameters      | Return         | Description                                                                       |
| --------------- | --------------- | -------------- | --------------------------------------------------------------------------------- |
| ARRAY_APPEND    | Array, Any      | Array          | Append element to the end of the array.                                           |
| ARRAY_PREPEND   | Any, Array      | Array          | Append element to the start of the array.                                         |
| ARRAY_REMOVE    | Array, Any      | Array          | Remove element from the array.                                                    |
| ARRAY_CAT       | Array, Array    | Array          | Concatenates two arrays with the same type.                                       |
| ARRAY_LENGTH    | Array           | Integer        | Return the length of Array.                                                       |
| ARRAY_SHUFFLE   | Array           | Array          | Return Randomly shuffles the first dimension of the array.                        |
| ARRAY_POSITION  | Array, Any      | Integer        | Return the position of element in array or NULL if not found.                     |
| ARRAY_POSITIONS | Array, Any      | Array<Integer> | Return the an array of positions of element in array.                             |
| ARRAY_DIMS      | Array           | Text           | Returns a text representation of the array's dimensions.                          |
| ARRAY_REPLACE   | Array, Any, Any | Array          | Replaces each array element equal to the second argument with the third argument. |
| TRIM_ARRAY      | Array, Integer  | Array          | Remove the last n elements from the array.                                        |
| CARDINALITY     | Array           | Integer        | Counts the total number of array elements.                                        |