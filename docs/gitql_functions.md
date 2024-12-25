## GitQL Application Functions

Beside the common SQL scalar, aggregation and window functions there are some extra functions related to the GitQL as application not as SDK,
those functions are available only in the gitql application.

### GitQL Commits functions

| Name                | Parameters | Return | Description                                                        |
| ------------------- | ---------- | ------ | ------------------------------------------------------------------ |
| COMMIT_CONVENTIONAL | Text       | Text   | Return the commit conventional from commits (Part before the `:`). |

### GitQL Diffs functions

| Name                     | Parameters  | Return  | Description                                           |
| ------------------------ | ----------- | ------- | ----------------------------------------------------- |
| DIFF_CHANGES_FILES_COUNT | DiffChanges | Integer | Return number of unique files changes in this commit. |
