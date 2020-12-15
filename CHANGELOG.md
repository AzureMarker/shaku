# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Breaking Changes
- To support lazy components, `resolve_mut` is removed. It relied upon having a
  single `Arc` reference to the component, which can not be guaranteed in many
  cases (including the case of lazy components).
- To support lazy components, component parameters (the non-injected/provided
  struct properties) must implement `Send` when the `thread_safe` feature is
  enabled.
- Component parameters no longer require `Default` by default (pun intended)
  when using the Component derive.
  If a parameter is not provided during module creation, there will be a panic.
  The `#[shaku(default)]` and `#[shaku(default = ...)]` annotations can be used
  to enable a default (first via the `Default` trait, second via the provided
  expression). The `#[shaku(no_default)]` annotation has been removed since it
  is now the default.

### Added
- Components can now be lazily created. Annotate a component in the module with
  `#[lazy]` to make it lazy:
  ```rust
  module! {
      MyModule {
          components = [#[lazy] ServiceImpl],
          providers = []
      }
  }
  ```
  Now `ServiceImpl` will not be created until `resolve` or `resolve_ref` is
  called to access it, or until it is required by another component/provider.

## [0.5.0] - 2020-06-19
### Breaking Changes
- The `module` macro is now a procedural macro and thus requires the `derive`
  feature.
- The `module` macro can no longer be used in statement position until 1.45,
  when support for proc-macros in that position becomes stable.

### Added
- The `module` macro now supports associated types in generic module bounds
  and where clauses on the module.
- A new annotation for properties without a default is added:
  `#[shaku(no_default)]`. If the parameters are not provided for a component
  with such a property, there will be a panic during module creation (unless the
  component is overridden). See the `no_default_parameter` test for an example.

## [0.4.1] - 2020-06-01
### Added
- Support generics in derives and the `module` macro. For example:
  ```rust
  use shaku::{module, Component, Interface, HasComponent};

  trait MyComponent<T: Interface>: Interface {}

  #[derive(Component)]
  #[shaku(interface = MyComponent<T>)]
  struct MyComponentImpl<T: Interface + Default> {
      value: T
  }
  impl<T: Interface + Default> MyComponent<T> for MyComponentImpl<T> {}

  module! {
      MyModule<T: Interface + Default> {
          components = [MyComponentImpl<T>],
          providers = []
      }
  }
  ```

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

[Unreleased]: https://github.com/Mcat12/shaku/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/Mcat12/shaku/releases/tag/v0.5.0
[0.4.1]: https://github.com/Mcat12/shaku/releases/tag/v0.4.1
[0.4.0]: https://github.com/Mcat12/shaku/releases/tag/v0.4.0
[0.3.1]: https://github.com/Mcat12/shaku/releases/tag/v0.3.1
[0.3.0]: https://github.com/Mcat12/shaku/releases/tag/v0.3.0
[0.2.0]: https://github.com/Mcat12/shaku/releases/tag/v0.2.0
[0.1.0]: https://github.com/Mcat12/shaku/releases/tag/v0.1.0
