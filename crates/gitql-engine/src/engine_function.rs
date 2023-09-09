use gix::reference::Category;
use std::collections::HashMap;

use gitql_ast::expression::Expression;
use gitql_ast::expression::SymbolExpression;
use gitql_ast::object::GQLObject;
use gitql_ast::value::Value;

use crate::engine_evaluator::evaluate_expression;

pub fn select_gql_objects(
    repo: &gix::Repository,
    table: String,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Vec<GQLObject> {
    return match table.as_str() {
        "refs" => select_references(repo, fields_names, fields_values, alias_table),
        "commits" => select_commits(repo, fields_names, fields_values, alias_table),
        "branches" => select_branches(repo, fields_names, fields_values, alias_table),
        "diffs" => select_diffs(repo, fields_names, fields_values, alias_table),
        "tags" => select_tags(repo, fields_names, fields_values, alias_table),
        _ => select_values(repo, fields_names, fields_values, alias_table),
    };
}

fn select_references(
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Vec<GQLObject> {
    let repo_path = repo.path().to_str().unwrap().to_string();
    let mut gql_references: Vec<GQLObject> = Vec::new();
    let git_references = repo.references();
    if git_references.is_err() {
        return gql_references;
    }

    let references = git_references.ok().unwrap();
    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for reference_result in references.all().unwrap() {
        if reference_result.is_err() {
            break;
        }

        let reference = reference_result.ok().unwrap();
        let mut attributes: HashMap<String, Value> = HashMap::new();

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if !value.as_any().downcast_ref::<SymbolExpression>().is_some() {
                    let evaulated = evaluate_expression(value, &attributes);
                    let column_name = get_column_name(&alias_table, field_name);
                    if evaulated.is_err() {
                        println!("Error {}", evaulated.err().unwrap());
                        continue;
                    }
                    attributes.insert(column_name, evaulated.ok().unwrap());
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
                let column_name = get_column_name(&alias_table, &"name".to_string());
                attributes.insert(column_name, Value::Text(name));
                continue;
            }

            if field_name == "full_name" {
                let full_name = reference.name().as_bstr().to_string();
                let column_name = get_column_name(&alias_table, &"full_name".to_string());
                attributes.insert(column_name, Value::Text(full_name));
                continue;
            }

            if field_name == "type" {
                let category = reference.name().category();
                let column_name = get_column_name(&alias_table, &"type".to_string());
                if category.map_or(false, |cat| cat == Category::LocalBranch) {
                    attributes.insert(column_name, Value::Text("branch".to_owned()));
                } else if category.map_or(false, |cat| cat == Category::RemoteBranch) {
                    attributes.insert(column_name, Value::Text("remote".to_owned()));
                } else if category.map_or(false, |cat| cat == Category::Tag) {
                    attributes.insert(column_name, Value::Text("tag".to_owned()));
                } else if category.map_or(false, |cat| cat == Category::Note) {
                    attributes.insert(column_name, Value::Text("note".to_owned()));
                } else {
                    attributes.insert(column_name, Value::Text("other".to_owned()));
                }
                continue;
            }

            if field_name == "repo" {
                let column_name = get_column_name(&alias_table, &"repo".to_string());
                attributes.insert(column_name, Value::Text(repo_path.to_string()));
                continue;
            }
        }

        let gql_reference = GQLObject { attributes };
        gql_references.push(gql_reference);
    }

    return gql_references;
}

fn select_commits(
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Vec<GQLObject> {
    let repo_path = repo.path().to_str().unwrap().to_string();

    let mut commits: Vec<GQLObject> = Vec::new();
    let revwalk = repo.head_id().unwrap().ancestors().all().unwrap();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for commit_info in revwalk {
        let commit_info = commit_info.unwrap();
        let commit = repo.find_object(commit_info.id).unwrap().into_commit();
        let commit = commit.decode().unwrap();

        let mut attributes: HashMap<String, Value> = HashMap::new();

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if !value.as_any().downcast_ref::<SymbolExpression>().is_some() {
                    let evaulated = evaluate_expression(value, &attributes);
                    let column_name = get_column_name(&alias_table, field_name);
                    if evaulated.is_err() {
                        println!("Error {}", evaulated.err().unwrap());
                        continue;
                    }
                    attributes.insert(column_name, evaulated.ok().unwrap());
                    continue;
                }
            }

            if field_name == "commit_id" {
                let commit_id = Value::Text(commit_info.id.to_string());
                let column_name = get_column_name(&alias_table, &"commit_id".to_string());
                attributes.insert(column_name, commit_id);
                continue;
            }

            if field_name == "name" {
                let name = commit.author().name.to_string();
                let column_name = get_column_name(&alias_table, &"name".to_string());
                attributes.insert(column_name, Value::Text(name));
                continue;
            }

            if field_name == "email" {
                let email = commit.author().email.to_string();
                let column_name = get_column_name(&alias_table, &"email".to_string());
                attributes.insert(column_name, Value::Text(email));
                continue;
            }

            if field_name == "title" {
                let summary = Value::Text(commit.message().summary().to_string());
                let column_name = get_column_name(&alias_table, &"title".to_string());
                attributes.insert(column_name, summary);
                continue;
            }

            if field_name == "message" {
                let message = Value::Text(commit.message.to_string());
                let column_name = get_column_name(&alias_table, &"message".to_string());
                attributes.insert(column_name, message);
                continue;
            }

            if field_name == "time" {
                let column_name = get_column_name(&alias_table, &"time".to_string());
                let time_stamp = commit_info
                    .commit_time
                    .unwrap_or_else(|| commit.time().seconds);
                attributes.insert(column_name, Value::Date(time_stamp));
                continue;
            }

            if field_name == "repo" {
                let column_name = get_column_name(&alias_table, &"repo".to_string());
                attributes.insert(column_name, Value::Text(repo_path.to_string()));
                continue;
            }
        }

        let gql_commit = GQLObject { attributes };
        commits.push(gql_commit);
    }

    return commits;
}

