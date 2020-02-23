# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Now uses compile time magic (generics) to check the dependency graph. This
  involves a lot of internal changes, although upgrading should be
  straightforward (add a Module type via the module macro and update
  ContainerBuilder usage).
- Minimum supported Rust version changed to 1.38.0 (from 1.40.0)

## [0.1.0] - 2020-02-06
### Added
- Initial release

[Unreleased]: https://github.com/Mcat12/shaku/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Mcat12/shaku/releases/tag/v0.1.0
