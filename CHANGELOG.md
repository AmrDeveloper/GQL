Change Log
==========

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