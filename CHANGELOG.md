# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### shaku_axum 0.6.0
#### Breaking Changes
- Updated to axum 0.8 (thanks [@ethanhann](https://github.com/ethanhann))

## [0.6.2] - 2024-08-31
### Fixed
- Switched from `anymap` to `anymap2` to fix a possible soundness bug.
  See [RUSTSEC-2021-0065](https://rustsec.org/advisories/RUSTSEC-2021-0065.html)
  (thanks [@wolpert](https://github.com/wolpert))

## [2024-05-19]
### shaku_rocket 0.7.0
#### Breaking Changes
- Updated to support Rocket 0.5.0 (thanks [@BennyPLS](https://github.com/BennyPLS))

### shaku_axum 0.5.0
#### Breaking Changes
- Updated to axum 0.7

## [2023-04-20]
### shaku_axum 0.4.0
#### Breaking Changes
- Updated to axum 0.6

## [2022-04-01]
### shaku_actix 0.2.0
#### Breaking changes
- Updated to actix 4.0

### shaku_axum 0.3.0
#### Breaking Changes
- Updated to axum 0.5

## [2021-12-02]
### shaku_axum 0.2.0
#### Breaking Changes
- Updated to axum 0.4

## [2021-11-28]
### shaku_axum 0.1.0
- Initial release (thanks [@Dispersia](https://github.com/Dispersia)!).
  See [#31](https://github.com/AzureMarker/shaku/pull/31).

## [2021-06-13]
### shaku_rocket 0.7.0-rc.1
#### Breaking Changes
- Update Rocket to 0.5.0-rc.1

## [2021-04-10]
### shaku_derive 0.6.1
#### Added
- The `Component` derive now documents the generated parameters struct. The
  struct is documented as "Parameters for {component}" while the fields use the
  documentation attached to the original fields.

## [2021-02-09]
### shaku_rocket 0.6.0
#### Breaking Changes
- Modules must now be inserted into Rocket's state wrapped in a `Box`. This
  supports inserting module trait objects (e.g. `Box<dyn MyModule>`).

#### Added
- Added support for module trait objects. This is useful if the module changes
  at runtime (e.g. production vs development modules):
  ```rust
  trait MyModule: HasComponent<dyn MyService> {}

  #[get("/")]
  fn index(service: Inject<dyn MyModule, dyn MyService>) {
      // ...
  }
  ```

### shaku_actix 0.1.1
#### Added
- Relaxed requirements on the module type in `Inject` and `InjectProvided` to
  support module trait objects (e.g. `Arc<dyn MyModule>`):

  ```rust
  trait MyModule: HasComponent<dyn MyService> {}

  async fn index(service: Inject<dyn MyModule, dyn MyService>) {
    // ...
  }
  ```

## [0.6.1] - 2021-02-05
### Added
- Added `ModuleBuilder::with_component_override_fn`. This will allow you to
  override a component with a mock that has injected fields (i.e. is a
  `Component`). See [#24](https://github.com/AzureMarker/shaku/issues/24).

### Misc
- Added links to the guides to the readme.
- Added a section to the readme on `Component` vs `Provider`.
- Additionally run the tests without the `thread_safe` feature in CI.

## [0.6.0] - 2021-01-09
### Breaking Changes
- To support lazy components, `resolve_mut` is removed. It relied upon having a
  single `Arc` reference to the component, which can not be guaranteed in many
  cases (including the case of lazy components).
- To support lazy components, component parameters (the non-injected/provided
  struct properties) must implement `Send` when the `thread_safe` feature is
  enabled.
- `Module::build` now takes the `ModuleBuildContext` by value instead of by
  mutable reference.
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

### Changed
- Improved macro error messages by highlighting the relevant piece of code in
  the error (via `syn::Error`).

### Fixed
- Fixed accidentally importing `syn::export::Hash`, which is not part of the
  public API.

### shaku_actix 0.1.0
- New crate added to support Actix Web. It functions similarly to shaku_rocket.
  It works with both shaku 0.5 and 0.6.

### shaku_rocket 0.5.1
- This version supports both shaku 0.5 and 0.6. This crate is now independently
  versioned from the main shaku crate.

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

[Unreleased]: https://github.com/AzureMarker/shaku/compare/318619b80311266ecf66caa7d49e19b46a86c040...HEAD
[2024-05-19]: https://github.com/AzureMarker/shaku/commit/318619b80311266ecf66caa7d49e19b46a86c040
[2023-04-20]: https://github.com/AzureMarker/shaku/commit/c33b8e5258f6440b47ee8532168d7cdc91eb263b
[2022-04-01]: https://github.com/AzureMarker/shaku/commit/896beb182058b4db9b9643551c80016f9a76430b
[2021-12-02]: https://github.com/AzureMarker/shaku/commit/1d3451f5d980d4ad8549b2f8e93874ff40685910
[2021-11-28]: https://github.com/AzureMarker/shaku/commit/9cb8edb7b51c9cefd7c8bc1cdf073cb0ebe219ae
[2021-06-13]: https://github.com/AzureMarker/shaku/commit/065433b2ddf7e4269fd1cfc356d8d37c5260246c
[2021-04-10]: https://github.com/AzureMarker/shaku/commit/5eee11ef073d179215265fb60f2d506e28716f96
[2021-02-09]: https://github.com/AzureMarker/shaku/commit/342673133b06ea2fa5414a2458fa066e338b828e
[0.6.2]: https://github.com/AzureMarker/shaku/releases/tag/v0.6.2
[0.6.1]: https://github.com/AzureMarker/shaku/releases/tag/v0.6.1
[0.6.0]: https://github.com/AzureMarker/shaku/releases/tag/v0.6.0
[0.5.0]: https://github.com/AzureMarker/shaku/releases/tag/v0.5.0
[0.4.1]: https://github.com/AzureMarker/shaku/releases/tag/v0.4.1
[0.4.0]: https://github.com/AzureMarker/shaku/releases/tag/v0.4.0
[0.3.1]: https://github.com/AzureMarker/shaku/releases/tag/v0.3.1
[0.3.0]: https://github.com/AzureMarker/shaku/releases/tag/v0.3.0
[0.2.0]: https://github.com/AzureMarker/shaku/releases/tag/v0.2.0
[0.1.0]: https://github.com/AzureMarker/shaku/releases/tag/v0.1.0
