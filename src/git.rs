use std::error::Error;
use std::path::PathBuf;

use crate::url::Remote;
use git2::Repository;

pub struct Repo {
    repository: Repository,
}

impl Repo {
    pub fn new(path: &PathBuf) -> Self {
        let mut cwd = path.clone();

        let repository = loop {
            match Repository::open(&cwd) {
                Ok(r) => break r,
                Err(_e) => {
                    if !cwd.pop() {
                        panic!("Unable to open repository at path or parent: {:?}", path);
                    }
                }
            }
        };

        Self { repository }
    }

    pub fn remote(&self, name: &str) -> Result<Remote, Box<dyn Error>> {
        let repo_remote = self.repository.find_remote(name).unwrap();
        let remote_url = repo_remote.url().unwrap();
        if let Some(remote) = Remote::parse(remote_url) {
            return Ok(remote);
        }
        Err("nop".into())
    }

    pub fn exist(&self, remote: &str, branch: &str) -> bool {
        let reference_name = format!("refs/remotes/{}/{}", remote, branch);
        match self.repository.find_reference(&reference_name) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn current_branch(&self) -> Result<String, Box<dyn Error>> {
        let head = self.repository.head()?;
        if let Some(name) = head.shorthand() {
            Ok(name.to_string())
        } else {
            Err("Could not get current branch name".into())
        }
    }
}
