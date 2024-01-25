Change Log
==========

Version 0.13.0 *(2024-01-25)*
-----------------------------

* Make `SING` function accept Int or Float type.
* Implement `CONCAT_WS` Text function.
* Fix Minus unary operator for f64.
* Implement exporting data as `JSON`, `CSV`.
* Implemnet `DAY` Date function
* Fix not reporting diagnostic when date or time format has number out of range.
* Perform projection operator before export as `JSON`, `CSV`.
* Fix the order of parsing prefix unary with binary expression.
* Handle passing 0 tokens to the parser.

Version 0.12.0 *(2024-01-13)*
-----------------------------

* Change GitQLObject structure to get more speedup and keep values sorted.
* Supports `LIMIT OFFSET` shorthand inspired by MySQL.
* Implement `HOUR` Date functions.
* Implement `STRCMP` Text Function.
* Implement `GREATEST`, `LEAST` General function.
* Implement `ISDATE` Date function.
* Optimize `in` expression in case of empty list.
* Add Support for `NOT IN` expression.
* Report error if user write un expected content after valid statement.
* Fix Date and DateTime incorrect equals #71.
* Allow `BETWEEN` to work with any type.
* Fix ArithmeticExpression expr_type if any side is float.

Version 0.11.0 *(2023-12-29)*
-----------------------------

* Support Assignment expressions `@name := value`.
* Allow Assignment expressions to store aggregation value.
* Allow lazy evaluate any expression that has aggregation value.
* Prevent assign aggregation value to global variable with SET statement.
* Support creating identifier using backticks.
* Support `Either` type in the type system.
* Support `Optional` type in the type system.
* Support `Varargs` type in the type system.
* Implement `ACOS`, `ATAN`, `ATN2` and `SIGN` Math functions.
* Implement `CHARINDEX` Text function.
* Implement `DAYNAME`, `MONTHNAME` Date functions.
* Update `CONCAT` Text function to accept 2 or more Text values.
* Support Aggregation `MAX`, `MIN` to work with different types.
* Support Implicit Type casting for Function arguments.
* Revamp GQLError to a new Diagnostic representation.
* Migrate to Gix v0.57.0.
* Update `CONCAT` function to work with any value type.

Version 0.10.0 *(2023-12-08)*
-----------------------------

* Migrate from `git2` to `gix`.
* Implement `ASIN` function.
* Implement `TAN` function.
* Use current directory as repository path if no path is passed.
* Implement `--query | -q` flat to run a single query without repl mode.
* Support receiving input from a pipe or file redirection.
* Support consuming `;` at the end of query main statement.
* Support User defined variables.
* Suppoer `:=` operator.

Version 0.9.0 *(2023-11-25)*
-----------------------------

* Preallocate the attributes hash with row length.
* Fix Clippy comments and setup CI for Lint and Format.
* Implement `typeof` function.
* Implement `ROUND` function.
* Make Identifiers case-insensitive.
* Support `<=>` operator.
* Implement `SIN` function.
* Implement `COS` function.
* Support Implicit casting Text to Time.
* Support Implicit casting Text to Date.
* Support Implicit casting DateTime to Text.

Version 0.8.0 *(2023-11-10)*
-----------------------------

* Support `GLOB` keyword.
* Support `DISTINCT` keyword.
* Make sure `SELECT *` used with specific table.
* Migrate from Prettytables-rs to comfy-table for render tables.
* Support optional Pagination with user custom page size.
* Support `<>` Operator.
* Implement `PI` function.
* Implement `FLOOR` function.

Version 0.7.2 *(2023-10-26)*
-----------------------------

* Support `NULL` keyword.
* Implement `ISNULL` function.
* Implement `ISNUMERIC` function.
* Handle crash for undefined symbol as argument at runtime.

Version 0.7.1 *(2023-09-26)*
-----------------------------

* Implement `NOW` function.
* Fix handling grouping with aggregations.
* Print Date and DateTime with formats.

Version 0.7.0 *(2023-09-22)*
-----------------------------

* Support `Like` Expression.
* Remote un needed Check expression.
* Support order by any expression.
* Ignore input if its empty or new line.
* Update Git2 version from `0.17.1` to `0.18.0`.
* Implement 20 Text Functions #13 by @Lilit0x and @tbro.

Version 0.6.0 *(2023-09-06)*
-----------------------------

* Support `<<` and `>>` overflow.
* Fix reporting error with out of index position.
* Implement `Case` expression.
* Support bang equal != for comparisons.
* Improve error message for unexpected token.
* Support negative numbers.
* Add repository path as a field for data all tables.
* Make function name case-insensitive.
* Support Text `reverse` function.
* Support Text `replicate` function.
* Support Text `ltrim`, `rtrim` function.
* Select the same field twice.
* Optimize engine to work on one repo only if table name is empty.
* Fix merging empty groups.
* Add custom error message for invalid use of `asc` and `desc`.
* Fix resolving symbols.
* Fix name alias for non symbols.
* Fix name alias for aggregation function.
* Use aggregation function after select statement.
* Don't allow using aggregation in where statement.
* Fix hidden selections.
* Alias the same name twice.
* Fix evaluate function before argument.

Version 0.5.0 *(2023-08-23)*
-----------------------------

* Split the project into multi crates.
* Support query from multi repositories.
* Add CLI flag to enable/disable reporting analysis.
* Report error when `WHERE` or `HAVING` condition is not boolean.
* Introduce Runtime exceptions.
* Report runtime exception for divide by zero.
* Report runtime exception for reminder by zero.
* Report runtime exception for right and left shift overflow.

Version 0.4.1 *(2023-07-19)*
-----------------------------

* Prevent crash and report more error messages.
* Make sure select statement is used before any other statement.
* Make sure having is used after group by expression.

Version 0.4.0 *(2023-07-14)*
-----------------------------

* Support hex decimal number format.
* Support binary decimal number format.
* Support octal decimal number format.
* Support Aggregations function without selecting the field.
* Support Merging group if it only select aggregations.
* Implement Aggregation functions `avg`.
* Improve render performance.
* Allow calling aggregation function with upper or lower cases.

Version 0.3.0 *(2023-07-07)*
-----------------------------

* Implement Aggregation functions `count`, `max`, `min`, `sum`.
* Implement insertions, deletations, file changes for diffs table.
* Remove un needed dependencies #4.
* Publish the project on crates.io.
* Create docs website.
* Support Number expression.
* Support Arithmetics operators.
* Support Bitwise operators.
* Support selecting commit id

Version 0.2.0 *(2023-06-27)*
-----------------------------

* Support Aggregation Functions.
* Select number of commits for each branch.
* Add column alias name.
* Add Group by statement.
* Add Having statement.
* Support order by Ascending and Descending.
* Introduce simple type system with error messages.
* Report error messages for transformations.
* Allow engine to reorder the commands.
* Print output in table format.

Version 0.1.0 *(2023-06-16)*
-----------------------------

* First release of GQL.