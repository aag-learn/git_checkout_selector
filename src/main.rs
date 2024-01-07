use crate::ui::UI;
use git2::{BranchType, Repository};

pub mod ui;
// We'll use a SelectView here.
//
// A SelectView is a scrollable list of items, from which the user can select
// one.

fn main() {
    let repo = init_repo();
    let app = UI {
        branch_names: get_branch_names_from_repo(&repo),
    };
    app.start();
}

fn get_branch_names_from_repo(repo: &Repository) -> Vec<String> {
    let local_branch_option: Option<BranchType> = Some(BranchType::Local);
    let local_branches = match repo.branches(local_branch_option) {
        Ok(branches) => branches,
        Err(e) => panic!("failed to get branches: {}", e),
    };
    let mut branch_names: Vec<String> = Vec::new();
    for result in local_branches {
        match result {
            Ok((b, _btype)) => match b.name() {
                Ok(n) => branch_names.push(String::from(n.unwrap())),
                Err(e) => panic!("error! {}", e),
            },
            Err(e) => panic!("error! {}", e),
        };
    }
    branch_names
}

fn init_repo() -> Repository {
    match Repository::open("./") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    }
}