fn select_diffs(
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Vec<GQLObject> {
    let repo = {
        let mut repo = repo.clone();
        repo.object_cache_size_if_unset(4 * 1024 * 1024);
        repo
    };
    let mut diffs: Vec<GQLObject> = Vec::new();
    let revwalk = repo.head_id().unwrap().ancestors().all().unwrap();

    let repo_path = repo.path().to_str().unwrap().to_string();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for commit_info in revwalk {
        let commit_info = commit_info.unwrap();
        let commit = commit_info.id().object().unwrap().into_commit();
        let mut attributes: HashMap<String, Value> = HashMap::new();

        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if !value.as_any().downcast_ref::<SymbolExpression>().is_some() {
                    let evaulated = evaluate_expression(value, &attributes);
                    let column_name = get_column_name(&alias_table, field_name);
                    if evaulated.is_err() {
                        println!("Error {}", evaulated.err().unwrap());
                        continue;
                    }
                    attributes.insert(column_name, evaulated.ok().unwrap());
                    continue;
                }
            }

            if field_name == "commit_id" {
                let column_name = get_column_name(&alias_table, &"commit_id".to_string());
                attributes.insert(column_name, Value::Text(commit_info.id.to_string()));
                continue;
            }

            if field_name == "name" {
                let name = commit.author().unwrap().name.to_string();
                let column_name = get_column_name(&alias_table, &"name".to_string());
                attributes.insert(column_name, Value::Text(name));
                continue;
            }

            if field_name == "email" {
                let email = commit.author().unwrap().email.to_string();
                let column_name = get_column_name(&alias_table, &"email".to_string());
                attributes.insert(column_name, Value::Text(email));
                continue;
            }

            if field_name == "repo" {
                let column_name = get_column_name(&alias_table, &"repo".to_string());
                attributes.insert(column_name, Value::Text(repo_path.to_string()));
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
                let (mut insertions, mut deletions, mut files_changed) = (0, 0, 0);
                previous
                    .changes()
                    .unwrap()
                    .for_each_to_obtain_tree(
                        &current,
                        |change| -> Result<_, gix::object::blob::diff::init::Error> {
                            files_changed += usize::from(change.event.entry_mode().is_no_tree());
                            if let Some(diff) = change.event.diff().transpose()? {
                                let counts = diff.line_counts();
                                deletions += counts.removals;
                                insertions += counts.insertions;
                            }
                            Ok(gix::object::tree::diff::Action::Continue)
                        },
                    )
                    .unwrap();

                if field_name == "insertions" {
                    let insertions = Value::Number(insertions as i64);
                    let column_name = get_column_name(&alias_table, &"insertions".to_string());
                    attributes.insert(column_name, insertions);
                    continue;
                }

                if field_name == "deletions" {
                    let deletations = Value::Number(deletions as i64);
                    let column_name = get_column_name(&alias_table, &"deletions".to_string());
                    attributes.insert(column_name, deletations);
                    continue;
                }

                if field_name == "files_changed" {
                    let file_changed = Value::Number(files_changed as i64);
                    let column_name = get_column_name(&alias_table, &"files_changed".to_string());
                    attributes.insert(column_name, file_changed);
                    continue;
                }
            }
        }

        let gql_diff = GQLObject { attributes };
        diffs.push(gql_diff);
    }

    return diffs;
}

