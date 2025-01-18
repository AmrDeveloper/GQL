### Aggregations functions

A Window function in GitQL performs a calculation on a window (frame) of values and returns a single value

| Name        | Parameters | Return | Description                                                                        |
| ----------- | ---------- | ------ | ---------------------------------------------------------------------------------- |
| FIRST_VALUE | ANY        | Any    | Return first value in the window of values                                         |
| NTH_VALUE   | ANY, INT   | Any    | Return n value in the window of values                                             |
| LAST_VALUE  | ANY        | Any    | Return last value in the window of values                                          |
| ROW_NUMBER  |            | INT    | Return unique sequential integer to each row within the partition, starting from 1 |
