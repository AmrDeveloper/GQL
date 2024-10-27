use gitql_core::object::Row;

use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::datetime::DateTimeValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::text::TextValue;
use gitql_engine::data_provider::DataProvider;

use gix::refs::Category;

pub struct GitDataProvider {
    pub repos: Vec<gix::Repository>,
}

impl GitDataProvider {
    pub fn new(repos: Vec<gix::Repository>) -> Self {
        Self { repos }
    }
}

impl DataProvider for GitDataProvider {
    fn provide(&self, table: &str, selected_columns: &[String]) -> Result<Vec<Row>, String> {
        let mut rows: Vec<Row> = vec![];

        for repository in &self.repos {
            let mut repo_rows =
                select_gql_objects(repository, table.to_string(), selected_columns)?;
            rows.append(&mut repo_rows);
        }

        Ok(rows)
    }
}

fn select_gql_objects(
    repo: &gix::Repository,
    table: String,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    match table.as_str() {
        "refs" => select_references(repo, selected_columns),
        "commits" => select_commits(repo, selected_columns),
        "branches" => select_branches(repo, selected_columns),
        "diffs" => select_diffs(repo, selected_columns),
        "tags" => select_tags(repo, selected_columns),
        _ => Ok(vec![Row { values: vec![] }]),
    }
}

