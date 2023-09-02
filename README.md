# Overview
The Conventional Commit CLI is a Command Line Interface (CLI) tool designed for creating [Conventional Commit](https://www.conventionalcommits.org/en/v1.0.0/) compliant Git commits.
It provides an interactive prompt to help you build your commit messages according to the Conventional Commit specifications, 
ensuring that your commit history is semantic and easily understandable.

## Usage
```bash
cli commit
```

## The Interactive Prompts

On running the command, you'll encounter a series of questions:

- `Type` - Kind of Commit: Choose the type of your commit (e.g., feat, fix, chore, docs, style, refactor, perf, test).
- `Scope` - Optionally, provide a scope for your commit.
- `Subject` - Write a short description of the change.
- `Body` - Optionally, provide a longer description of the change.
- `Breaking Change` - Specify if this is a breaking change.
- `Issues Affected` - Optionally, list any related issues, separated by commas.

## Examples

**The following:**

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

