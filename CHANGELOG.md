## [0.7.1](https://github.com/sparkforge/node-size-analyzer/compare/v0.7.0...v0.7.1) (2025-05-07)


### Bug Fixes

* replace cli/setup-gh@v1 with manual GitHub CLI installation ([4a3dbc2](https://github.com/sparkforge/node-size-analyzer/commit/4a3dbc22d6398262acd4a0693beccf0a7849d4dd))

# [0.7.0](https://github.com/sparkforge/node-size-analyzer/compare/v0.6.0...v0.7.0) (2025-05-07)


### Features

* add detailed view for module inspection ([81edcfa](https://github.com/sparkforge/node-size-analyzer/commit/81edcfa13ec4e80e7651d3af71da4db190be7185))

# [0.6.0](https://github.com/sparkforge/node-size-analyzer/compare/v0.5.0...v0.6.0) (2025-05-07)


### Features

* add force publish workflow and version sync tools ([d1458c5](https://github.com/sparkforge/node-size-analyzer/commit/d1458c570d432d094c49e095ed15ca38c7283914))

# [0.5.0](https://github.com/sparkforge/node-size-analyzer/compare/v0.4.1...v0.5.0) (2025-05-06)


### Features

* add manual crates.io publishing workflow ([aaf8aaf](https://github.com/sparkforge/node-size-analyzer/commit/aaf8aaf8349f2f1b7b1c0dd394d56e7a654f0db2))

## [0.4.1](https://github.com/sparkforge/node-size-analyzer/compare/v0.4.0...v0.4.1) (2025-05-06)


### Bug Fixes

* update workflows to use CARGO_REGISTRY_TOKEN ([0941fe0](https://github.com/sparkforge/node-size-analyzer/commit/0941fe05675a1242e44844f5f6b9a4dd3c01ed5a))

# [0.4.0](https://github.com/sparkforge/node-size-analyzer/compare/v0.3.0...v0.4.0) (2025-05-06)


### Features

* enhance crates.io automated publishing ([d755266](https://github.com/sparkforge/node-size-analyzer/commit/d75526682d39ee55874e688f6966e3db99daa326))

# [0.3.0](https://github.com/sparkforge/node-size-analyzer/compare/v0.2.0...v0.3.0) (2025-05-06)


### Features

* add scrolling controls for module list ([e928f07](https://github.com/sparkforge/node-size-analyzer/commit/e928f075680ebd7dc75091ca28eb3bdb48fe8197))

# [0.2.0](https://github.com/sparkforge/node-size-analyzer/compare/v0.1.0...v0.2.0) (2025-05-06)


### Bug Fixes

* resolve shell substitution issues in semantic-release workflow ([99fa410](https://github.com/sparkforge/node-size-analyzer/commit/99fa410117e5d4677be9a8664834961c7e60bb24))


### Features

* configure semantic versioning and automated releases ([eab1b42](https://github.com/sparkforge/node-size-analyzer/commit/eab1b42dd47ca2e68e2c462c956c4157ecf85f99))

# Changelog

All notable changes to this project will be documented in this file. This project adheres to [Semantic Versioning](https://semver.org/).

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-05-06

### Added
- Initial release
- Interactive terminal UI using ratatui
- Real-time size calculation of node_modules
- Sorted display by size (largest modules first)
- Human-readable size formatting (B, KB, MB)
- Cross-platform support (Windows, MacOS, Linux)