fn select_references(
    repo: &gix::Repository,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let git_references = repo.references();
    if let Err(error) = git_references {
        return Err(error.to_string());
    }

    let repo_path = repo.path().to_str().unwrap().to_string();
    let references = git_references.ok().unwrap();
    let mut rows: Vec<Row> = vec![];

    for reference in references.all().unwrap().flatten() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());
        for field_name in selected_columns {
            if field_name == "name" {
                let name = reference
                    .name()
                    .category_and_short_name()
                    .map(|(_, sn)| sn)
                    .unwrap_or("".into())
                    .to_string();
                values.push(Box::new(TextValue { value: name }));
                continue;
            }

            if field_name == "full_name" {
                let full_name = reference.name().as_bstr().to_string();
                values.push(Box::new(TextValue { value: full_name }));
                continue;
            }

            if field_name == "type" {
                let category = reference.name().category();
                if category.map_or(false, |cat| cat == Category::LocalBranch) {
                    values.push(Box::new(TextValue {
                        value: "branch".to_string(),
                    }));
                } else if category.map_or(false, |cat| cat == Category::RemoteBranch) {
                    values.push(Box::new(TextValue {
                        value: "remote".to_string(),
                    }));
                } else if category.map_or(false, |cat| cat == Category::Tag) {
                    values.push(Box::new(TextValue {
                        value: "tag".to_string(),
                    }));
                } else if category.map_or(false, |cat| cat == Category::Note) {
                    values.push(Box::new(TextValue {
                        value: "note".to_string(),
                    }));
                } else {
                    values.push(Box::new(TextValue {
                        value: "other".to_string(),
                    }));
                }
                continue;
            }

            if field_name == "repo" {
                values.push(Box::new(TextValue {
                    value: repo_path.to_string(),
                }));
                continue;
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_commits(repo: &gix::Repository, selected_columns: &[String]) -> Result<Vec<Row>, String> {
    let head_id = repo.head_id();
    if let Err(error) = head_id {
        return Err(error.to_string());
    }

    let repo_path = repo.path().to_str().unwrap().to_string();
    let revwalk = head_id.unwrap().ancestors().all().unwrap();
    let mut rows: Vec<Row> = vec![];

    for commit_info in revwalk {
        let commit_info = commit_info.unwrap();
        let commit = repo.find_object(commit_info.id).unwrap().into_commit();
        let commit = commit.decode().unwrap();

        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());
        for column_name in selected_columns {
            if column_name == "commit_id" {
                values.push(Box::new(TextValue {
                    value: commit_info.id.to_string(),
                }));
                continue;
            }

            if column_name == "author_name" {
                values.push(Box::new(TextValue {
                    value: commit.author().name.to_string(),
                }));
                continue;
            }

            if column_name == "author_email" {
                values.push(Box::new(TextValue {
                    value: commit.author().email.to_string(),
                }));
                continue;
            }

            if column_name == "committer_name" {
                values.push(Box::new(TextValue {
                    value: commit.committer().name.to_string(),
                }));
                continue;
            }

            if column_name == "committer_email" {
                values.push(Box::new(TextValue {
                    value: commit.committer().email.to_string(),
                }));
                continue;
            }

            if column_name == "title" {
                values.push(Box::new(TextValue {
                    value: commit.message().summary().to_string(),
                }));
                continue;
            }

            if column_name == "message" {
                values.push(Box::new(TextValue {
                    value: commit.message.to_string(),
                }));
                continue;
            }

            if column_name == "datetime" {
                let time_stamp = commit_info
                    .commit_time
                    .unwrap_or_else(|| commit.time().seconds);
                values.push(Box::new(DateTimeValue { value: time_stamp }));
                continue;
            }

            if column_name == "parents_count" {
                values.push(Box::new(IntValue {
                    value: commit.parents.len() as i64,
                }));
                continue;
            }

            if column_name == "repo" {
                values.push(Box::new(TextValue {
                    value: repo_path.to_string(),
                }));
                continue;
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_branches(
    repo: &gix::Repository,
    selected_columns: &[String],
) -> Result<Vec<Row>, String> {
    let mut rows: Vec<Row> = vec![];

    let repo_path = repo.path().to_str().unwrap().to_string();
    let platform = repo.references().unwrap();
    let local_branches = platform.local_branches().unwrap();
    let remote_branches = platform.remote_branches().unwrap();
    let local_and_remote_branches = local_branches.chain(remote_branches);
    let head_ref_result = repo.head_ref();
    if let Err(error) = head_ref_result {
        return Err(error.to_string());
    }

    let head_ref_option = head_ref_result.unwrap();
    if head_ref_option.is_none() {
        return Ok(rows);
    }

    let head_ref = head_ref_option.unwrap();

    for mut branch in local_and_remote_branches.flatten() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());

        for column_name in selected_columns {
            if column_name == "name" {
                let branch_name = branch.name().as_bstr().to_string();
                values.push(Box::new(TextValue { value: branch_name }));
                continue;
            }

            if column_name == "commit_count" {
                let commit_count = if let Some(id) = branch.try_id() {
                    if let Ok(revwalk) = id.ancestors().all() {
                        revwalk.count() as i64
                    } else {
                        -1
                    }
                } else {
                    -1
                };

                values.push(Box::new(IntValue {
                    value: commit_count,
                }));
                continue;
            }

            if column_name == "updated" {
                if let Ok(top_commit_id) = branch.peel_to_id_in_place() {
                    let revwalk = top_commit_id.ancestors().all().unwrap();
                    if let Some(commit_info) = revwalk.into_iter().next() {
                        let commit_info = commit_info.unwrap();
                        if let Some(commit_timestamp) = commit_info.commit_time {
                            values.push(Box::new(DateTimeValue {
                                value: commit_timestamp,
                            }));
                            continue;
                        }

                        let commit = repo.find_object(commit_info.id).unwrap().into_commit();
                        let commit = commit.decode().unwrap();
                        let time_stamp = commit.time().seconds;
                        values.push(Box::new(DateTimeValue { value: time_stamp }));
                        continue;
                    }
                }

                values.push(Box::new(NullValue));
                continue;
            }

            if column_name == "is_head" {
                values.push(Box::new(BoolValue {
                    value: branch.inner == head_ref.inner,
                }));
                continue;
            }

            if column_name == "is_remote" {
                let is_remote = branch
                    .name()
                    .category()
                    .map_or(false, |cat| cat == Category::RemoteBranch);
                values.push(Box::new(BoolValue { value: is_remote }));
                continue;
            }

            if column_name == "repo" {
                values.push(Box::new(TextValue {
                    value: repo_path.to_string(),
                }));
                continue;
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_diffs(repo: &gix::Repository, selected_columns: &[String]) -> Result<Vec<Row>, String> {
    let repo = {
        let mut repo = repo.clone();
        repo.object_cache_size_if_unset(4 * 1024 * 1024);
        repo
    };

    let revwalk = repo.head_id().unwrap().ancestors().all().unwrap();
    let repo_path = repo.path().to_str().unwrap().to_string();

    let mut rewrite_cache = repo
        .diff_resource_cache(gix::diff::blob::pipeline::Mode::ToGit, Default::default())
        .unwrap();

    let mut diff_cache = rewrite_cache.clone();
    let mut rows: Vec<Row> = vec![];

    let select_insertions_or_deletions = selected_columns.contains(&"insertions".to_string())
        || selected_columns.contains(&"deletions".to_string());

    for commit_info in revwalk {
        let commit_info = commit_info.unwrap();
        let commit = commit_info.id().object().unwrap().into_commit();
        let commit_ref = commit.decode().unwrap();
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());

        for column_name in selected_columns {
            if column_name == "commit_id" {
                values.push(Box::new(TextValue {
                    value: commit_info.id.to_string(),
                }));
                continue;
            }

            if column_name == "name" {
                values.push(Box::new(TextValue {
                    value: commit_ref.author().name.to_string(),
                }));
                continue;
            }

            if column_name == "email" {
                values.push(Box::new(TextValue {
                    value: commit_ref.author().email.to_string(),
                }));
                continue;
            }

            if column_name == "datetime" {
                let time_stamp = commit_info
                    .commit_time
                    .unwrap_or_else(|| commit_ref.time().seconds);
                values.push(Box::new(DateTimeValue { value: time_stamp }));
                continue;
            }

            if column_name == "repo" {
                values.push(Box::new(TextValue {
                    value: repo_path.to_string(),
                }));
                continue;
            }

            if column_name == "insertions"
                || column_name == "deletions"
                || column_name == "files_changed"
            {
                let current = commit.tree().unwrap();

                let previous = commit_info
                    .parent_ids()
                    .next()
                    .map(|id| id.object().unwrap().into_commit().tree().unwrap())
                    .unwrap_or_else(|| repo.empty_tree());
                rewrite_cache.clear_resource_cache();
                diff_cache.clear_resource_cache();

                let (mut insertions, mut deletions, mut files_changed) = (0, 0, 0);
                let _ = previous
                    .changes()
                    .unwrap()
                    .for_each_to_obtain_tree_with_cache(
                        &current,
                        &mut rewrite_cache,
                        |change| -> Result<_, gix::object::blob::diff::init::Error> {
                            files_changed += usize::from(change.entry_mode().is_no_tree());
                            if select_insertions_or_deletions {
                                if let Ok(mut platform) = change.diff(&mut diff_cache) {
                                    if let Ok(Some(counts)) = platform.line_counts() {
                                        deletions += counts.removals;
                                        insertions += counts.insertions;
                                    }
                                }
                            }
                            Ok(gix::object::tree::diff::Action::Continue)
                        },
                    );

                if column_name == "insertions" {
                    values.push(Box::new(IntValue {
                        value: insertions as i64,
                    }));
                    continue;
                }

                if column_name == "deletions" {
                    values.push(Box::new(IntValue {
                        value: deletions as i64,
                    }));
                    continue;
                }

                if column_name == "files_changed" {
                    values.push(Box::new(IntValue {
                        value: files_changed as i64,
                    }));
                    continue;
                }
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}

fn select_tags(repo: &gix::Repository, selected_columns: &[String]) -> Result<Vec<Row>, String> {
    let platform = repo.references().unwrap();
    let tag_names = platform.tags().unwrap();
    let repo_path = repo.path().to_str().unwrap().to_string();
    let mut rows: Vec<Row> = vec![];
    for tag_ref in tag_names.flatten() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());

        for column_name in selected_columns {
            if column_name == "name" {
                let tag_name = tag_ref
                    .name()
                    .category_and_short_name()
                    .map_or_else(String::default, |(_, short_name)| short_name.to_string());
                values.push(Box::new(TextValue {
                    value: tag_name.to_string(),
                }));
                continue;
            }

            if column_name == "repo" {
                values.push(Box::new(TextValue {
                    value: repo_path.to_string(),
                }));
                continue;
            }

            values.push(Box::new(NullValue));
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(rows)
}
