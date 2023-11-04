use std::collections::HashMap;

use gitql_ast::expression::Expression;
use gitql_ast::expression::SymbolExpression;
use gitql_ast::object::GQLObject;
use gitql_ast::value::Value;

use crate::engine_evaluator::evaluate_expression;

pub fn select_gql_objects(
    repo: &git2::Repository,
    table: String,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Result<Vec<GQLObject>, String> {
    match table.as_str() {
        "refs" => select_references(repo, fields_names, fields_values, alias_table),
        "commits" => select_commits(repo, fields_names, fields_values, alias_table),
        "branches" => select_branches(repo, fields_names, fields_values, alias_table),
        "diffs" => select_diffs(repo, fields_names, fields_values, alias_table),
        "tags" => select_tags(repo, fields_names, fields_values, alias_table),
        _ => select_values(repo, fields_names, fields_values, alias_table),
    }
}

fn select_references(
    repo: &git2::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Result<Vec<GQLObject>, String> {
    let repo_path = repo.path().to_str().unwrap().to_string();
    let mut gql_references: Vec<GQLObject> = Vec::new();
    let git_references = repo.references();
    if git_references.is_err() {
        return Ok(gql_references);
    }

    let references = git_references.ok().unwrap();
    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for reference_result in references {
        if reference_result.is_err() {
            break;
        }

        let reference = reference_result.ok().unwrap();
        let mut attributes: HashMap<String, Value> = HashMap::new();

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaulated = evaluate_expression(value, &attributes)?;
                    let column_name = get_column_name(alias_table, field_name);
                    attributes.insert(column_name, evaulated);
                    continue;
                }
            }

            if field_name == "name" {
                let name = reference.shorthand().unwrap_or("").to_string();
                let column_name = get_column_name(alias_table, &"name".to_string());
                attributes.insert(column_name, Value::Text(name));
                continue;
            }

            if field_name == "full_name" {
                let full_name = reference.name().unwrap_or("").to_string();
                let column_name = get_column_name(alias_table, &"full_name".to_string());
                attributes.insert(column_name, Value::Text(full_name));
                continue;
            }

            if field_name == "type" {
                let column_name = get_column_name(alias_table, &"type".to_string());
                if reference.is_branch() {
                    attributes.insert(column_name, Value::Text("branch".to_owned()));
                } else if reference.is_remote() {
                    attributes.insert(column_name, Value::Text("remote".to_owned()));
                } else if reference.is_tag() {
                    attributes.insert(column_name, Value::Text("tag".to_owned()));
                } else if reference.is_note() {
                    attributes.insert(column_name, Value::Text("note".to_owned()));
                } else {
                    attributes.insert(column_name, Value::Text("other".to_owned()));
                }
                continue;
            }

            if field_name == "repo" {
                let column_name = get_column_name(alias_table, &"repo".to_string());
                attributes.insert(column_name, Value::Text(repo_path.to_string()));
                continue;
            }
        }

        let gql_reference = GQLObject { attributes };
        gql_references.push(gql_reference);
    }

    Ok(gql_references)
}

