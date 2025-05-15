use git2::{Error, IndexAddOption, Repository};

use crate::output::CommandResult;

pub fn commit_impl(msg: &str) -> Result<String, Error> {
    let repo = Repository::open(".")?;
    let mut index = repo.index()?;

    let _ = index.add_all(["."], IndexAddOption::DEFAULT, None);
    let _ = index.write();

    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    let parent = repo.head()?.peel_to_commit()?;

    let commit = repo.commit(
        Some("HEAD"),
        &repo.signature()?,
        &repo.signature()?,
        msg,
        &tree,
        &[&parent],
    )?;

    Ok(format!("{}", commit))
}

pub fn commit(msg: &str) -> CommandResult<String> {
    match commit_impl(msg) {
        Ok(commit) => CommandResult {
            status: true,
            value: commit,
        },
        Err(msg) => CommandResult {
            status: false,
            value: String::from(msg.message()),
        },
    }
}
