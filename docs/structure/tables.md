## GQL Tables 
---

### References table

| Name      | Type | Description         |
| --------- | ---- | ------------------- |
| name      | Text | Reference name      |
| full_name | Text | Reference full name |
| type      | Text | Reference type      |

### Commits table

---

| Name      | Type | Description         |
| --------- | ---- | ------------------- |
| commit_id | Text | Commit id           |
| title     | Text | Commit title        |
| message   | Text | Commit full message |
| name      | Text | Author name         |
| email     | Text | Author email        |
| time      | Date | Commit date         |

---

### Diffs table

| Name          | Type   | Description              |
| ------------- | ------ | ------------------------ |
| commit_id     | Text   | Commit id                |
| name          | Text   | Author name              |
| email         | Text   | Author email             |
| insertions    | Number | Number of inserted lines |
| deletions     | Number | Number of deleted lines  |
| files_changed | Number | Number of file changed   |

---

### Branches table

| Name         | Type   | Description                      |
| ------------ | ------ | -------------------------------- |
| name         | Text   | Branch name                      |
| commit_count | Number | Number of commits in this branch |
| is_head      | Bool   | Is the head branch               |
| is_remote    | Bool   | Is a remote branch               |

---

### Tags table

| Name | Type | Description |
| ---- | ---- | ----------- |
| name | Text | Tag name    |