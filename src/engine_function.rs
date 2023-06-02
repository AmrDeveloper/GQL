use crate::object;

use std::collections::HashMap;

pub fn select_gql_objects(
    repo: &git2::Repository,
    table: String,
    fields: Vec<String>,
) -> Vec<object::GQLObject> {
    if table == "commits" {
        return select_commits(repo, fields);
    }

    if table == "branches" {
        return select_branches(repo, fields);
    }
    return vec![];
}

fn select_commits(repo: &git2::Repository, fields: Vec<String>) -> Vec<object::GQLObject> {
    let mut commits: Vec<object::GQLObject> = Vec::new();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id.unwrap()).unwrap();

        let mut attributes: HashMap<String, String> = HashMap::new();
        let is_limit_fields_empty = fields.is_empty();

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

        let gql_commit = object::GQLObject { attributes };
        commits.push(gql_commit);
    }

    return commits;
}

fn select_branches(repo: &git2::Repository, fields: Vec<String>) -> Vec<object::GQLObject> {
    let mut branches: Vec<object::GQLObject> = Vec::new();
    let local_branches = repo.branches(None).unwrap();

    for branch in local_branches {
        let (branch, _) = branch.unwrap();

        let mut attributes: HashMap<String, String> = HashMap::new();
        let is_limit_fields_empty = fields.is_empty();

        if is_limit_fields_empty || fields.contains(&String::from("name")) {
            attributes.insert(
                "name".to_string(),
                branch.name().unwrap().unwrap().to_string(),
            );
        }

        if is_limit_fields_empty || fields.contains(&String::from("ishead")) {
            attributes.insert("ishead".to_string(), branch.is_head().to_string());
        }

        if is_limit_fields_empty || fields.contains(&String::from("isremote")) {
            attributes.insert("isremote".to_string(), branch.get().is_remote().to_string());
        }

        let gql_branch = object::GQLObject { attributes };
        branches.push(gql_branch);
    }

    return branches;
}
