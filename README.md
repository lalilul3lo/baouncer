# Overview
The Conventional Commit CLI is a Command Line Interface (CLI) tool designed for creating [Conventional Commit](https://www.conventionalcommits.org/en/v1.0.0/) compliant Git commits.
It provides an interactive prompt to help you build your commit messages according to the Conventional Commit specifications, 
ensuring that your commit history is semantic and easily understandable.

## Installation
Until the future of this project has been decided, you'll need to clone and cargo install locally.

**clone**
```bash
git clone https://github.com/lalilul3lo/baouncer.git
```

**cd into root of project**
```bash
cd baouncer
```

**install**
```bash
cargo install --path .
```

## commands

### `commit`

**On running the command, you'll encounter a series of questions:**

- `Type` - Kind of Commit: Choose the type of your commit (e.g., feat, fix, chore, docs, style, refactor, perf, test).
- `Scope` - Optionally, provide a scope for your commit.
- `Subject` - Write a short description of the change.
- `Body` - Optionally, provide a longer description of the change.
- `Breaking Change` - Specify if this is a breaking change.
- `Issues Affected` - Optionally, list any related issues, separated by commas.

**For example, the following answers:**

- Type of Commit: `feat - A new feature (🎉)`
- Scope: `authentication`
- Subject: `implement JWT authentication`
- Body: `This commit implements JWT authentication for better security.`
- Breaking Change: `""`
- Issues Affected: `42,43`

**...will generate the following commit message:***

```bash
feat(authentication): implement JWT authentication

This commit implements JWT authentication for better security.

closes #42, #43
```


### `commit-msg-hook`
Will generate a `git` `commit-msg` hook at `./git/hooks/commit-msg`, that looks like the following:
```sh
#!/usr/bin/env sh

commit_message=$(grep -v '^#' "$1")

baouncer check-commit -m "$commit_message"
```

### `commit-msg-hook`
Validates whether or not a string is a [Conventional Commit](https://www.conventionalcommits.org/en/v1.0.0/) compliant `git` commit.
