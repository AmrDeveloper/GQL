Change Log
==========

Version 0.7.1 *(2023-09-26)*
-----------------------------

* Implement `NOW` function.
* Fix handling grouping with aggregations.
* Print Date and DateTime with formats.
* Update gitql-ast to version `0.4.0`.
* Update gitql-cli to version `0.5.0`.
* Update gitql-parser to version `0.4.0`.
* Update gitql-engine to version `0.5.0`.
* 
Version 0.7.0 *(2023-09-22)*
-----------------------------

* Support `Like` Expression.
* Remote un needed Check expression.
* Support order by any expression.
* Ignore input if its empty or new line.
* Update Git2 version from `0.17.1` to `0.18.0`.
* Implement 20 Text Functions #13 by @Lilit0x and @tbro.
* Update gitql-ast to version `0.3.0`.
* Update gitql-cli to version `0.5.0`.
* Update gitql-parser to version `0.4.0`.
* Update gitql-engine to version `0.5.0`.

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
* Alias the same name twice
* Fix evaluate function before argument

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
* Allow calling aggregation function with upper or lowre cases.

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