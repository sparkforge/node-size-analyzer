# Contributing to Node Size Analyzer

Thank you for considering contributing to Node Size Analyzer! This document provides guidelines for contributing to the project and explains our semantic release process.

## Commit Message Guidelines

We use semantic versioning and conventional commits to automatically determine version numbers and generate changelogs. Please follow these rules when writing commit messages:

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### Types

The `type` field must be one of the following:

- **feat**: A new feature (triggers a MINOR version bump)
- **fix**: A bug fix (triggers a PATCH version bump)
- **docs**: Documentation only changes (triggers a PATCH version bump)
- **style**: Changes that do not affect the meaning of the code (triggers a PATCH version bump)
- **refactor**: A code change that neither fixes a bug nor adds a feature (triggers a PATCH version bump)
- **perf**: A code change that improves performance (triggers a PATCH version bump)
- **test**: Adding missing tests or correcting existing tests (triggers a PATCH version bump)
- **build**: Changes that affect the build system or external dependencies (triggers a PATCH version bump)
- **ci**: Changes to our CI configuration files and scripts (triggers a PATCH version bump)
- **chore**: Other changes that don't modify src or test files (triggers a PATCH version bump)

### Breaking Changes

Breaking changes should be indicated by adding `BREAKING CHANGE:` in the commit message body, or by appending `!` after the type/scope. This will trigger a MAJOR version bump.

Example:
```
feat(api)!: change API response format

BREAKING CHANGE: The API response format has been completely redesigned.
```

### Examples

```
feat: add support for nested modules scanning

fix(output): correct size calculation for symlinks

docs: update installation instructions

test: add unit tests for directory traversal

refactor!: completely redesign the CLI interface

BREAKING CHANGE: The CLI interface has been redesigned with new flag names.
```

## Pull Request Process

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Add tests for your changes
4. Ensure all tests pass (`cargo test`)
5. Commit your changes following the semantic commit message format
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

When your PR is merged to main, our fully automated semantic release process will:

1. Analyze commit messages to determine the next version number
2. Update the version in Cargo.toml
3. Create/update the CHANGELOG.md entry
4. Create a Git tag for the new version
5. Create a new GitHub release with binaries for all platforms
6. Automatically publish to crates.io

The entire process is automated through GitHub Actions workflows:

- **Semantic Release Workflow**: Triggered when code is pushed to main. It analyzes commits, determines the next version, updates files, and creates a tag.
- **Release Workflow**: Triggered by the new tag. It builds binaries, creates a GitHub release, and publishes to crates.io.

### Publishing to crates.io

Publishing to crates.io is fully automated. The workflow:

1. Verifies the package is publishable with a dry run
2. Validates that the version in Cargo.toml matches the Git tag
3. Publishes to crates.io with automatic retries if needed
4. Verifies the package is available on crates.io

No manual action is required for releases if your commits follow the conventional commit format.

## Development Setup

```bash
git clone https://github.com/Caryyon/node-size-analyzer.git
cd node-size-analyzer
cargo build
```

## Running Tests

```bash
cargo test
```

Thank you for contributing to Node Size Analyzer!