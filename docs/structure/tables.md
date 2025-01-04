## GQL Tables

You can see list of all tables directly in the repl using

```sql
show tables

```

---

### References table

| Name      | Type | Description          |
| --------- | ---- | -------------------- |
| name      | Text | Reference name       |
| full_name | Text | Reference full name  |
| type      | Text | Reference type       |
| repo      | Text | Repository full path |

### Commits table

---

| Name            | Type     | Description              |
| --------------- | -------- | ------------------------ |
| commit_id       | Text     | Commit id                |
| title           | Text     | Commit title             |
| message         | Text     | Commit full message      |
| author_name     | Text     | Author name              |
| author_email    | Text     | Author email             |
| committer_name  | Text     | Committer name           |
| committer_email | Text     | Committer email          |
| parents_count   | Integer  | Number of commit parents |
| datetime        | DateTime | Commit date time         |
| repo            | Text     | Repository full path     |

---

### Diffs table

| Name          | Type        | Description                       |
| ------------- | ----------- | --------------------------------- |
| commit_id     | Text        | Commit id                         |
| author_name   | Text        | Author name                       |
| author_email  | Text        | Author email                      |
| insertions    | Integer     | Number of inserted lines          |
| removals      | Integer     | Number of deleted lines           |
| files_changed | Integer     | Number of file changed            |
| diff_changes  | DiffChanges | Diff content and info for changes |
| datetime      | DateTime    | Commit date time                  |
| repo          | Text        | Repository full path              |

---

## Diffs Changes table

| Name       | Type    | Description                                                              |
| ---------- | ------- | ------------------------------------------------------------------------ |
| commit_id  | Text    | Commit id                                                                |
| insertions | Integer | Number of inserted lines in one change                                   |
| removals   | Integer | Number of deleted lines in one change                                    |
| mode       | Text    | Change mode A for Add, D for Delete, M for Modification or R for Rewrite |
| path       | Text    | Location of the change                                                   |
| repo       | Text    | Repository full path                                                     |

---

### Branches table

| Name         | Type     | Description                      |
| ------------ | -------- | -------------------------------- |
| name         | Text     | Branch name                      |
| commit_count | Number   | Number of commits in this branch |
| is_head      | Bool     | Is the head branch               |
| is_remote    | Bool     | Is a remote branch               |
| updated      | DateTime | Last update Commit date time     |
| repo         | Text     | Repository full path             |

---

### Tags table

| Name | Type | Description          |
| ---- | ---- | -------------------- |
| name | Text | Tag name             |
| repo | Text | Repository full path |

---

### List all tables in the current schema

```sql
SHOW TABLES;
```

---

### Query the description of table by name

```sql
DESCRIBE commits;
DESCRIBE branches;
```