fn select_commits(
    repo: &git2::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Result<Vec<GQLObject>, String> {
    let repo_path = repo.path().to_str().unwrap().to_string();

    let mut commits: Vec<GQLObject> = Vec::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id.unwrap()).unwrap();

        let mut attributes: HashMap<String, Value> = HashMap::new();

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaulated = evaluate_expression(value, &attributes)?;
                    let column_name = get_column_name(alias_table, field_name);
                    attributes.insert(column_name, evaulated);
                    continue;
                }
            }

            if field_name == "commit_id" {
                let commit_id = Value::Text(commit.id().to_string());
                let column_name = get_column_name(alias_table, &"commit_id".to_string());
                attributes.insert(column_name, commit_id);
                continue;
            }

            if field_name == "name" {
                let name = commit.author().name().unwrap_or("").to_string();
                let column_name = get_column_name(alias_table, &"name".to_string());
                attributes.insert(column_name, Value::Text(name));
                continue;
            }

            if field_name == "email" {
                let email = commit.author().email().unwrap_or("").to_string();
                let column_name = get_column_name(alias_table, &"email".to_string());
                attributes.insert(column_name, Value::Text(email));
                continue;
            }

            if field_name == "title" {
                let summary = Value::Text(commit.summary().unwrap().to_string());
                let column_name = get_column_name(alias_table, &"title".to_string());
                attributes.insert(column_name, summary);
                continue;
            }

            if field_name == "message" {
                let message = Value::Text(commit.message().unwrap_or("").to_string());
                let column_name = get_column_name(alias_table, &"message".to_string());
                attributes.insert(column_name, message);
                continue;
            }

            if field_name == "time" {
                let column_name = get_column_name(alias_table, &"time".to_string());
                let time_stamp = commit.time().seconds();
                attributes.insert(column_name, Value::DateTime(time_stamp));
                continue;
            }

            if field_name == "repo" {
                let column_name = get_column_name(alias_table, &"repo".to_string());
                attributes.insert(column_name, Value::Text(repo_path.to_string()));
                continue;
            }
        }

        let gql_commit = GQLObject { attributes };
        commits.push(gql_commit);
    }

    Ok(commits)
}

fn select_diffs(
    repo: &git2::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Result<Vec<GQLObject>, String> {
    let mut diffs: Vec<GQLObject> = Vec::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let repo_path = repo.path().to_str().unwrap().to_string();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id.unwrap()).unwrap();
        let mut attributes: HashMap<String, Value> = HashMap::new();

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaulated = evaluate_expression(value, &attributes)?;
                    let column_name = get_column_name(alias_table, field_name);
                    attributes.insert(column_name, evaulated);
                    continue;
                }
            }

            if field_name == "commit_id" {
                let column_name = get_column_name(alias_table, &"commit_id".to_string());
                attributes.insert(column_name, Value::Text(commit.id().to_string()));
                continue;
            }

            if field_name == "name" {
                let name = commit.author().name().unwrap_or("").to_string();
                let column_name = get_column_name(alias_table, &"name".to_string());
                attributes.insert(column_name, Value::Text(name));
                continue;
            }

            if field_name == "email" {
                let email = commit.author().email().unwrap_or("").to_string();
                let column_name = get_column_name(alias_table, &"email".to_string());
                attributes.insert(column_name, Value::Text(email));
                continue;
            }

            if field_name == "repo" {
                let column_name = get_column_name(alias_table, &"repo".to_string());
                attributes.insert(column_name, Value::Text(repo_path.to_string()));
                continue;
            }

            if field_name == "insertions"
                || field_name == "deletions"
                || field_name == "files_changed"
            {
                let diff = if commit.parents().len() > 0 {
                    repo.diff_tree_to_tree(
                        Some(&commit.parent(0).unwrap().tree().unwrap()),
                        Some(&commit.tree().unwrap()),
                        None,
                    )
                } else {
                    repo.diff_tree_to_tree(None, Some(&commit.tree().unwrap()), None)
                };

                let diff_status = diff.unwrap().stats().unwrap();

                if field_name == "insertions" {
                    let insertions = Value::Integer(diff_status.insertions() as i64);
                    let column_name = get_column_name(alias_table, &"insertions".to_string());
                    attributes.insert(column_name, insertions);
                    continue;
                }

                if field_name == "deletions" {
                    let deletations = Value::Integer(diff_status.deletions() as i64);
                    let column_name = get_column_name(alias_table, &"deletions".to_string());
                    attributes.insert(column_name, deletations);
                    continue;
                }

                if field_name == "files_changed" {
                    let file_changed = Value::Integer(diff_status.files_changed() as i64);
                    let column_name = get_column_name(alias_table, &"files_changed".to_string());
                    attributes.insert(column_name, file_changed);
                    continue;
                }
            }
        }

        let gql_diff = GQLObject { attributes };
        diffs.push(gql_diff);
    }

    Ok(diffs)
}

