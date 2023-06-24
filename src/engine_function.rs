use crate::object;

use std::collections::HashMap;

pub fn select_gql_objects(
    repo: &git2::Repository,
    table: String,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<object::GQLObject> {
    return match table.as_str() {
        "refs" => select_references(repo, fields, alias_table),
        "commits" => select_commits(repo, fields, alias_table),
        "branches" => select_branches(repo, fields, alias_table),
        "tags" => select_tags(repo, fields, alias_table),
        _ => vec![],
    };
}

fn select_references(
    repo: &git2::Repository,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<object::GQLObject> {
    let mut gql_references: Vec<object::GQLObject> = Vec::new();
    let git_references = repo.references();
    if git_references.is_err() {
        return gql_references;
    }

    let is_limit_fields_empty = fields.is_empty();
    let references = git_references.ok().unwrap();

    for reference_result in references {
        if reference_result.is_err() {
            break;
        }

        let reference = reference_result.ok().unwrap();
        let mut attributes: HashMap<String, String> = HashMap::new();

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            let key = if alias_table.contains_key("name") {
                alias_table.get("name").unwrap()
            } else {
                "name"
            }
            .to_string();

            attributes.insert(key, reference.shorthand().unwrap().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("full_name")) {
            let key = if alias_table.contains_key("full_name") {
                alias_table.get("full_name").unwrap()
            } else {
                "full_name"
            }
            .to_string();

            attributes.insert(key, reference.name().unwrap().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("type")) {
            let key = if alias_table.contains_key("type") {
                alias_table.get("type").unwrap()
            } else {
                "type"
            }
            .to_string();

            if reference.is_branch() {
                attributes.insert(key, "branch".to_owned());
            } else if reference.is_remote() {
                attributes.insert(key, "remote".to_owned());
            } else if reference.is_tag() {
                attributes.insert(key, "tag".to_owned());
            } else if reference.is_note() {
                attributes.insert(key, "note".to_owned());
            } else {
                attributes.insert(key, "other".to_owned());
            }
        }

        let gql_reference = object::GQLObject { attributes };
        gql_references.push(gql_reference);
    }

    return gql_references;
}

fn select_commits(
    repo: &git2::Repository,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<object::GQLObject> {
    let mut commits: Vec<object::GQLObject> = Vec::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let is_limit_fields_empty = fields.is_empty();

    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id.unwrap()).unwrap();

        let mut attributes: HashMap<String, String> = HashMap::new();

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            let key = if alias_table.contains_key("name") {
                alias_table.get("name").unwrap()
            } else {
                "name"
            }
            .to_string();

            attributes.insert(key, commit.author().name().unwrap().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("email")) {
            let key = if alias_table.contains_key("email") {
                alias_table.get("email").unwrap()
            } else {
                "email"
            }
            .to_string();

            attributes.insert(key, commit.author().email().unwrap().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("title")) {
            let key = if alias_table.contains_key("title") {
                alias_table.get("title").unwrap()
            } else {
                "title"
            }
            .to_string();

            attributes.insert(key, commit.summary().unwrap().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("message")) {
            let key = if alias_table.contains_key("message") {
                alias_table.get("message").unwrap()
            } else {
                "message"
            }
            .to_string();

            attributes.insert(key, commit.message().unwrap().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("time")) {
            let key = if alias_table.contains_key("time") {
                alias_table.get("time").unwrap()
            } else {
                "time"
            }
            .to_string();

            attributes.insert(key, commit.time().seconds().to_string());
        }

        let gql_commit = object::GQLObject { attributes };
        commits.push(gql_commit);
    }

    return commits;
}

fn select_branches(
    repo: &git2::Repository,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<object::GQLObject> {
    let mut branches: Vec<object::GQLObject> = Vec::new();
    let local_branches = repo.branches(None).unwrap();
    let is_limit_fields_empty = fields.is_empty();

    for branch in local_branches {
        let (branch, _) = branch.unwrap();

        let mut attributes: HashMap<String, String> = HashMap::new();

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            let key = if alias_table.contains_key("name") {
                alias_table.get("name").unwrap()
            } else {
                "name"
            }
            .to_string();

            attributes.insert(key, branch.name().unwrap().unwrap().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("commit_count")) {
            let key = if alias_table.contains_key("commit_count") {
                alias_table.get("commit_count").unwrap()
            } else {
                "commit_count"
            }
            .to_string();

            let branch_ref = branch.get().peel_to_commit().unwrap();
            let mut revwalk = repo.revwalk().unwrap();
            let _ = revwalk.push(branch_ref.id());
            attributes.insert(key, revwalk.count().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("is_head")) {
            let key = if alias_table.contains_key("is_head") {
                alias_table.get("is_head").unwrap()
            } else {
                "is_head"
            }
            .to_string();

            attributes.insert(key, branch.is_head().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("is_remote")) {
            let key = if alias_table.contains_key("is_remote") {
                alias_table.get("is_remote").unwrap()
            } else {
                "is_remote"
            }
            .to_string();

            attributes.insert(key, branch.get().is_remote().to_string());
        }

        let gql_branch = object::GQLObject { attributes };
        branches.push(gql_branch);
    }

    return branches;
}

fn select_tags(
    repo: &git2::Repository,
    fields: Vec<String>,
    alias_table: HashMap<String, String>,
) -> Vec<object::GQLObject> {
    let mut tags: Vec<object::GQLObject> = Vec::new();
    let tag_names = repo.tag_names(None).unwrap();
    let is_limit_fields_empty = fields.is_empty();

    for tag_name in tag_names.iter() {
        match tag_name {
            Some(name) => {
                let mut attributes: HashMap<String, String> = HashMap::new();
                if is_limit_fields_empty || fields.contains(&String::from("name")) {
                    let key = if alias_table.contains_key("name") {
                        alias_table.get("name").unwrap()
                    } else {
                        "name"
                    }
                    .to_string();

                    attributes.insert(key, name.to_string());
                    let gql_tag = object::GQLObject { attributes };
                    tags.push(gql_tag);
                }
            }
            None => {}
        }
    }
    return tags;
}
