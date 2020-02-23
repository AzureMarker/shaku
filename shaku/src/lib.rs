//! Shaku is a compile time dependency injection library. It can be used directly or through
//! integration with application frameworks such as [Rocket](https://rocket.rs) (see
//! [`shaku_rocket`](https://crates.io/crates/shaku_rocket)).
//!
//! # Getting started
//! Note: This getting started guide focuses on components, which live for the lifetime of the
//! application (or, technically, the container). After reading this getting started guide, check
//! out [`Provider`] to learn how to create services with shorter lifetimes.
//!
//! ## Structure your application
//! Start with your application's structs and traits. Use `Arc<dyn T>` for
//! dependencies.
//!
//! ```
//! use std::sync::Arc;
//!
//! trait IOutput {
//!     fn write(&self, content: String);
//! }
//!
//! trait IDateWriter {
//!     fn write_date(&self);
//! }
//!
//! struct ConsoleOutput;
//!
//! impl IOutput for ConsoleOutput {
//!     fn write(&self, content: String) {
//!         println!("{}", content);
//!     }
//! }
//!
//! struct TodayWriter {
//!     output: Arc<dyn IOutput>,
//!     today: String,
//!     year: usize,
//! }
//!
//! impl IDateWriter for TodayWriter {
//!     fn write_date(&self) {
//!         self.output.write(format!("Today is {}, {}", self.today, self.year));
//!     }
//! }
//! ```
//!
//! ## Inherit "Interface" for the interface traits
//!
//! Interface traits require certain bounds, such as `'static` and optionally `Send + Sync` if using
//! the `thread_safe` feature. The [`Interface`] trait acts as a trait alias for these bounds, and is
//! automatically implemented on types which implement the bounds.
//!
//! In our example, the two interface traits would become:
//!
//! ```
//! use shaku::Interface;
//!
//! trait IOutput: Interface {
//!     fn write(&self, content: String);
//! }
//!
//! trait IDateWriter: Interface {
//!     fn write_date(&self);
//! }
//! ```
//!
//! ## Implement Component
//! A component is a struct that implements an [`Interface`] trait. In our example, we have 2
//! components:
//!
//! - `TodayWriter` of type `IDateWriter`
//! - `ConsoleOutput` of type `IOutput`
//!
//! These components must implement [`Component`], which can either be done manually or through a
//! derive macro (using the `derive` feature):
//!
//! ```
//! # use shaku::Interface;
//! #
//! # trait IOutput: Interface { fn write(&self, content: String); }
//! #
//! # impl IOutput for ConsoleOutput {
//! #     fn write(&self, content: String) { println!("{}", content); }
//! # }
//! #
//! use shaku::Component;
//!
//! #[derive(Component)]
//! #[shaku(interface = IOutput)]
//! struct ConsoleOutput;
//! ```
//!
//! ## Express dependencies
//! Components can depend on other components. In our example, `TodayWriter` requires an `IOutput`
//! component.
//!
//! To express this dependency, first make sure the property is declared as a
//! [trait object](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) wrapped in an [`Arc`].
//! Then (when using the derive macro) use the `#[shaku(inject)]` attribute on the property to tell
//! shaku to inject the dependency.
//!
//! In our example:
//!
//! ```
//! # use shaku::Interface;
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface { fn write(&self, content: String); }
//! # trait IDateWriter: Interface { fn write_date(&self); }
//! #
//! # impl IDateWriter for TodayWriter {
//! #     fn write_date(&self) {
//! #         self.output.write(format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! use shaku::Component;
//!
//! #[derive(Component)]
//! #[shaku(interface = IDateWriter)]
//! struct TodayWriter {
//!     #[shaku(inject)]
//!     output: Arc<dyn IOutput>,
//!     today: String,
//!     year: usize,
//! }
//! ```
//!
//! If you don't use the derive macro, add [`HasComponent`] bounds to your module generic and inject
//! the dependencies manually with [`ContainerBuildContext::resolve`].
//!
//! ## Create a Module
//! Modules link together components and providers, and is core to providing shaku's compile time
//! guarentees. A [`Module`] can be created manually or via the [`module`][module macro] macro (the `derive`
//! feature is not necessary):
//!
//! ```
//! # use shaku::{Component, Interface};
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface { fn write(&self, content: String); }
//! # trait IDateWriter: Interface { fn write_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IOutput)]
//! # struct ConsoleOutput;
//! # impl IOutput for ConsoleOutput {
//! #     fn write(&self, content: String) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IDateWriter)]
//! # struct TodayWriter {
//! #     #[shaku(inject)]
//! #     output: Arc<dyn IOutput>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl IDateWriter for TodayWriter {
//! #     fn write_date(&self) {
//! #         self.output.write(format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! use shaku::module;
//!
//! module! {
//!     MyModule {
//!         components = [ConsoleOutput, TodayWriter],
//!         providers = []
//!     }
//! }
//! ```
//!
//! This module implements `HasComponent<dyn IOutput>` and `HasComponent<dyn IDateWriter>` using the
//! provided component implementations.
//!
//! ## Build a Container
//! At application startup, create a [`Container`] using a [`ContainerBuilder`]. You can use this
//! container to resolve the module's services.
//!
//! ```
//! # use shaku::{module, Component, Interface};
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface { fn write(&self, content: String); }
//! # trait IDateWriter: Interface { fn write_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IOutput)]
//! # struct ConsoleOutput;
//! # impl IOutput for ConsoleOutput {
//! #     fn write(&self, content: String) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IDateWriter)]
//! # struct TodayWriter {
//! #     #[shaku(inject)]
//! #     output: Arc<dyn IOutput>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl IDateWriter for TodayWriter {
//! #     fn write_date(&self) {
//! #         self.output.write(format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! # module! {
//! #     MyModule {
//! #         components = [ConsoleOutput, TodayWriter],
//! #         providers = []
//! #     }
//! # }
//! #
//! use shaku::{Container, ContainerBuilder};
//!
//! let container: Container<MyModule> = ContainerBuilder::new().build();
//! // Alternatively, let container = Container::<MyModule>::default();
//! ```
//!
//! ### Passing parameters
//! In many cases you need to pass parameters to a component. This can be done during container
//! creation. Each component has an associated parameters type, and the derive generates a
//! `*Parameters` struct for you (named after the component struct). Use this struct to pass in the
//! parameters.
//!
//! Note that if you don't pass in parameters, the parameters' default values will be used. You can
//! override the default value by annotating the property with `#[shaku(default = ...)]`.
//!
//! ```
//! # use shaku::{module, Component, Container, ContainerBuilder, Interface};
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface { fn write(&self, content: String); }
//! # trait IDateWriter: Interface { fn write_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IOutput)]
//! # struct ConsoleOutput;
//! # impl IOutput for ConsoleOutput {
//! #     fn write(&self, content: String) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IDateWriter)]
//! # struct TodayWriter {
//! #     #[shaku(inject)]
//! #     output: Arc<dyn IOutput>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl IDateWriter for TodayWriter {
//! #     fn write_date(&self) {
//! #         self.output.write(format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! # module! {
//! #     MyModule {
//! #         components = [ConsoleOutput, TodayWriter],
//! #         providers = []
//! #     }
//! # }
//! #
//! let container: Container<MyModule> = ContainerBuilder::new()
//!     .with_component_parameters::<TodayWriter>(TodayWriterParameters {
//!         today: "Jan 26".to_string(),
//!         year: 2020
//!     })
//!     .build();
//! ```
//!
//! ## Resolve components
//! Once you created the [`Container`], you can resolve the components.
//!
//! ```
//! # use shaku::{module, Component, Container, ContainerBuilder, Interface};
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface { fn write(&self, content: String); }
//! # trait IDateWriter: Interface { fn write_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IOutput)]
//! # struct ConsoleOutput;
//! # impl IOutput for ConsoleOutput {
//! #     fn write(&self, content: String) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IDateWriter)]
//! # struct TodayWriter {
//! #     #[shaku(inject)]
//! #     output: Arc<dyn IOutput>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl IDateWriter for TodayWriter {
//! #     fn write_date(&self) {
//! #         self.output.write(format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! # module! {
//! #     MyModule {
//! #         components = [ConsoleOutput, TodayWriter],
//! #         providers = []
//! #     }
//! # }
//! #
//! # let container: Container<MyModule> = ContainerBuilder::new()
//! #     .with_component_parameters::<TodayWriter>(TodayWriterParameters {
//! #         today: "Jan 26".to_string(),
//! #         year: 2020
//! #     })
//! #     .build();
//! #
//! let writer: &dyn IDateWriter = container.resolve_ref();
//! writer.write_date(); // Prints "Today is Jan 26, 2020"
//! ```
//!
//! ## Overriding components
//! Although shaku is a compile time DI library, you can override the implementation of a service
//! during the container build. This can be useful during testing, for example using an in-memory
//! database while doing integration tests. For components, simply pass in a struct instance which
//! implements the interface you want to override to [`with_component_override`]\:
//!
//! ```
//! # use shaku::{module, Component, Container, ContainerBuilder, Interface};
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface { fn write(&self, content: String); }
//! # trait IDateWriter: Interface { fn write_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IOutput)]
//! # struct ConsoleOutput;
//! # impl IOutput for ConsoleOutput {
//! #     fn write(&self, content: String) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IDateWriter)]
//! # struct TodayWriter {
//! #     #[shaku(inject)]
//! #     output: Arc<dyn IOutput>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl IDateWriter for TodayWriter {
//! #     fn write_date(&self) {
//! #         self.output.write(format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! # module! {
//! #     MyModule {
//! #         components = [ConsoleOutput, TodayWriter],
//! #         providers = []
//! #     }
//! # }
//! #
//! #[derive(Component)]
//! #[shaku(interface = IOutput)]
//! struct FakeOutput;
//!
//! impl IOutput for FakeOutput {
//!     fn write(&self, _content: String) {
//!         // We don't want to actually log stuff during tests
//!     }
//! }
//!
//! let container: Container<MyModule> = ContainerBuilder::new()
//!     .with_component_override::<dyn IOutput>(Box::new(FakeOutput))
//!     .with_component_parameters::<TodayWriter>(TodayWriterParameters {
//!         today: "Jan 26".to_string(),
//!         year: 2020
//!     })
//!     .build();
//!
//! let writer: &dyn IDateWriter = container.resolve_ref();
//! writer.write_date(); // Nothing will be printed
//! ```
//!
//! ## The full example
//! ```
//! use shaku::{module, Component, Container, ContainerBuilder, Interface};
//! use std::sync::Arc;
//!
//! trait IOutput: Interface {
//!     fn write(&self, content: String);
//! }
//!
//! trait IDateWriter: Interface {
//!     fn write_date(&self);
//! }
//!
//! #[derive(Component)]
//! #[shaku(interface = IOutput)]
//! struct ConsoleOutput;
//!
//! impl IOutput for ConsoleOutput {
//!     fn write(&self, content: String) {
//!         println!("{}", content);
//!     }
//! }
//!
//! #[derive(Component)]
//! #[shaku(interface = IDateWriter)]
//! struct TodayWriter {
//!     #[shaku(inject)]
//!     output: Arc<dyn IOutput>,
//!     today: String,
//!     year: usize,
//! }
//!
//! impl IDateWriter for TodayWriter {
//!     fn write_date(&self) {
//!         self.output.write(format!("Today is {}, {}", self.today, self.year));
//!     }
//! }
//!
//! module! {
//!     MyModule {
//!         components = [ConsoleOutput, TodayWriter],
//!         providers = []
//!     }
//! }
//!
//! let container: Container<MyModule> = ContainerBuilder::new()
//!     .with_component_parameters::<TodayWriter>(TodayWriterParameters {
//!         today: "Jan 26".to_string(),
//!         year: 2020
//!     })
//!     .build();
//!
//! let writer: &dyn IDateWriter = container.resolve_ref();
//! writer.write_date();
//! ```
//!
//! # Crate features
//! By default shaku is thread-safe and exposes derive macros, but these can be disabled by opting
//! out of the following features:
//!
//! - `thread_safe`: Requires components to be `Send + Sync` and provided services to be `Send`
//! - `derive`: Uses the `shaku_derive` crate to provide proc-macro derives of `Component` and
//!   `Provider`.
//!
//! [`Provider`]: trait.Provider.html
//! [`Interface`]: trait.Interface.html
//! [`Component`]: trait.Component.html
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`HasComponent`]: trait.HasComponent.html
//! [`ContainerBuildContext::resolve`]: struct.ContainerBuildContext.html#method.resolve
//! [`Module`]: trait.Module.html
//! [module macro]: macro.module.html
//! [`ContainerBuilder`]: struct.ContainerBuilder.html
//! [`Container`]: struct.Container.html
//! [`with_component_override`]: struct.ContainerBuilder.html#method.with_component_override

// Modules
#[macro_use]
mod trait_alias;
mod component;
mod container;
mod error;
mod module;
mod parameters;
mod provider;

// Reexport derives
#[cfg(feature = "derive")]
pub use {shaku_derive::Component, shaku_derive::Provider};

// Expose a flat module structure
pub use crate::{component::*, container::*, error::*, module::*, provider::*};