fn select_branches(
    repo: &git2::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Result<Vec<GQLObject>, String> {
    let mut branches: Vec<GQLObject> = Vec::new();
    let local_and_remote_branches = repo.branches(None).unwrap();
    let repo_path = repo.path().to_str().unwrap().to_string();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for branch in local_and_remote_branches {
        let (branch, _) = branch.unwrap();

        let mut attributes: HashMap<String, Value> = HashMap::new();
        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaulated = evaluate_expression(value, &attributes)?;
                    let column_name = get_column_name(alias_table, field_name);
                    attributes.insert(column_name, evaulated);
                    continue;
                }
            }

            if field_name == "name" {
                let branch_name = branch.name().unwrap().unwrap_or("").to_string();
                let column_name = get_column_name(alias_table, &"name".to_string());
                attributes.insert(column_name, Value::Text(branch_name));
                continue;
            }

            if field_name == "commit_count" {
                let branch_ref = branch.get().peel_to_commit().unwrap();
                let mut revwalk = repo.revwalk().unwrap();
                let _ = revwalk.push(branch_ref.id());
                let column_name = get_column_name(alias_table, &"commit_count".to_string());
                attributes.insert(column_name, Value::Integer(revwalk.count() as i64));
                continue;
            }

            if field_name == "is_head" {
                let column_name = get_column_name(alias_table, &"is_head".to_string());
                attributes.insert(column_name, Value::Boolean(branch.is_head()));
                continue;
            }

            if field_name == "is_remote" {
                let column_name = get_column_name(alias_table, &"is_remote".to_string());
                attributes.insert(column_name, Value::Boolean(branch.get().is_remote()));
                continue;
            }

            if field_name == "repo" {
                let column_name = get_column_name(alias_table, &"repo".to_string());
                attributes.insert(column_name, Value::Text(repo_path.to_string()));
                continue;
            }
        }

        let gql_branch = GQLObject { attributes };
        branches.push(gql_branch);
    }

    Ok(branches)
}

fn select_tags(
    repo: &git2::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Result<Vec<GQLObject>, String> {
    let mut tags: Vec<GQLObject> = Vec::new();
    let tag_names = repo.tag_names(None).unwrap();
    let repo_path = repo.path().to_str().unwrap().to_string();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for tag_name in tag_names.iter().flatten() {
        let mut attributes: HashMap<String, Value> = HashMap::new();

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];
            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];

                if value.as_any().downcast_ref::<SymbolExpression>().is_none() {
                    let evaulated = evaluate_expression(value, &attributes)?;
                    let column_name = get_column_name(alias_table, field_name);
                    attributes.insert(column_name, evaulated);
                    continue;
                }
            }

            if field_name == "name" {
                let column_name = get_column_name(alias_table, &"name".to_string());
                attributes.insert(column_name, Value::Text(tag_name.to_string()));
                continue;
            }

            if field_name == "repo" {
                let column_name = get_column_name(alias_table, &"repo".to_string());
                attributes.insert(column_name, Value::Text(repo_path.to_string()));
                continue;
            }
        }

        let gql_tag = GQLObject { attributes };
        tags.push(gql_tag);
    }

    Ok(tags)
}

fn select_values(
    _repo: &git2::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Result<Vec<GQLObject>, String> {
    let mut values: Vec<GQLObject> = Vec::new();
    let mut attributes: HashMap<String, Value> = HashMap::new();
    let len = fields_values.len();

    for index in 0..len {
        let field_name = &fields_names[index];
        let value = &fields_values[index];
        let evaulated = evaluate_expression(value, &attributes)?;
        let column_name = get_column_name(alias_table, field_name);
        attributes.insert(column_name, evaulated);
    }

    let gql_object = GQLObject { attributes };
    values.push(gql_object);
    Ok(values)
}

#[inline(always)]
pub fn get_column_name(alias_table: &HashMap<String, String>, name: &String) -> String {
    alias_table
        .get(name)
        .unwrap_or(&name.to_string())
        .to_string()
}
