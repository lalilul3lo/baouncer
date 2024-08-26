use std::io;
use std::process::{Command, Output};

pub trait GitCommandRunner {
    fn has_staged_changes(&self) -> bool;
    fn commit(&self, commit: String) -> io::Result<Output>;
}

pub struct GitRunner;

impl GitRunner {
    pub fn new() -> Self {
        Self {}
    }
}

impl GitCommandRunner for GitRunner {
    fn has_staged_changes(&self) -> bool {
        let output_status = Command::new("git")
            .args(["--no-pager", "diff", "--staged"])
            .output()
            .expect("Failed to execute git command");

        let status_message = String::from_utf8_lossy(&output_status.stdout)
            .trim()
            .to_string();

        !status_message.is_empty()
    }

    fn commit(&self, commit: String) -> io::Result<Output> {
        Command::new("git").args(["commit", "-m", &commit]).output()
    }
}

pub struct CommitWriter<R: GitCommandRunner> {
    runner: R,
}

impl<R: GitCommandRunner> CommitWriter<R> {
    pub fn new(runner: R) -> Self {
        Self { runner }
    }

    pub fn write_commit(&self, commit: String) -> io::Result<()> {
        let staged_changes = self.runner.has_staged_changes();

        if !staged_changes {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "There are no staged changes to commit.",
            ));
        }

        let output = self.runner.commit(commit)?;

        if output.status.success() {
            Ok(())
        } else {
            let error_message = String::from_utf8_lossy(&output.stderr);
            Err(io::Error::new(io::ErrorKind::Other, error_message))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{io::ErrorKind, os::unix::process::ExitStatusExt, process::ExitStatus};

    use super::*;

    struct MockGitCommandRunner {
        has_staged: bool,
    }

    impl MockGitCommandRunner {
        fn new(has_staged: bool) -> Self {
            Self { has_staged }
        }
    }

    impl GitCommandRunner for MockGitCommandRunner {
        fn has_staged_changes(&self) -> bool {
            self.has_staged
        }

        fn commit(&self, _commit: String) -> io::Result<Output> {
            Ok(Output {
                stdout: vec![],
                stderr: vec![],
                status: ExitStatus::from_raw(0),
            })
        }
    }

    #[test]
    fn test_write_commit_no_staged_changes() {
        let commit = String::from("some commit");

        let mock_runner = MockGitCommandRunner::new(false);

        let commit_writer = CommitWriter::new(mock_runner);

        match commit_writer.write_commit(commit) {
            Ok(_) => panic!("Expected error, got Ok()"),
            Err(e) => {
                assert_eq!(e.kind(), ErrorKind::InvalidInput);
                assert_eq!(e.to_string(), "There are no staged changes to commit.");
            }
        }
    }
    #[test]
    fn test_write_commit_staged_changes() {
        let commit = String::from("some commit");

        let mock_runner = MockGitCommandRunner::new(true);

        let commit_writer = CommitWriter::new(mock_runner);

        if commit_writer.write_commit(commit).is_err() {
            panic!("Expected Ok(), got error");
        }
    }
}
