### Interval expression

The Interval is another type of data type used to store and deploy Time in years, months, days, hours, minutes, seconds. And the years, months and days, hours and minutes values are integers values, whereas the second's field can be the fractions values.

Inspired by PostgreSQL, interval data type value involves 16 bytes storage size, which helps to store a period with the acceptable range from -178000000 years to 178000000 years.

#### Examples

```SQL
SELECT INTERVAL '1 years 1 days'  = INTERVAL '1 years 2 days';
SELECT INTERVAL '1 years 1 days' != INTERVAL '1 years 2 days';
SELECT INTERVAL '1 years 1 days'  + INTERVAL '1 years 2 days';
SELECT INTERVAL '1 years 1 days'  - INTERVAL '1 years 2 days';
SELECT INTERVAL '1 years 1 days' * 2;
SELECT INTERVAL '2 years 2 days' / 2;
```