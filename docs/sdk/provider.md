After defining the Schema of your custom data to the SDK you need to teach the Engine from where and how it can load the data? this can be different from project to another one, so this part is created as an interface called `DataProvider`, which you can implement it for any kind of data, files, git, code AST, API response, system information ...etc, Sky is the limit.

### Data Provider Interface

The Data Provider is a single function interface that take the target table name and which columns the query need,
and then you either return the list of Rows or an String with error message.

```rust linenums="1"
pub trait DataProvider {
    fn provide(&self, 
        table: &str, 
        selected_columns: &[String]
    ) -> Result<Vec<Row>, String>;
}
```

> **_NOTE:_**  You don't need to check here if this column is valid or not, this done in the parser depending on your schema.

---

> **_NOTE:_**  You can cache and restore the data from and to this provider.

---

> **_NOTE:_**  You can traverse and build the list of rows in single or multi thread.

---

So lets try to implement a simple Data provider for file system, first lets create a custom data provider that contains what paths we should search in and what files we should excludes.

```rust linenums="1"
use gitql_engine::data_provider::DataProvider;

pub struct FileDataProvider {
    pub paths: Vec<String>,
    pub excludes: Vec<String>,
}

impl FileDataProvider {
    pub fn new(paths: Vec<String>, excludes: Vec<String>) -> Self {
        Self { paths, excludes }
    }
}
```

---

Now lets implement the `provide` function to provide data from file system, i recommend to create a function
for each table, to keep every thing organize

```rust linenums="1"
impl DataProvider for FileDataProvider {
    fn provide(&self, table: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
        match table {
            "files" => select_files(&self.paths, &self.excludes, selected_columns),
            _ => Ok(vec![Row { values: vec![] }]),
        }
    }
}

fn select_files(
    paths: &[String],
    excludes: &[String],
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let files = collect_paths_nested_files_and_excludes(paths, excludes);

    let mut rows: Vec<Row> = Vec::with_capacity(paths.len());

    // Iterate over each file to extract information that we need
    for file in files.iter() {
        // Preallocate the list of values in this row
        let mut values: Vec<Value> = Vec::with_capacity(selected_columns.len());
        let path = Path::new(&file);

        for column_name in selected_columns {
            if column_name == "file_name" {
                let file_path_string = path.to_str().unwrap_or("");
                values.push(Box::new(TextValue {
                    value: file_path_string.to_string(),
                }));
                continue;
            }

            if column_name == "is_dir" {
                values.push(Box::new(BoolValue {
                    value: path.is_dir(),
                }));
                continue;
            }

            // If this symbol is not a column name
            values.push(Box::new(NullValue));
        }

        // Create a new fow for this file and append it to the result rows
        let row = Row { values };
        rows.push(row);
    }

    // Return the rows if every thing is Okay, or return error
    Ok(rows)
}
```

---

To create DataProvider instance

```rust linenums="1"
let provider: Box<dyn DataProvider> = 
    Box::new(FileDataProvider::new(paths, excludes));
```

And now our Data Provider is done and ready to use, this code is not dummy, it's actual code from `FileQL` Data Provider.
