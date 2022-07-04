# Git-ct

Git-ct lets you your own standard set of commit templates.

## Installation

`npm` is used to distribute `git-ct` even though it's not written in JS.

```shell
$> npm i -g git-ct
```

## Usage

First create a file in your project's root directoy called `commit-templates.toml`.
Check `commit-templates.default.toml` for an example of what goes there.

Then you can call `git-ct` instead of `git commit`.

```shell
$> git-ct
? Choose a template:
> Fix 🐞
  Docs ✏️
  Perf ⚡️
  Refactor 💡
  Feat 🎸
  Release 🏹
v Style 💄
[↑↓ to move, enter to select, type to filter]
```

After selecting an option and filling in the values, you editor of choice ($EDITOR)
will be opened to let you edit the commit message.
