## GitQL Application Functions

Beside the common SQL scalar and aggregation functions there are some extra functions related to the GitQL as application not as SDK,
those functions are available only in the gitql application.

### GitQL functions

| Name                | Parameters | Return | Description                                                        |
| ------------------- | ---------- | ------ | ------------------------------------------------------------------ |
| COMMIT_CONVENTIONAL | Text       | Text   | Return the commit conventional from commits (Part before the `:`). |

### samples

```sql
SELECT title FROM commits WHERE commit_conventional(title) = "feat"
```