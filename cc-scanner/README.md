# CC-Scanner
conventional commit parser

> [!IMPORTANT]  
> Heyo! 👋
> 
> You were looking for a conventional commit parser, right? Well you'd be better served if you directed your attention here **👉**
> [conventional_commits_parser_rs](https://github.com/cocogitto/conventional_commits_parser_rs), which this library is heavily inspired by.



## Usage
The anatomy of a conventional commit:
<img width="819" alt="image" src="https://github.com/user-attachments/assets/0ad5b18e-2354-46d7-82a1-687bcf110857" />

The method of most importance is `parse_commit`, but this library also exports methods for parsing structural parts of a commit for convenience.

### Examples
**commit_type**
```rs
let commit_type = parse_commit_type("feat: add a new feature")?;

assert_eq!(commit_type, "feat");
```
**scope**
```rs
let scope = parse_scope("cli")?;

assert_eq!(scope.noun, "cli");
```
**description**
```rs
let description = parse_description("feat(scope): add something new")?;

assert_eq!(description, "add something new");
```
**body**
```rs
let body = parse_body("This is the body of the commit")?;

assert_eq!(body, "This is the body of the commit");
```
**footer**
```rs
let footer = parse_footer("Signed-off-by: Iroquois Pliskin")?;
assert_eq!(
  footer,
    Footer {
      token: "Signed-off-by".to_string(),
      separator: Separator::from(": "),
      content: "Iroquois Pliskin".to_string(),
    }
);
```
