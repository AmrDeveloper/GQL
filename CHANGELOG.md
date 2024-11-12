# Change Log

## Version 0.31.0 _(2024-11-12)_

- Implement Cast function call expression `CAST(expr AS <Type>)`.
- Implement Cast operator `expr::<type>`.
- Created the TypesTable component to register types and aliases.
- Fix schema datetime and updated types from Date to DateTime.
- Fix consuming table name twice.

## Version 0.30.0 _(2024-11-08)_

- Implement Composite type.
- Implement Member access expression for Composite type.
- Replace `atty` with std is_terminal.
- Improve order of iterating over composite type members.

## Version 0.29.0 _(2024-10-30)_

- Implement Dynamic type system.
- Implement new Dynamic values system.
- Improve the output printer implementation.
- Change the analysis format to be similar to popular engines.
- Implement `benchmark` function.
- Implement Bitwise XOR operator for integers.
- Implement Contained By expression using `<@` operator.
- Speedup order by statement by using pre eval map.
- Support multi lines and unicode in `LIKE` and `REGEX` expressions.
- Improve error message for invalid column name.
- Improve safety check for std or aggregation signature.
- Migrate to Gix `0.67.0`.
- Integrate with LineEditor.

## Version 0.28.0 _(2024-09-27)_

- Enable LTO (Link time optimization).
- Optimize set alias for selected expression.
- Handle groups rows elemenations in case group by don't create extra groups #117.
- Implement contains operator for Range in other Range.
- Implement `IF`, `IFNULL` general functions.
- Implement `ARRAT_POSITIONS`, `TRIM_ARRAY` array functions.
- Implement `ISEMPTY` range function.
- Implement `WITH ROLLUP` feature.
- Implement `ORDER BY ... USING <operator>` feature.
- Implement Overlap operator for Arrays and Ranges.
- Remove hidden selection information from the render.
- Handle `WITH ROLLUP` edge case for using one column only in grouping.
- Improve classifying columns on tables.

## Version 0.27.0 _(2024-09-07)_

- Improve comparing Arrays values.
- Support Range data types.
- Simplify the dynamic types helper functions.
- Update `ARRAT_POSITION` signature.
- Implement `int4range`, `daterange`, `tsrange` range function.
- Implement `ARRAY_PREPEND`, `ARRAY_REMOVE` Array functions.
- Implement `ARRAY_APPEND`, `ARRAY_REPLACE` Array functions.
- Implement `BIT_XOR`, `ARRAY_AGG` Aggregation functions.
- Organize the std functions registers.
]- Improve the type checker to resolve dynamic types in arguments.
- Implement Contains operator `@>` between Range and Element.

## Version 0.26.0 _(2024-08-17)_

- Fix iagnostic position when parsing undefined symbol.
- Improve handle error in data provider.
- Don't apply CROSS join operator if one of the tables is empty.
- Update docs for new Data provider design.
- Remove un needed code for remove hidden selection after engine.
- Support exponentiation operator.
- Optimize the calling of data provider if table is empty.
- Implement Select ... INTO OUTFILE.
- Support INTO OUTFILE Terminated options and enclosed.
- Implement Select ... into dumpfile feature.
- Improve error messages when use options with dumpfile.

## Version 0.25.0 _(2024-07-09)_

- Support JOIN more than two tables togther in same query.
- Handle hidden selection with multi tables.
- Support query `datetime` from diffs table.
- Implement LEFT, RIGHT, INNER and CROSS JOINS operation.
- Implement JOIN predicate using `ON` keyword.
- Simplifiy the DataProvider Design.
- Implement Bitwise `xor` operator.
- Change XOR operator to match postgresql.
- Implement `ARRAY_SHUFFLE` Array function.
- Implement `ARRAY_POSITION` Array Function.
- Implement `ARRAY_DIMS` Array Function.
- Support `Infinity` and `NaN` values.
- Support `OUTER` keyword.

## Version 0.24.0 _(2024-06-21)_

- Fix passing global variable value to function call.
- Support slice with default start and end and optimize it.
- Implement PostgreSQL `DISTINCT ON` operator.
- Support `GROUP BY` one or more expression.
- Improve the parse and performance of `DESCRIBE` query.
- Support PostgreSQL boolean values literals.
- Support query `parents_count` of commit.
- Support query `committer_name` and `committer_email` of commit.
- Rename `name` and `email` to `author_name` and `author_email` of commit.
- Support `commit_conventional` function in gitql application.
- Support implicit casting in `WHERE` statement.
- Support implicit casting in `HAVING` statement.

## Version 0.23.0 _(2024-06-13)_

- Fix resolving return type of function with Dynamic and depend on Variant types.
- Fix Projection check for symbols after select statement.

## Version 0.22.1 _(2024-06-12)_

- Hot fix type checker if eval expression without table.

## Version 0.22.0 _(2024-06-12)_

