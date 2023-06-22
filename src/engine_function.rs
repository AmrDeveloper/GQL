use crate::object;

use std::collections::HashMap;

pub fn select_gql_objects(
    repo: &git2::Repository,
    table: String,
    fields: Vec<String>,
) -> Vec<object::GQLObject> {
    return match table.as_str() {
        "refs" => select_references(repo, fields),
        "commits" => select_commits(repo, fields),
        "branches" => select_branches(repo, fields),
        "tags" => select_tags(repo, fields),
        _ => vec![],
    };
}

fn select_references(repo: &git2::Repository, fields: Vec<String>) -> Vec<object::GQLObject> {
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
            attributes.insert(
                "name".to_string(),
                reference.shorthand().unwrap().to_string(),
            );
        }

        if is_limit_fields_empty || fields.contains(&String::from("full_name")) {
            attributes.insert(
                "full_name".to_string(),
                reference.name().unwrap().to_string(),
            );
        }

        if is_limit_fields_empty || fields.contains(&String::from("type")) {
            if reference.is_branch() {
                attributes.insert("kind".to_string(), "branch".to_owned());
            } else if reference.is_remote() {
                attributes.insert("kind".to_string(), "remote".to_owned());
            } else if reference.is_tag() {
                attributes.insert("kind".to_string(), "tag".to_owned());
            } else if reference.is_note() {
                attributes.insert("kind".to_string(), "note".to_owned());
            } else {
                attributes.insert("kind".to_string(), "other".to_owned());
            }
        }

        let gql_reference = object::GQLObject { attributes };
        gql_references.push(gql_reference);
    }

    return gql_references;
}

fn select_commits(repo: &git2::Repository, fields: Vec<String>) -> Vec<object::GQLObject> {
    let mut commits: Vec<object::GQLObject> = Vec::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let is_limit_fields_empty = fields.is_empty();

    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id.unwrap()).unwrap();

        let mut attributes: HashMap<String, String> = HashMap::new();

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            attributes.insert(
                "name".to_string(),
                commit.author().name().unwrap().to_string(),
            );
        }

        if is_limit_fields_empty || fields.contains(&String::from("email")) {
            attributes.insert(
                "email".to_string(),
                commit.author().email().unwrap().to_string(),
            );
        }

        if is_limit_fields_empty || fields.contains(&String::from("title")) {
            attributes.insert("title".to_string(), commit.summary().unwrap().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("message")) {
            attributes.insert("message".to_string(), commit.message().unwrap().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("time")) {
            attributes.insert("time".to_string(), commit.time().seconds().to_string());
        }

        let gql_commit = object::GQLObject { attributes };
        commits.push(gql_commit);
    }

    return commits;
}

fn select_branches(repo: &git2::Repository, fields: Vec<String>) -> Vec<object::GQLObject> {
    let mut branches: Vec<object::GQLObject> = Vec::new();
    let local_branches = repo.branches(None).unwrap();
    let is_limit_fields_empty = fields.is_empty();

    for branch in local_branches {
        let (branch, _) = branch.unwrap();

        let mut attributes: HashMap<String, String> = HashMap::new();

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            attributes.insert(
                "name".to_string(),
                branch.name().unwrap().unwrap().to_string(),
            );
        }

        if is_limit_fields_empty || fields.contains(&String::from("is_head")) {
            attributes.insert("ishead".to_string(), branch.is_head().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("is_remote")) {
            attributes.insert("isremote".to_string(), branch.get().is_remote().to_string());
        }

        let gql_branch = object::GQLObject { attributes };
        branches.push(gql_branch);
    }

    return branches;
}

fn select_tags(repo: &git2::Repository, fields: Vec<String>) -> Vec<object::GQLObject> {
    let mut tags: Vec<object::GQLObject> = Vec::new();
    let tag_names = repo.tag_names(None).unwrap();
    let is_limit_fields_empty = fields.is_empty();

    for tag_name in tag_names.iter() {
        match tag_name {
            Some(name) => {
                let mut attributes: HashMap<String, String> = HashMap::new();
                if is_limit_fields_empty || fields.contains(&String::from("name")) {
                    attributes.insert("name".to_string(), name.to_string());
                    let gql_tag = object::GQLObject { attributes };
                    tags.push(gql_tag);
                }
            }
            None => {}
        }
    }
    return tags;
}
