use cc_scanner::conventional_commit::ConventionalCommit;
use git2::{Oid, Reference, Repository};
use log::info;
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CommitError {
    /// The working tree has unstaged changes, preventing commit.
    #[error("Unstaged changes exist, cannot commit")]
    UnstagedChanges,

    /// A Git-specific error from the `git2` crate.
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
}

fn has_unstaged_changes(head: Reference<'_>, tree_id: Oid) -> Result<bool, CommitError> {
    let head_commit = head.peel_to_commit()?;
    let head_tree_id = head_commit.tree_id();

    Ok(head_tree_id == tree_id)
}

pub fn commit(mut commit: ConventionalCommit) -> Result<Oid, CommitError> {
    let current_dir = std::env::current_dir().expect("");
    let repo = Repository::discover(&current_dir)?;
    let sig = repo.signature()?;
    let tree_id = repo.index()?.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let head = repo.head()?;
    let head_target = head.target().expect("Cannot get HEAD target");
    let tip = repo.find_commit(head_target)?;
    let parents = &[&tip];

    if has_unstaged_changes(head, tree_id)? {
        Err(CommitError::UnstagedChanges)
    } else {
        let oid = repo.commit(Some("HEAD"), &sig, &sig, &commit.as_str(), &tree, parents)?;

        info!("\n{}", commit.as_str());

        Ok(oid)
    }
}
