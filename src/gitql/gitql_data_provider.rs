use std::convert::Infallible;

use gitql_core::object::Row;
use gitql_core::values::base::Value;
use gitql_core::values::boolean::BoolValue;
use gitql_core::values::datetime::DateTimeValue;
use gitql_core::values::integer::IntValue;
use gitql_core::values::null::NullValue;
use gitql_core::values::text::TextValue;
use gitql_engine::data_provider::DataProvider;

use gix::diff::blob::pipeline::Mode;
use gix::object::tree::diff::Change;
use gix::refs::Category;

use super::values::diff_changes::DiffChange;
use super::values::diff_changes::DiffChangeKind;
use super::values::diff_changes::DiffChangesValue;

pub struct GitQLDataProvider {
    pub repos: Vec<gix::Repository>,
}

impl GitQLDataProvider {
    pub fn new(repos: Vec<gix::Repository>) -> Self {
        Self { repos }
    }
}

impl DataProvider for GitQLDataProvider {
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

    let repo_path = repo.path().to_str().unwrap();
    let references = git_references.ok().unwrap();
    let mut rows: Vec<Row> = vec![];

    for reference in references.all().unwrap().flatten() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());
        for field_name in selected_columns {
            if field_name == "name" {
                let name = reference.name().shorten().to_string();
                values.push(Box::new(TextValue { value: name }));
                continue;
            }

            if field_name == "full_name" {
                let full_name = reference.name().as_bstr().to_string();
                values.push(Box::new(TextValue { value: full_name }));
                continue;
            }

            if field_name == "type" {
                let category = if let Some(category) = reference.name().category() {
                    format!("{:?}", category)
                } else {
                    "Other".to_string()
                };
                values.push(Box::new(TextValue { value: category }));
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

    let repo_path = repo.path().to_str().unwrap();
    let walker = head_id.unwrap().ancestors().all().unwrap();
    let mut rows: Vec<Row> = vec![];

    for commit_info in walker {
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

    let repo_path = repo.path().to_str().unwrap();
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
                    if let Ok(walker) = id.ancestors().all() {
                        walker.count() as i64
                    } else {
                        0
                    }
                } else {
                    0
                };

                values.push(Box::new(IntValue {
                    value: commit_count,
                }));
                continue;
            }

            if column_name == "updated" {
                if let Ok(top_commit_id) = branch.peel_to_id_in_place() {
                    let walker = top_commit_id.ancestors().all().unwrap();
                    if let Some(commit_info) = walker.into_iter().next() {
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

    let mut rewrite_cache = repo
        .diff_resource_cache(Mode::ToGit, Default::default())
        .unwrap();

    let mut diff_cache = rewrite_cache.clone();

    let should_calculate_diffs = selected_columns.iter().any(|col| {
        col == "insertions" || col == "removals" || col == "files_changed" || col == "diff_changes"
    });

    let repo_path = repo.path().to_str().unwrap();
    let walker = repo.head_id().unwrap().ancestors().all().unwrap();
    let commits_info = walker.filter_map(Result::ok);

    let mut rows: Vec<Row> = vec![];

    for commit_info in commits_info.into_iter() {
        let commit = commit_info.id().object().unwrap().into_commit();
        let commit_ref = commit.decode().unwrap();
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());

        // Calculate the diff between two commits take time, and  should calculated once per commit
        let (mut insertions, mut removals, mut files_changed) = (0, 0, 0);
        let mut diff_changes: Vec<DiffChange> = vec![];

        if should_calculate_diffs {
            if let Some(parent) = commit_info
                .parent_ids()
                .next()
                .map(|id| id.object().unwrap().into_commit().tree().unwrap())
            {
                let current = commit.tree().unwrap();
                rewrite_cache.clear_resource_cache_keep_allocation();
                diff_cache.clear_resource_cache_keep_allocation();

                if let Ok(mut changes) = current.changes() {
                    let _ = changes.for_each_to_obtain_tree_with_cache(
                        &parent,
                        &mut rewrite_cache,
                        |change| {
                            files_changed += usize::from(change.entry_mode().is_no_tree());
                            match change {
                                Change::Addition {
                                    location,
                                    entry_mode: _,
                                    relation: _,
                                    id,
                                } => {
                                    let mut diff_change = DiffChange::new(DiffChangeKind::Addition);
                                    diff_change.location = location.to_string();
                                    if let Ok(object) = repo.find_object(id) {
                                        if let Ok(blob) = object.try_into_blob() {
                                            diff_change.content = blob.data.clone();
                                        }
                                    }

                                    if let Ok(mut platform) = change.diff(&mut diff_cache) {
                                        if let Ok(Some(counts)) = platform.line_counts() {
                                            diff_change.insertions += counts.insertions;
                                            diff_change.removals += counts.removals;
                                        }
                                    }

                                    insertions += diff_change.insertions;
                                    removals += diff_change.removals;
                                    diff_changes.push(diff_change);
                                }
                                Change::Deletion {
                                    location,
                                    entry_mode: _,
                                    relation: _,
                                    id,
                                } => {
                                    let mut diff_change = DiffChange::new(DiffChangeKind::Deletion);
                                    diff_change.location = location.to_string();
                                    if let Ok(object) = repo.find_object(id) {
                                        if let Ok(blob) = object.try_into_blob() {
                                            diff_change.content = blob.data.clone();
                                        }
                                    }

                                    if let Ok(mut platform) = change.diff(&mut diff_cache) {
                                        if let Ok(Some(counts)) = platform.line_counts() {
                                            diff_change.insertions += counts.insertions;
                                            diff_change.removals += counts.removals;
                                        }
                                    }

                                    insertions += diff_change.insertions;
                                    removals += diff_change.removals;
                                    diff_changes.push(diff_change);
                                }
                                Change::Modification {
                                    location,
                                    previous_entry_mode: _,
                                    previous_id: _,
                                    entry_mode: _,
                                    id,
                                } => {
                                    let mut diff_change =
                                        DiffChange::new(DiffChangeKind::Modification);
                                    diff_change.location = location.to_string();
                                    if let Ok(object) = repo.find_object(id) {
                                        if let Ok(blob) = object.try_into_blob() {
                                            diff_change.content = blob.data.clone();
                                        }
                                    }

                                    if let Ok(mut platform) = change.diff(&mut diff_cache) {
                                        if let Ok(Some(counts)) = platform.line_counts() {
                                            diff_change.insertions += counts.insertions;
                                            diff_change.removals += counts.removals;
                                        }
                                    }

                                    insertions += diff_change.insertions;
                                    removals += diff_change.removals;
                                    diff_changes.push(diff_change);
                                }
                                Change::Rewrite {
                                    source_location: _,
                                    source_relation: _,
                                    source_entry_mode: _,
                                    source_id: _,
                                    diff: _,
                                    entry_mode: _,
                                    location,
                                    id,
                                    relation: _,
                                    copy: _,
                                } => {
                                    let mut diff_change = DiffChange::new(DiffChangeKind::Rewrite);
                                    diff_change.location = location.to_string();
                                    if let Ok(object) = repo.find_object(id) {
                                        if let Ok(blob) = object.try_into_blob() {
                                            diff_change.content = blob.data.clone();
                                        }
                                    }

                                    if let Ok(mut platform) = change.diff(&mut diff_cache) {
                                        if let Ok(Some(counts)) = platform.line_counts() {
                                            diff_change.insertions += counts.insertions;
                                            diff_change.removals += counts.removals;
                                        }
                                    }

                                    insertions += diff_change.insertions;
                                    removals += diff_change.removals;
                                    diff_changes.push(diff_change);
                                }
                            }
                            Ok::<_, Infallible>(Default::default())
                        },
                    );
                }
            }
        }

        for column_name in selected_columns {
            if column_name == "commit_id" {
                let value = commit_info.id.to_string();
                values.push(Box::new(TextValue { value }));
                continue;
            }

            if column_name == "author_name" {
                let value = commit_ref.author().name.to_string();
                values.push(Box::new(TextValue { value }));
                continue;
            }

            if column_name == "author_email" {
                let value = commit_ref.author().email.to_string();
                values.push(Box::new(TextValue { value }));
                continue;
            }

            if column_name == "datetime" {
                let time_stamp = commit_info
                    .commit_time
                    .unwrap_or_else(|| commit_ref.time().seconds);
                values.push(Box::new(DateTimeValue { value: time_stamp }));
                continue;
            }

            if column_name == "insertions" {
                let value = insertions as i64;
                values.push(Box::new(IntValue { value }));
                continue;
            }

            if column_name == "removals" {
                let value = removals as i64;
                values.push(Box::new(IntValue { value }));
                continue;
            }

            if column_name == "files_changed" {
                let value = files_changed as i64;
                values.push(Box::new(IntValue { value }));
                continue;
            }

            if column_name == "diff_changes" {
                let value = DiffChangesValue {
                    changes: diff_changes.to_owned(),
                };
                values.push(Box::new(value));
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

fn select_tags(repo: &gix::Repository, selected_columns: &[String]) -> Result<Vec<Row>, String> {
    let platform = repo.references().unwrap();
    let tag_names = platform.tags().unwrap();
    let repo_path = repo.path().to_str().unwrap();
    let mut rows: Vec<Row> = vec![];
    for tag_ref in tag_names.flatten() {
        let mut values: Vec<Box<dyn Value>> = Vec::with_capacity(selected_columns.len());

        for column_name in selected_columns {
            if column_name == "name" {
                let tag_name = tag_ref
                    .name()
                    .category_and_short_name()
                    .map_or_else(String::default, |(_, short_name)| short_name.to_string());
                values.push(Box::new(TextValue { value: tag_name }));
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
