# Contributing to Piltover

Welcome, contributing to `piltover` is easy!

1. Submit or comment your intent on an issue
1. We will try to respond quickly
1. Fork this repo
1. Submit your PR against `main`
1. Address PR Review

## Issue

Project tracking is done via GitHub [issues](https://github.com/keep-starknet-strange/piltover/issues).
First look at open issues to see if your request is already submitted.
If it is comment on the issue requesting assignment, if not open an issue.

We use 3 issue labels for development:

- `feat` -> suggest new feature
- `bug` -> create a reproducible bug report
- `dev` -> non-functional repository changes

These labels are used as prefixes as follows for `issue`, `branch name`, `pr title`:

- `[feat]` -> `feat/{issue #}-{issue name}` -> `feat:`
- `[bug]` -> `bug/{issue #}-{issue name}` -> `bug:`
- `[dev]` -> `dev/{issue #}-{issue name}` -> `dev:`

## Submit PR

Ensure your code is well formatted, well tested and well documented. A core contributor
will review your work. Address changes, ensure ci passes,
and voilà you're a piltover contributor.

Markdown [linter](https://github.com/markdownlint/markdownlint?tab=readme-ov-file#markdown-lint-tool):

```bash
mdl -s .github/linter/readme_style.rb README.md
```

Scarb linter:

```bash
scarb fmt
```
