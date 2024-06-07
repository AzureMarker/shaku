//! # Getting started with submodules
//! This guide assumes you have already read the general [getting started guide].
//!
//! [`Module`]s can contain other modules, called submodules. For example, a
//! top-level `RootModule` may use an `AuthModule` to provide authentication
//! services. The submodule's implementation may be known, or the submodule may
//! be hidden behind a trait (aka module interface).
//!
//! ## Why use submodules?
//! Unlike components or providers, the implementation of a submodule does not need to be known to
//! the module. You can easily swap out, say, an OAuth authenticaion implementation for a fake
//! during development. This is more powerful than overriding components/providers because with
//! overriding you cannot remove services from the dependency graph. For example, the OAuth
//! `AuthManager` implementation may use an `OAuthKeyStore` internally. When you swap the module out
//! for a fake, the `OAuthKeyStore` is no longer part of the dependency graph. If the `AuthManager`
//! was instead overridden, the keystore would still be created or would need to be overridden as
//! well, despite being internal or private to the OAuth `AuthManager`.
//!
//! ## The example
//! ```rust
//! use shaku::{module, Component, HasComponent, Interface};
//! use std::sync::Arc;
//!
//! trait MyComponent: Interface {}
//! trait AuthManager: Interface {}
//! trait AuthModule: HasComponent<dyn AuthManager> {}
//!
//! #[derive(Component)]
//! #[shaku(interface = MyComponent)]
//! struct MyComponentImpl {
//!     #[shaku(inject)]
//!     auth_manager: Arc<dyn AuthManager>
//! }
//! impl MyComponent for MyComponentImpl {}
//!
//! module! {
//!     RootModule {
//!         components = [MyComponentImpl],
//!         providers = [],
//!         interfaces = [],
//!
//!         use dyn AuthModule {
//!             components = [dyn AuthManager],
//!             providers = [],
//!             interfaces = []
//!         }
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! In this example, `RootModule` knows the implementation of its `MyComponent` component, but it
//! does not know the implementation of `AuthManager` or the `AuthModule` that it gets it from. The
//! `AuthModule` implementation is passed in when `RootModule` is built.
//!
//! ## Providing submodule implementations
//! To build a module, you need to give it a reference to each submodule implementation. The
//! [`module`][module macro] macro will generate a `builder` function which takes in the submodules
//! and outputs a [`ModuleBuilder`]
//!
//! ```rust
//! # use shaku::{module, Component, HasComponent, Interface};
//! # use std::sync::Arc;
//! #
//! # trait MyComponent: Interface {}
//! # trait AuthManager: Interface {}
//! # trait AuthModule: HasComponent<dyn AuthManager> {}
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = MyComponent)]
//! # struct MyComponentImpl { #[shaku(inject)] auth_manager: Arc<dyn AuthManager> }
//! # impl MyComponent for MyComponentImpl {}
//! #
//! # module! {
//! #     RootModule {
//! #         components = [MyComponentImpl], providers = [], interfaces = [],
//! #         use dyn AuthModule { components = [dyn AuthManager], providers = [], interfaces = [] }
//! #     }
//! # }
//! #
//! #[derive(Component)]
//! #[shaku(interface = AuthManager)]
//! struct AuthManagerImpl;
//! impl AuthManager for AuthManagerImpl {}
//!
//! module! {
//!     AuthModuleImpl: AuthModule {
//!         components = [AuthManagerImpl],
//!         providers = [],
//!         interfaces = []
//!     }
//! }
//!
//! # fn main() {
//! let auth_module = Arc::new(AuthModuleImpl::builder().build());
//! let root_module = RootModule::builder(auth_module).build();
//!
//! let my_component: &dyn MyComponent = root_module.resolve_ref();
//! # }
//! ```
//!
//! `AuthModuleImpl` has no submodules, thus its `builder` function has no arguments.
//! `RootModule` has one submodule, thus its `builder` function takes in an
//! `Arc<dyn AuthModule>`.
//!
//! Note: `AuthModuleImpl` uses a feature of the [`module`][module macro] macro to automatically
//! implement `AuthModule`. It does this by adding `: AuthModule` after the name of the module.
//! This is shorthand for the statement `impl AuthModule for AuthModuleImpl {}`.
//!
//! [getting started guide]: ../index.html
//! [`Module`]: ../../trait.Module.html
//! [module macro]: ../../macro.module.html
//! [`ModuleBuilder`]: ../../struct.ModuleBuilder.html