- Allow using column native name in condition after alias it
- Implement Slice expression for Collection `[start : end]`.
- Implement Slice expression for Collection with optional start and end.
- Fix calling function without table name.
- Improve projection columns type checker

## Version 0.21.0 _(2024-06-07)_

- Implement `bit_and` and `bit_or` Aggregation function.
- Implement Array literal and Index expression.
- Implement Index expression for Multi dimensions arrays.
- Implement Index expression and fix exception handling.
- Implement `array_length` Array function.
- Fix runtime exception handling.

## Version 0.20.0 _(2024-05-31)_

- Make `COUNT()` aggregation argument to be `Option<Any>`.
- Replace `lazy_static` crate by `std::sync::OnceLock`.
- Migrate to gix `0.63.0`.
- Fix hidden selection in group by statement.
- Implement `GROUP_CONCAT` Aggregation function.
- Catching function argument with undefined type.
- Update Regex expression implementation to not converted to call.
- Introduce `gitql-core` and `gitql-std` to allow dynamic std.
- Implement `BIN` Text function.
- Implement `bool_and` and `bool_or` aggregation functions.

## Version 0.19.1 _(2024-05-19)_

- Fix Count aggregation function parameter type.

## Version 0.19.0 _(2024-05-18)_

- Improve the structure of the parser.
- Support string literal with single quotes.
- Improve the Type checker to allow mix of optional and varargs parameters.
- Implement `UUID` general function.
- Implement `STR` TExt function.

## Version 0.18.0 _(2024-04-26)_

- Support unicode in the tokenizer.
- Migrate to latest chrono and make clippy happy.
- Support query branch last active date as `updated` column.
- Update `gix` version to `0.62.0`.

## Version 0.17.0 _(2024-04-05)_

- Implement `RAND` Math functions.
- Implement `REGEXPR` expression.
- Implement `NOT REGEXPR` expression.
- fix: Diagnostic position for invalid table name.
- Update `gix` version to `0.61.0`.

## Version 0.16.0 _(2024-03-15)_

- Fix Implicit casting with variant type.
- Support `DIV` and `MOD` keywords.
- Implement `REGEXP_INSTR`, `REGEXP_LIKE`, `REGEXP_REPLACE` and `REGEXP_SUBSTR` Regex function.
- Implement `DATE`, `MINUTE`, `MONTH`, `LAST_DAY` Date functions.
- Implement `WEEKOFYEAR`, `WEEKDAY`, `YEARWEEK` Date functions.
- Update `gix` version to `0.60.0`.

## Version 0.15.0 _(2024-03-01)_

- implement 'describe table_name' to show fields and types of a table.
- Add mysql like `show tables` statement to list all available tables.
- Implement `DAYOFWEEK`, `DAYOFMONTH` and `DAYOFYEAR` Date functions.
- Implement `QUARTER`, `YEAR` and `TO_DAYS` Date function.
- Implement `QUOTENAME` String function.
- Fix Parsing function without right paren at the end

## Version 0.14.0 _(2024-02-16)_

- Implement DataProvider interface to allow custom data.
- Implement Data Schema component to allow custom data schema.
- Improve `ROUND` implementation to supports decimal places.
- Implement `MOD` function.
- Implement Dynamic DataType to be calculated depending on other types.

## Version 0.13.0 _(2024-01-25)_

- Make `SING` function accept Int or Float type.
- Implement `CONCAT_WS` Text function.
- Fix Minus unary operator for f64.
- Implement exporting data as `JSON`, `CSV`.
- Implemnet `DAY` Date function
- Fix not reporting diagnostic when date or time format has number out of range.
- Perform projection operator before export as `JSON`, `CSV`.
- Fix the order of parsing prefix unary with binary expression.
- Handle passing 0 tokens to the parser.

## Version 0.12.0 _(2024-01-13)_

- Change GitQLObject structure to get more speedup and keep values sorted.
- Supports `LIMIT OFFSET` shorthand inspired by MySQL.
- Implement `HOUR` Date functions.
- Implement `STRCMP` Text Function.
- Implement `GREATEST`, `LEAST` General function.
- Implement `ISDATE` Date function.
- Optimize `in` expression in case of empty list.
- Add Support for `NOT IN` expression.
- Report error if user write un expected content after valid statement.
- Fix Date and DateTime incorrect equals #71.
- Allow `BETWEEN` to work with any type.
- Fix ArithmeticExpression expr_type if any side is float.

## Version 0.11.0 _(2023-12-29)_

- Support Assignment expressions `@name := value`.
- Allow Assignment expressions to store aggregation value.
- Allow lazy evaluate any expression that has aggregation value.
- Prevent assign aggregation value to global variable with SET statement.
- Support creating identifier using backticks.
- Support `Either` type in the type system.
- Support `Optional` type in the type system.
- Support `Varargs` type in the type system.
- Implement `ACOS`, `ATAN`, `ATN2` and `SIGN` Math functions.
- Implement `CHARINDEX` Text function.
- Implement `DAYNAME`, `MONTHNAME` Date functions.
- Update `CONCAT` Text function to accept 2 or more Text values.
- Support Aggregation `MAX`, `MIN` to work with different types.
- Support Implicit Type casting for Function arguments.
- Revamp GQLError to a new Diagnostic representation.
- Migrate to Gix v0.57.0.
- Update `CONCAT` function to work with any value type.

