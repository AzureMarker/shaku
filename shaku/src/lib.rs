//! Shaku is a dependency injection library. It can be used directly or through integration with
//! application frameworks such as [Rocket](https://rocket.rs) (see
//! [`shaku_rocket`](https://crates.io/crates/shaku_rocket)).
//!
//! # Getting started
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
//! struct ConsoleOutput;
//!
//! impl IOutput for ConsoleOutput {
//!     fn write(&self, content: String) {
//!         println!("{}", content);
//!     }
//! }
//!
//! trait IDateWriter {
//!     fn write_date(&self);
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
//!         self.output.write(format!("Today is {} {}", self.today, self.year));
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
//! ## Mark structs as Component
//! A component is a struct that implements an interface trait. In our example, we have 2
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
//! # trait IOutput: Interface {
//! #     fn write(&self, content: String);
//! # }
//! #
//! # impl IOutput for ConsoleOutput {
//! #     fn write(&self, content: String) {
//! #         println!("{}", content);
//! #     }
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
//! To express this dependency (when using the derive macro), use the `#[shaku(inject)]` attribute
//! within your struct to declare the property as a
//! [trait object](https://doc.rust-lang.org/book/ch17-02-trait-objects.html) wrapped in an [`Arc`].
//!
//! In our example:
//!
//! ```
//! # use shaku::Interface;
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface {
//! #     fn write(&self, content: String);
//! # }
//! #
//! # trait IDateWriter: Interface {
//! #     fn write_date(&self);
//! # }
//! #
//! # impl IDateWriter for TodayWriter {
//! #     fn write_date(&self) {
//! #         self.output.write(format!("Today is {} {}", self.today, self.year));
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
//! If you don't use the derive macro, return [`Dependency`] objects in [`Component::dependencies`]
//! and inject them manually in [`Component::build`].
//!
//! ## Register components
//! At application startup, create a [`ContainerBuilder`] and register your components with it. It
//! will create a [`Container`] which you can use to resolve components.
//!
//! ```
//! # use shaku::{Component, Interface};
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface {
//! #     fn write(&self, content: String);
//! # }
//! #
//! # impl IOutput for ConsoleOutput {
//! #     fn write(&self, content: String) {
//! #         println!("{}", content);
//! #     }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IOutput)]
//! # struct ConsoleOutput;
//! #
//! use shaku::ContainerBuilder;
//!
//! let mut builder = ContainerBuilder::new();
//! builder.register_type::<ConsoleOutput>();
//!
//! let container = builder.build().unwrap();
//! ```
//!
//! ### Passing parameters
//! In many cases you need to pass parameters to a component. This can be done when
//! registering a component into a [`ContainerBuilder`].
//!
//! You can register parameters either using their property name or their property type. In the
//! latter case, you need to ensure that it is unique.
//!
//! Passing parameters is done using the [`with_named_parameter`] or [`with_typed_parameter`]
//! chained methods:
//!
//! ```
//! # use shaku::{Component, ContainerBuilder, Interface};
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface {
//! #     fn write(&self, content: String);
//! # }
//! #
//! # trait IDateWriter: Interface {
//! #     fn write_date(&self);
//! # }
//! #
//! # impl IDateWriter for TodayWriter {
//! #     fn write_date(&self) {
//! #         self.output.write(format!("Today is {} {}", self.today, self.year));
//! #     }
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
//! #
//! # let mut builder = ContainerBuilder::new();
//! builder
//!     .register_type::<TodayWriter>()
//!     .with_named_parameter("today", "Jan 26".to_string())
//!     .with_typed_parameter::<usize>(2020);
//! ```
//!
//! ## Resolve components
//! During application execution, you’ll need to make use of the components you registered. You do
//! this by resolving them from a [`Container`] with one of `resolve` methods.
//!
//! Here's how we can print the date in our exmaple:
//!
//! ```
//! # use shaku::{Component, ContainerBuilder, Interface};
//! # use std::sync::Arc;
//! #
//! # trait IOutput: Interface {
//! #     fn write(&self, content: String);
//! # }
//! #
//! # impl IOutput for ConsoleOutput {
//! #     fn write(&self, content: String) {
//! #         println!("{}", content);
//! #     }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = IOutput)]
//! # struct ConsoleOutput;
//! #
//! # trait IDateWriter: Interface {
//! #     fn write_date(&self);
//! # }
//! #
//! # impl IDateWriter for TodayWriter {
//! #     fn write_date(&self) {
//! #         self.output.write(format!("Today is {} {}", self.today, self.year));
//! #     }
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
//! #
//! # let mut builder = ContainerBuilder::new();
//! # builder.register_type::<ConsoleOutput>();
//! # builder
//! #     .register_type::<TodayWriter>()
//! #     .with_named_parameter("today", "Jan 26".to_string())
//! #     .with_typed_parameter::<usize>(2020);
//! # let container = builder.build().unwrap();
//! #
//! let writer: &dyn IDateWriter = container.resolve_ref().unwrap();
//! writer.write_date();
//! ```
//!
//! Now when you run your program...
//!
//! - The components and their parameters will be registered in the [`ContainerBuilder`].
//! - `builder.build()` will create the registered components in order of dependency
//!   (first `ConsoleOutput`, then `TodayWriter`). These components will be stored in the
//!   [`Container`].
//! - The `resolve_ref()` method asks the [`Container`] for an `IDateWriter`.
//! - The [`Container`] sees that `IDateWriter` maps to `TodayWriter`, and it returns the component.
//!
//! Later, if we wanted our application to write output in a different way, we would just have to
//! implement a different `IOutput` and then change the registration at app startup. We won’t have
//! to change any other code. Yay, inversion of control!
//!
//! ## The full example
//! ```
//! use shaku::{Component, ContainerBuilder, Interface};
//! use std::sync::Arc;
//!
//! trait IOutput: Interface {
//!     fn write(&self, content: String);
//! }
//!
//! impl IOutput for ConsoleOutput {
//!     fn write(&self, content: String) {
//!         println!("{}", content);
//!     }
//! }
//!
//! #[derive(Component)]
//! #[shaku(interface = IOutput)]
//! struct ConsoleOutput;
//!
//! trait IDateWriter: Interface {
//!     fn write_date(&self);
//! }
//!
//! impl IDateWriter for TodayWriter {
//!     fn write_date(&self) {
//!         self.output.write(format!("Today is {} {}", self.today, self.year));
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
//! let mut builder = ContainerBuilder::new();
//! builder.register_type::<ConsoleOutput>();
//! builder
//!     .register_type::<TodayWriter>()
//!     .with_named_parameter("today", "Jan 26".to_string())
//!     .with_typed_parameter::<usize>(2020);
//! let container = builder.build().unwrap();
//!
//! let writer: &dyn IDateWriter = container.resolve_ref().unwrap();
//! writer.write_date();
//! ```
//!
//! [`Interface`]: component/trait.Interface.html
//! [`Component`]: component/trait.Component.html
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`Dependency`]: container/struct.Dependency.html
//! [`Component::dependencies`]: component/trait.Component.html#tymethod.dependencies
//! [`Component::build`]: component/trait.Component.html#tymethod.build
//! [`ContainerBuilder`]: container/struct.ContainerBuilder.html
//! [`Container`]: container/struct.Container.html
//! [`with_named_parameter`]: container/struct.RegisteredType.html#method.with_named_parameter
//! [`with_typed_parameter`]: container/struct.RegisteredType.html#method.with_typed_parameter

// Linting
#![deny(unused_must_use)]

// Modules
#[macro_use]
mod trait_alias;

pub mod component;
pub mod container;
pub mod parameter;

// Reexport Component derive
#[cfg(feature = "derive")]
pub use shaku_derive::Component;

// Reexport Error type from shaku_internals
pub use shaku_internals::error::Error;

/// Alias for a `Result` with the error type [shaku::Error](enum.Error.html)
pub type Result<T> = std::result::Result<T, shaku_internals::error::Error>;

// Shortcut to main types / traits
pub use crate::component::Component;
pub use crate::component::Interface;
pub use crate::container::Container;
pub use crate::container::ContainerBuildContext;
pub use crate::container::ContainerBuilder;
pub use crate::container::Dependency;
