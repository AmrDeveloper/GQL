## GitQL Application Functions

Beside the common SQL scalar, aggregation and window functions there are some extra functions related to the GitQL as application not as SDK,
those functions are available only in the gitql application.

### GitQL Commits functions

| Name                | Parameters | Return | Description                                                        |
| ------------------- | ---------- | ------ | ------------------------------------------------------------------ |
| COMMIT_CONVENTIONAL | Text       | Text   | Return the commit conventional from commits (Part before the `:`). |

### GitQL Diffs functions

| Name                               | Parameters        | Return      | Description                                                              |
| ---------------------------------- | ----------------- | ----------- | ------------------------------------------------------------------------ |
| DIFF_CONTENT                       | DiffChanges       | Text        | Return the full content of all changes appended together.                |
| DIFF_ADDED_CONTENT                 | DiffChanges       | Text        | Return the added content of all changes appended together.               |
| DIFF_DELETED_CONTENT               | DiffChanges       | Text        | Return the deleted content of all changes appended together.             |
| DIFF_MODIFIED_CONTENT              | DiffChanges       | Text        | Return the modified content of all changes appended together.            |
| DIFF_CONTENT_CONTAINS              | DiffChanges, Text | Text        | Return true if the all content of changes contains second argument.      |
| DIFF_ADDED_CONTENT_CONTAINS        | DiffChanges, Text | Text        | Return true if the added content of changes contains second argument.    |
| DIFF_DELETED_CONTENT_CONTAINS      | DiffChanges, Text | Text        | Return true if the deleted content of changes contains second argument.  |
| DIFF_MODIFICATION_CONTENT_CONTAINS | DiffChanges, Text | Text        | Return true if the modified content of changes contains second argument. |
| DIFF_CHANGED_FILES                 | DiffChanges       | Array<Text> | Return changes files in this change as array of strings.                 |
| DIFF_FILES_COUNT                   | DiffChanges       | Integer     | Return number of unique files changes in this commit.                    |
| IS_DIFF_HAS_FILE                   | DiffChanges, Text | Boolean     | Return true if this diff changes contains file.                          |
