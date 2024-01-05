use gitql_ast::environment::Environment;
use gitql_ast::object::Group;
use gitql_ast::object::Row;
use gix::refs::Category;
use std::collections::HashMap;

use gitql_ast::expression::Expression;
use gitql_ast::expression::SymbolExpression;
use gitql_ast::value::Value;

use crate::engine_evaluator::evaluate_expression;

pub fn select_gql_objects(
    env: &mut Environment,
    repo: &gix::Repository,
    table: String,
    fields_names: &Vec<String>,
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    match table.as_str() {
        "refs" => select_references(env, repo, fields_names, titles, fields_values),
        "commits" => select_commits(env, repo, fields_names, titles, fields_values),
        "branches" => select_branches(env, repo, fields_names, titles, fields_values),
        "diffs" => select_diffs(env, repo, fields_names, titles, fields_values),
        "tags" => select_tags(env, repo, fields_names, titles, fields_values),
        _ => select_values(env, titles, fields_values),
    }
}

fn select_references(
    env: &mut Environment,
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    let repo_path = repo.path().to_str().unwrap().to_string();

    let mut rows: Vec<Row> = vec![];
    let git_references = repo.references();
    if git_references.is_err() {
        return Ok(Group { rows });
    }

    let references = git_references.ok().unwrap();
    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for reference in references.all().unwrap().flatten() {
        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            if field_name == "name" {
                let name = reference
                    .name()
                    .category_and_short_name()
                    .map(|(_, sn)| sn)
                    .unwrap_or("".into())
                    .to_string();
                values.push(Value::Text(name));
                continue;
            }

            if field_name == "full_name" {
                let full_name = reference.name().as_bstr().to_string();
                values.push(Value::Text(full_name));
                continue;
            }

            if field_name == "type" {
                let category = reference.name().category();
                if category.map_or(false, |cat| cat == Category::LocalBranch) {
                    values.push(Value::Text("branch".to_owned()));
                } else if category.map_or(false, |cat| cat == Category::RemoteBranch) {
                    values.push(Value::Text("remote".to_owned()));
                } else if category.map_or(false, |cat| cat == Category::Tag) {
                    values.push(Value::Text("tag".to_owned()));
                } else if category.map_or(false, |cat| cat == Category::Note) {
                    values.push(Value::Text("note".to_owned()));
                } else {
                    values.push(Value::Text("other".to_owned()));
                }
                continue;
            }

            if field_name == "repo" {
                values.push(Value::Text(repo_path.to_string()));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(Group { rows })
}

fn select_commits(
    env: &mut Environment,
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    let repo_path = repo.path().to_str().unwrap().to_string();

    let mut rows: Vec<Row> = vec![];
    let head_id = repo.head_id();
    if head_id.is_err() {
        return Ok(Group { rows });
    }

    let revwalk = head_id.unwrap().ancestors().all().unwrap();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for commit_info in revwalk {
        let commit_info = commit_info.unwrap();
        let commit = repo.find_object(commit_info.id).unwrap().into_commit();
        let commit = commit.decode().unwrap();

        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            if field_name == "commit_id" {
                let commit_id = Value::Text(commit_info.id.to_string());
                values.push(commit_id);
                continue;
            }

            if field_name == "name" {
                let name = commit.author().name.to_string();
                values.push(Value::Text(name));
                continue;
            }

            if field_name == "email" {
                let email = commit.author().email.to_string();
                values.push(Value::Text(email));
                continue;
            }

            if field_name == "title" {
                let summary = Value::Text(commit.message().summary().to_string());
                values.push(summary);
                continue;
            }

            if field_name == "message" {
                let message = Value::Text(commit.message.to_string());
                values.push(message);
                continue;
            }

            if field_name == "datetime" {
                let time_stamp = commit_info
                    .commit_time
                    .unwrap_or_else(|| commit.time().seconds);
                values.push(Value::DateTime(time_stamp));
                continue;
            }

            if field_name == "repo" {
                values.push(Value::Text(repo_path.to_string()));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(Group { rows })
}

fn select_branches(
    env: &mut Environment,
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    let mut rows: Vec<Row> = vec![];

    let repo_path = repo.path().to_str().unwrap().to_string();
    let platform = repo.references().unwrap();
    let local_branches = platform.local_branches().unwrap();
    let remote_branches = platform.remote_branches().unwrap();
    let local_and_remote_branches = local_branches.chain(remote_branches);
    let head_ref_result = repo.head_ref();
    if head_ref_result.is_err() {
        return Ok(Group { rows });
    }

    let head_ref_option = head_ref_result.unwrap();
    if head_ref_option.is_none() {
        return Ok(Group { rows });
    }

    let head_ref = head_ref_option.unwrap();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for branch in local_and_remote_branches.flatten() {
        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            if field_name == "name" {
                let branch_name = branch.name().as_bstr().to_string();
                values.push(Value::Text(branch_name));
                continue;
            }

            if field_name == "commit_count" {
                let commit_count = if let Some(id) = branch.try_id() {
                    if let Ok(revwalk) = id.ancestors().all() {
                        revwalk.count() as i64
                    } else {
                        -1
                    }
                } else {
                    -1
                };
                values.push(Value::Integer(commit_count));
                continue;
            }

            if field_name == "is_head" {
                values.push(Value::Boolean(branch.inner == head_ref.inner));
                continue;
            }

            if field_name == "is_remote" {
                let is_remote = branch
                    .name()
                    .category()
                    .map_or(false, |cat| cat == Category::RemoteBranch);
                values.push(Value::Boolean(is_remote));
                continue;
            }

            if field_name == "repo" {
                values.push(Value::Text(repo_path.to_string()));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(Group { rows })
}

fn select_diffs(
    env: &mut Environment,
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    let repo = {
        let mut repo = repo.clone();
        repo.object_cache_size_if_unset(4 * 1024 * 1024);
        repo
    };

    let mut rows: Vec<Row> = vec![];
    let revwalk = repo.head_id().unwrap().ancestors().all().unwrap();
    let repo_path = repo.path().to_str().unwrap().to_string();

    let mut rewrite_cache = repo
        .diff_resource_cache(gix::diff::blob::pipeline::Mode::ToGit, Default::default())
        .unwrap();
    let mut diff_cache = rewrite_cache.clone();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for commit_info in revwalk {
        let commit_info = commit_info.unwrap();
        let commit = commit_info.id().object().unwrap().into_commit();

        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            if field_name == "commit_id" {
                values.push(Value::Text(commit_info.id.to_string()));
                continue;
            }

            if field_name == "name" {
                let name = commit.author().unwrap().name.to_string();
                values.push(Value::Text(name));
                continue;
            }

            if field_name == "email" {
                let email = commit.author().unwrap().email.to_string();
                values.push(Value::Text(email));
                continue;
            }

            if field_name == "repo" {
                values.push(Value::Text(repo_path.to_string()));
                continue;
            }

            if field_name == "insertions"
                || field_name == "deletions"
                || field_name == "files_changed"
            {
                let current = commit.tree().unwrap();
                let previous = commit_info
                    .parent_ids()
                    .next()
                    .map(|id| id.object().unwrap().into_commit().tree().unwrap())
                    .unwrap_or_else(|| repo.empty_tree());

                let select_insertions_or_deletions =
                    field_name == "insertions" || field_name == "deletions";

                rewrite_cache.clear_resource_cache();
                diff_cache.clear_resource_cache();

                let (mut insertions, mut deletions, mut files_changed) = (0, 0, 0);

                previous
                    .changes()
                    .unwrap()
                    .for_each_to_obtain_tree_with_cache(
                        &current,
                        &mut rewrite_cache,
                        |change| -> Result<_, gix::object::blob::diff::init::Error> {
                            files_changed += usize::from(change.event.entry_mode().is_no_tree());
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
                    )
                    .unwrap();

                if field_name == "insertions" {
                    values.push(Value::Integer(insertions as i64));
                    continue;
                }

                if field_name == "deletions" {
                    values.push(Value::Integer(deletions as i64));
                    continue;
                }

                if field_name == "files_changed" {
                    values.push(Value::Integer(files_changed as i64));
                    continue;
                }
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(Group { rows })
}

fn select_tags(
    env: &mut Environment,
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    let platform = repo.references().unwrap();
    let tag_names = platform.tags().unwrap();
    let repo_path = repo.path().to_str().unwrap().to_string();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    let mut rows: Vec<Row> = vec![];

    for tag_ref in tag_names.flatten() {
        let mut values: Vec<Value> = Vec::with_capacity(fields_names.len());

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];
            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];

                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaluated = evaluate_expression(env, value, titles, &values)?;
                    values.push(evaluated);
                    continue;
                }
            }

            if field_name == "name" {
                let tag_name = tag_ref
                    .name()
                    .category_and_short_name()
                    .map_or_else(String::default, |(_, short_name)| short_name.to_string());
                values.push(Value::Text(tag_name.to_string()));
                continue;
            }

            if field_name == "repo" {
                values.push(Value::Text(repo_path.to_string()));
                continue;
            }

            values.push(Value::Null);
        }

        let row = Row { values };
        rows.push(row);
    }

    Ok(Group { rows })
}

fn select_values(
    env: &mut Environment,
    titles: &[String],
    fields_values: &[Box<dyn Expression>],
) -> Result<Group, String> {
    let mut group = Group { rows: vec![] };
    let mut values = Vec::with_capacity(fields_values.len());

    for value in fields_values.iter() {
        let evaluated = evaluate_expression(env, value, titles, &values)?;
        values.push(evaluated);
    }

    group.rows.push(Row { values });
    Ok(group)
}

#[inline(always)]
pub fn get_column_name(alias_table: &HashMap<String, String>, name: &str) -> String {
    alias_table
        .get(name)
        .unwrap_or(&name.to_string())
        .to_string()
}
