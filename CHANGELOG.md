# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2020-05-09
### Added
- Support submodules. Modules can now wrap other modules and use services from
  them. Modules can have multiple submodules.
- Add "submodule containers" (a type of `Container`) to support submodules.
  Normally these will only be used in `HasProvider` implementations.

## Changed
- Renamed `ContainerBuildContext` to `ModuleBuildContext` and modified some
  function signatures to support submodules.
- `Container` now has a lifetime parameter to support submodule containers.
  Normally this lifetime is eluded, so you shouldn't need to change your code.
- Moved getting started guides to dedicated module.

## [0.2.0] - 2020-02-23
### Breaking Changes
- Now uses compile time magic (generics) to check the dependency graph. This
  involves a lot of internal changes, although upgrading should be
  straightforward (add a Module type via the module macro and update
  ContainerBuilder usage).

## Changed
- Minimum supported Rust version changed to 1.38.0 (from 1.40.0)

## Removed
- Removed log dependency

## [0.1.0] - 2020-02-06
### Added
- Initial release

[Unreleased]: https://github.com/Mcat12/shaku/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/Mcat12/shaku/releases/tag/v0.3.0
[0.2.0]: https://github.com/Mcat12/shaku/releases/tag/v0.2.0
[0.1.0]: https://github.com/Mcat12/shaku/releases/tag/v0.1.0