fn select_branches(
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Vec<GQLObject> {
    let mut branches: Vec<GQLObject> = Vec::new();
    let platform = repo.references().unwrap();
    let local_branches = platform.local_branches().unwrap();
    let repo_path = repo.path().to_str().unwrap().to_string();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    let head_ref = repo.head_ref().unwrap().unwrap();

    for branch in local_branches {
        let branch = branch.unwrap();

        let mut attributes: HashMap<String, Value> = HashMap::new();
        for index in 0..names_len {
            let field_name = &fields_names[index as usize];

            if (index - padding) >= 0 {
                let value = &fields_values[(index - padding) as usize];
                if !value.as_any().downcast_ref::<SymbolExpression>().is_some() {
                    let evaulated = evaluate_expression(value, &attributes);
                    let column_name = get_column_name(&alias_table, field_name);
                    if evaulated.is_err() {
                        println!("Error {}", evaulated.err().unwrap());
                        continue;
                    }
                    attributes.insert(column_name, evaulated.ok().unwrap());
                    continue;
                }
            }

            if field_name == "name" {
                let branch_name = branch.name().as_bstr().to_string();
                let column_name = get_column_name(&alias_table, &"name".to_string());
                attributes.insert(column_name, Value::Text(branch_name));
                continue;
            }

            if field_name == "commit_count" {
                let revwalk = branch.id().ancestors().all().unwrap();
                let column_name = get_column_name(&alias_table, &"commit_count".to_string());
                attributes.insert(column_name, Value::Number(revwalk.count() as i64));
                continue;
            }

            if field_name == "is_head" {
                let column_name = get_column_name(&alias_table, &"is_head".to_string());
                attributes.insert(column_name, Value::Boolean(branch.inner == head_ref.inner));
                continue;
            }

            if field_name == "is_remote" {
                let column_name = get_column_name(&alias_table, &"is_remote".to_string());
                attributes.insert(
                    column_name,
                    Value::Boolean(
                        branch
                            .name()
                            .category()
                            .map_or(false, |cat| cat == Category::RemoteBranch),
                    ),
                );
                continue;
            }

            if field_name == "repo" {
                let column_name = get_column_name(&alias_table, &"repo".to_string());
                attributes.insert(column_name, Value::Text(repo_path.to_string()));
                continue;
            }
        }

        let gql_branch = GQLObject { attributes };
        branches.push(gql_branch);
    }

    return branches;
}

fn select_tags(
    repo: &gix::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Vec<GQLObject> {
    let mut tags: Vec<GQLObject> = Vec::new();
    let platform = repo.references().unwrap();
    let tag_names = platform.tags().unwrap();
    let repo_path = repo.path().to_str().unwrap().to_string();

    let names_len = fields_names.len() as i64;
    let values_len = fields_values.len() as i64;
    let padding = names_len - values_len;

    for tag_name in tag_names {
        match tag_name {
            Ok(tag_ref) => {
                let mut attributes: HashMap<String, Value> = HashMap::new();

                for index in 0..names_len {
                    let field_name = &fields_names[index as usize];
                    if (index - padding) >= 0 {
                        let value = &fields_values[(index - padding) as usize];

                        if !value.as_any().downcast_ref::<SymbolExpression>().is_some() {
                            let evaulated = evaluate_expression(value, &attributes);
                            let column_name = get_column_name(&alias_table, field_name);
                            if evaulated.is_err() {
                                println!("Error {}", evaulated.err().unwrap());
                                continue;
                            }
                            attributes.insert(column_name, evaulated.ok().unwrap());
                            continue;
                        }
                    }

                    if field_name == "name" {
                        let column_name = get_column_name(&alias_table, &"name".to_string());
                        attributes.insert(
                            column_name,
                            Value::Text(
                                tag_ref
                                    .name()
                                    .category_and_short_name()
                                    .map_or_else(String::default, |(_, short_name)| {
                                        short_name.to_string()
                                    }),
                            ),
                        );
                        continue;
                    }

                    if field_name == "repo" {
                        let column_name = get_column_name(&alias_table, &"repo".to_string());
                        attributes.insert(column_name, Value::Text(repo_path.to_string()));
                        continue;
                    }
                }

                let gql_tag = GQLObject { attributes };
                tags.push(gql_tag);
            }
            Err(_) => {}
        }
    }
    return tags;
}

fn select_values(
    _repo: &gix::Repository,
    fields_names: &Vec<String>,
    fields_values: &Vec<Box<dyn Expression>>,
    alias_table: &HashMap<String, String>,
) -> Vec<GQLObject> {
    let mut values: Vec<GQLObject> = Vec::new();
    let mut attributes: HashMap<String, Value> = HashMap::new();
    let len = fields_values.len();

    for index in 0..len {
        let field_name = &fields_names[index];
        let value = &fields_values[index];
        let evaulated = evaluate_expression(value, &attributes);
        let column_name = get_column_name(&alias_table, field_name);
        attributes.insert(column_name, evaulated.ok().unwrap());
    }

    let gql_object = GQLObject { attributes };
    values.push(gql_object);
    return values;
}

#[inline(always)]
pub fn get_column_name(alias_table: &HashMap<String, String>, name: &String) -> String {
    return alias_table
        .get(name)
        .unwrap_or(&name.to_string())
        .to_string();
}
