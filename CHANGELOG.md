# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0] - 2020-05-26
### Breaking Changes
- Container is dead, long live Module! Resolving services now deals with modules
  instead of a Container of the module. Instead of building a container, you
  build the module. Instead of calling `Container::resolve`, now you call the
  module's `HasComponent::resolve` method (similar for providers). The methods
  in `HasComponent` have been changed to match the old `Container` methods.
  `ContainerBuilder` has been renamed to `ModuleBuilder` and is normally created
  by calling the generated `builder` method on the module to be build.
- Modules can depend on module interfaces, i.e., traits which have
  `HasComponent`/`HasProvider` bounds. Because of this, the submodules must be
  provided to the module builder during module build. See the [module macro]
  documentation for more details.
- `ProvidedInterface` has been removed. Consequently, provided services do not
  need to implement `Send` to be thread-safe anymore.

### Added
- Added `ModuleInterface` to automatically enforce submodule thread-safety. It
  is a trait alias functionally equivalent to `Interface`. User code should
  never have to reference it as it is a supertrait of `Module`, `HasComponent`,
  and `HasProvider` so it comes "for free".
- Added an optional module interface specifier to the module macro (add
  `: MyModuleInterface` after the module name). If a module interface is
  specified, the generated module will implement it.
- Added a [submodule guide]

## Changed
- shaku_rocket now ensures that the thread_safe feature is enabled.

[module macro]: https://docs.rs/shaku/0.4.0/shaku/macro.module.html
[submodule guide]: https://docs.rs/shaku/0.4.0/shaku/guide/submodules/index.html

## [0.3.1] - 2020-05-11
### Fixed
- Fix "no function or associated item named ..." errors when using the
  `module` macro to generate a module and certain traits are not in scope.

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

[Unreleased]: https://github.com/Mcat12/shaku/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/Mcat12/shaku/releases/tag/v0.4.0
[0.3.1]: https://github.com/Mcat12/shaku/releases/tag/v0.3.1
[0.3.0]: https://github.com/Mcat12/shaku/releases/tag/v0.3.0
[0.2.0]: https://github.com/Mcat12/shaku/releases/tag/v0.2.0
[0.1.0]: https://github.com/Mcat12/shaku/releases/tag/v0.1.0
