### Call expression

## 1. Standard Function Calls

Standard functions call operate on individual rows and return a single value per row, with syntax similar to most programming languages for example

```sql
LEN(name)
LOWER(author_name)
```

## 2. Aggregate Function Calls

Aggregate functions call has the same syntax like standard function call but it's operate on a set of rows (a group) and return a single value for the entire group. They are often used with the `GROUP BY` clause, the value of aggregation function can be used only after group by statement.

```sql
SELECT author_name, COUNT(author_name) AS commit_num FROM commits GROUP BY author_name, author_email ORDER BY commit_num DESC LIMIT 10
```

## 3. Window functions

Window functions perform calculations across a set of rows that are related to the current row. Unlike aggregate functions with GROUP BY, window functions do not collapse rows into a single output row. Instead, they return a value for each input row based on a "window" of related rows,
in window function call you must to explicit define `OVER` clauses even if it empty, also you can use aggregation function as window function.

```sql
SELECT emp_name, dep_name, ROW_NUMBER() OVER(PARTITION BY dep_name) AS row_number FROM emp_salaries

SELECT emp_name,
       dep_name,
       ROW_NUMBER() OVER partition_dep_order_salary_des AS row_number_per_department,
       MIN(salary) OVER partition_dep_order_salary_des AS min_salary_per_department,
       MAX(salary) OVER partition_dep_order_salary_des AS max_salary_per_department
FROM emp_salaries
WINDOW partition_dep_order_salary_des AS (PARTITION BY dep_name ORDER BY salary DESC)
ORDER BY dep_name ASC NULLS LAST;
```