## Version 0.10.0 _(2023-12-08)_

- Migrate from `git2` to `gix`.
- Implement `ASIN` function.
- Implement `TAN` function.
- Use current directory as repository path if no path is passed.
- Implement `--query | -q` flat to run a single query without repl mode.
- Support receiving input from a pipe or file redirection.
- Support consuming `;` at the end of query main statement.
- Support User defined variables.
- Suppoer `:=` operator.

## Version 0.9.0 _(2023-11-25)_

- Preallocate the attributes hash with row length.
- Fix Clippy comments and setup CI for Lint and Format.
- Implement `typeof` function.
- Implement `ROUND` function.
- Make Identifiers case-insensitive.
- Support `<=>` operator.
- Implement `SIN` function.
- Implement `COS` function.
- Support Implicit casting Text to Time.
- Support Implicit casting Text to Date.
- Support Implicit casting DateTime to Text.

## Version 0.8.0 _(2023-11-10)_

- Support `GLOB` keyword.
- Support `DISTINCT` keyword.
- Make sure `SELECT *` used with specific table.
- Migrate from Prettytables-rs to comfy-table for render tables.
- Support optional Pagination with user custom page size.
- Support `<>` Operator.
- Implement `PI` function.
- Implement `FLOOR` function.

## Version 0.7.2 _(2023-10-26)_

- Support `NULL` keyword.
- Implement `ISNULL`, `ISNUMERIC` functions.
- Handle crash for undefined symbol as argument at runtime.

## Version 0.7.1 _(2023-09-26)_

- Implement `NOW` function.
- Fix handling grouping with aggregations.
- Print Date and DateTime with formats.

## Version 0.7.0 _(2023-09-22)_

- Support `Like` Expression.
- Remote un needed Check expression.
- Support order by any expression.
- Ignore input if its empty or new line.
- Update Git2 version from `0.17.1` to `0.18.0`.
- Implement 20 Text Functions #13 by @Lilit0x and @tbro.

## Version 0.6.0 _(2023-09-06)_

- Support `<<` and `>>` overflow.
- Fix reporting error with out of index position.
- Implement `Case` expression.
- Support bang equal != for comparisons.
- Improve error message for unexpected token.
- Support negative numbers.
- Add repository path as a field for data all tables.
- Make function name case-insensitive.
- Implement Text `reverse`, `replicate`, `ltrim`, `rtrim` function..
- Select the same field twice.
- Optimize engine to work on one repo only if table name is empty.
- Fix merging empty groups.
- Add custom error message for invalid use of `asc` and `desc`.
- Fix resolving symbols.
- Fix name alias for non symbols.
- Fix name alias for aggregation function.
- Use aggregation function after select statement.
- Don't allow using aggregation in where statement.
- Fix hidden selections.
- Alias the same name twice.
- Fix evaluate function before argument.

## Version 0.5.0 _(2023-08-23)_

- Split the project into multi crates.
- Support query from multi repositories.
- Add CLI flag to enable/disable reporting analysis.
- Report error when `WHERE` or `HAVING` condition is not boolean.
- Introduce Runtime exceptions.
- Report runtime exception for divide by zero.
- Report runtime exception for reminder by zero.
- Report runtime exception for right and left shift overflow.

## Version 0.4.1 _(2023-07-19)_

- Prevent crash and report more error messages.
- Make sure select statement is used before any other statement.
- Make sure having is used after group by expression.

## Version 0.4.0 _(2023-07-14)_

- Support hex decimal number format.
- Support binary decimal number format.
- Support octal decimal number format.
- Support Aggregations function without selecting the field.
- Support Merging group if it only select aggregations.
- Implement Aggregation functions `avg`.
- Improve render performance.
- Allow calling aggregation function with upper or lower cases.

## Version 0.3.0 _(2023-07-07)_

- Implement Aggregation functions `count`, `max`, `min`, `sum`.
- Implement insertions, deletations, file changes for diffs table.
- Remove un needed dependencies #4.
- Publish the project on crates.io.
- Create docs website.
- Support Number expression.
- Support Arithmetics operators.
- Support Bitwise operators.
- Support selecting commit id

## Version 0.2.0 _(2023-06-27)_

- Support Aggregation Functions.
- Select number of commits for each branch.
- Add column alias name.
- Add Group by statement.
- Add Having statement.
- Support order by Ascending and Descending.
- Introduce simple type system with error messages.
- Report error messages for transformations.
- Allow engine to reorder the commands.
- Print output in table format.

## Version 0.1.0 _(2023-06-16)_

- First release of GQL.
