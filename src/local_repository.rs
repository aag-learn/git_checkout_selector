use git2::{BranchType, Repository};

pub struct LocalRepository {
    repo: git2::Repository,
}

impl LocalRepository {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn branch_names(&self) -> Vec<String> {
        let local_branch_option: Option<BranchType> = Some(BranchType::Local);
        let local_branches = match self.repo.branches(local_branch_option) {
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
}

impl Default for LocalRepository {
    fn default() -> Self {
        Self {
            repo: match Repository::open("./") {
                Ok(repo) => repo,
                Err(e) => panic!("failed to open: {}", e),
            },
        }
    }
}
