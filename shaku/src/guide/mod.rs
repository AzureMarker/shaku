//! # Getting started guide
//! Note: This getting started guide focuses on components, which live for the lifetime of the
//! application (or, technically, the module). After reading this getting started guide, check
//! out the [provider guide] to learn how to create services with shorter lifetimes.
//!
//! ## Structure your application
//! Start with your application's structs and traits. Use `Arc<dyn T>` for dependencies.
//!
//! ```
//! use std::sync::Arc;
//!
//! trait Logger {
//!     fn log(&self, content: &str);
//! }
//!
//! trait DateLogger {
//!     fn log_date(&self);
//! }
//!
//! struct LoggerImpl;
//!
//! impl Logger for LoggerImpl {
//!     fn log(&self, content: &str) {
//!         println!("{}", content);
//!     }
//! }
//!
//! struct DateLoggerImpl {
//!     logger: Arc<dyn Logger>,
//!     today: String,
//!     year: usize,
//! }
//!
//! impl DateLogger for DateLoggerImpl {
//!     fn log_date(&self) {
//!         self.logger.log(&format!("Today is {}, {}", self.today, self.year));
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
//! trait Logger: Interface {
//!     fn log(&self, content: &str);
//! }
//!
//! trait DateLogger: Interface {
//!     fn log_date(&self);
//! }
//! ```
//!
//! ## Implement Component
//! A component is a struct that implements an [`Interface`] trait. In our example, we have 2
//! components:
//!
//! - `DateLoggerImpl` of type `DateLogger`
//! - `LoggerImpl` of type `Logger`
//!
//! These components must implement [`Component`], which can either be done manually or through a
//! derive macro (using the `derive` feature):
//!
//! ```
//! # use shaku::Interface;
//! #
//! # trait Logger: Interface { fn log(&self, content: &str); }
//! #
//! # impl Logger for LoggerImpl {
//! #     fn log(&self, content: &str) { println!("{}", content); }
//! # }
//! #
//! use shaku::Component;
//!
//! #[derive(Component)]
//! #[shaku(interface = Logger)]
//! struct LoggerImpl;
//! ```
//!
//! ## Express dependencies
//! Components can depend on other components. In our example, `DateLoggerImpl` requires an `Logger`
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
//! # trait Logger: Interface { fn log(&self, content: &str); }
//! # trait DateLogger: Interface { fn log_date(&self); }
//! #
//! # impl DateLogger for DateLoggerImpl {
//! #     fn log_date(&self) {
//! #         self.logger.log(&format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! use shaku::Component;
//!
//! #[derive(Component)]
//! #[shaku(interface = DateLogger)]
//! struct DateLoggerImpl {
//!     #[shaku(inject)]
//!     logger: Arc<dyn Logger>,
//!     today: String,
//!     year: usize,
//! }
//! ```
//!
//! If you don't use the derive macro, add [`HasComponent`] bounds to your module generic and inject
//! the dependencies manually with [`HasComponent::build_component`].
//!
//! ## Define a module
//! Modules link together components and providers, and are core to providing shaku's compile time
//! guarentees. A [`Module`] can be defined manually or via the [`module`][module macro] macro
//! (using the `derive` feature):
//!
//! ```
//! # use shaku::{Component, Interface};
//! # use std::sync::Arc;
//! #
//! # trait Logger: Interface { fn log(&self, content: &str); }
//! # trait DateLogger: Interface { fn log_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = Logger)]
//! # struct LoggerImpl;
//! # impl Logger for LoggerImpl {
//! #     fn log(&self, content: &str) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = DateLogger)]
//! # struct DateLoggerImpl {
//! #     #[shaku(inject)]
//! #     logger: Arc<dyn Logger>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl DateLogger for DateLoggerImpl {
//! #     fn log_date(&self) {
//! #         self.logger.log(&format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! # fn main() {}
//! #
//! use shaku::module;
//!
//! module! {
//!     MyModule {
//!         components = [LoggerImpl, DateLoggerImpl],
//!         providers = []
//!     }
//! }
//! ```
//!
//! This module implements `HasComponent<dyn Logger>` and `HasComponent<dyn DateLogger>` using the
//! provided component implementations.
//!
//! ## Build the module
//! At application startup, start building the module using the generated `builder` method (created
//! by the [`module`][module macro] macro). Alternatively, use [`ModuleBuilder::with_submodules`] to
//! create the builder. Then, call [`ModuleBuilder::build`] to get the module instance.
//!
//! ```
//! # use shaku::{module, Component, Interface};
//! # use std::sync::Arc;
//! #
//! # trait Logger: Interface { fn log(&self, content: &str); }
//! # trait DateLogger: Interface { fn log_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = Logger)]
//! # struct LoggerImpl;
//! # impl Logger for LoggerImpl {
//! #     fn log(&self, content: &str) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = DateLogger)]
//! # struct DateLoggerImpl {
//! #     #[shaku(inject)]
//! #     logger: Arc<dyn Logger>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl DateLogger for DateLoggerImpl {
//! #     fn log_date(&self) {
//! #         self.logger.log(&format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! # module! {
//! #     MyModule {
//! #         components = [LoggerImpl, DateLoggerImpl],
//! #         providers = []
//! #     }
//! # }
//! # fn main() {
//! let module = MyModule::builder().build();
//! # }
//! ```
//!
//! ### Passing parameters
//! In many cases you need to pass parameters to a component. This can be done during module
//! creation. Each component has an associated parameters type, and the derive generates a
//! `*Parameters` struct for you (named after the component struct). Use this struct to pass in the
//! parameters.
//!
//! Note that if you don't pass in parameters, the parameters' default values will be used. You can
//! override the default value by annotating the property with `#[shaku(default = ...)]`. If the
//! parameter should not have a default value, annotate it with `#[shaku(no_default)]`. This will
//! cause module creation to panic if no value is provided for the parameter.
//!
//! ```
//! # use shaku::{module, Component, Interface};
//! # use std::sync::Arc;
//! #
//! # trait Logger: Interface { fn log(&self, content: &str); }
//! # trait DateLogger: Interface { fn log_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = Logger)]
//! # struct LoggerImpl;
//! # impl Logger for LoggerImpl {
//! #     fn log(&self, content: &str) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = DateLogger)]
//! # struct DateLoggerImpl {
//! #     #[shaku(inject)]
//! #     logger: Arc<dyn Logger>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl DateLogger for DateLoggerImpl {
//! #     fn log_date(&self) {
//! #         self.logger.log(&format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! # module! {
//! #     MyModule {
//! #         components = [LoggerImpl, DateLoggerImpl],
//! #         providers = []
//! #     }
//! # }
//! #
//! # fn main() {
//! let module = MyModule::builder()
//!     .with_component_parameters::<DateLoggerImpl>(DateLoggerImplParameters {
//!         today: "Jan 26".to_string(),
//!         year: 2020
//!     })
//!     .build();
//! # }
//! ```
//!
//! ## Resolve components
//! Once you created the module, you can resolve the components using the module's [`HasComponent`]
//! methods.
//!
//! ```
//! # use shaku::{module, Component, Interface};
//! # use std::sync::Arc;
//! #
//! # trait Logger: Interface { fn log(&self, content: &str); }
//! # trait DateLogger: Interface { fn log_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = Logger)]
//! # struct LoggerImpl;
//! # impl Logger for LoggerImpl {
//! #     fn log(&self, content: &str) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = DateLogger)]
//! # struct DateLoggerImpl {
//! #     #[shaku(inject)]
//! #     logger: Arc<dyn Logger>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl DateLogger for DateLoggerImpl {
//! #     fn log_date(&self) {
//! #         self.logger.log(&format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! # module! {
//! #     MyModule {
//! #         components = [LoggerImpl, DateLoggerImpl],
//! #         providers = []
//! #     }
//! # }
//! #
//! # fn main() {
//! #     let module = MyModule::builder()
//! #         .with_component_parameters::<DateLoggerImpl>(DateLoggerImplParameters {
//! #             today: "Jan 26".to_string(),
//! #             year: 2020
//! #         })
//! #         .build();
//! #
//! use shaku::HasComponent;
//!
//! let date_logger: &dyn DateLogger = module.resolve_ref();
//! date_logger.log_date(); // Prints "Today is Jan 26, 2020"
//! # }
//! ```
//!
//! ## Overriding components
//! Although shaku is a compile time DI library, you can override the implementation of a service
//! during the module build. This can be useful during testing, for example using an in-memory
//! database while doing integration tests. For components, simply pass in a struct instance which
//! implements the interface you want to override to [`with_component_override`]\:
//!
//! ```
//! # use shaku::{module, Component, Interface, HasComponent};
//! # use std::sync::Arc;
//! #
//! # trait Logger: Interface { fn log(&self, content: &str); }
//! # trait DateLogger: Interface { fn log_date(&self); }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = Logger)]
//! # struct LoggerImpl;
//! # impl Logger for LoggerImpl {
//! #     fn log(&self, content: &str) { println!("{}", content); }
//! # }
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = DateLogger)]
//! # struct DateLoggerImpl {
//! #     #[shaku(inject)]
//! #     logger: Arc<dyn Logger>,
//! #     today: String,
//! #     year: usize,
//! # }
//! # impl DateLogger for DateLoggerImpl {
//! #     fn log_date(&self) {
//! #         self.logger.log(&format!("Today is {}, {}", self.today, self.year));
//! #     }
//! # }
//! #
//! # module! {
//! #     MyModule {
//! #         components = [LoggerImpl, DateLoggerImpl],
//! #         providers = []
//! #     }
//! # }
//! #
//! #[derive(Component)]
//! #[shaku(interface = Logger)]
//! struct FakeOutput;
//!
//! impl Logger for FakeOutput {
//!     fn log(&self, _content: &str) {
//!         // We don't want to actually log stuff during tests
//!     }
//! }
//!
//! # fn main() {
//! let module = MyModule::builder()
//!     .with_component_override::<dyn Logger>(Box::new(FakeOutput))
//!     .with_component_parameters::<DateLoggerImpl>(DateLoggerImplParameters {
//!         today: "Jan 26".to_string(),
//!         year: 2020
//!     })
//!     .build();
//!
//! let date_logger: &dyn DateLogger = module.resolve_ref();
//! date_logger.log_date(); // Nothing will be printed
//! # }
//! ```
//!
//! ## The full example
//! ```
//! use shaku::{module, Component, Interface, HasComponent};
//! use std::sync::Arc;
//!
//! trait Logger: Interface {
//!     fn log(&self, content: &str);
//! }
//!
//! trait DateLogger: Interface {
//!     fn log_date(&self);
//! }
//!
//! #[derive(Component)]
//! #[shaku(interface = Logger)]
//! struct LoggerImpl;
//!
//! impl Logger for LoggerImpl {
//!     fn log(&self, content: &str) {
//!         println!("{}", content);
//!     }
//! }
//!
//! #[derive(Component)]
//! #[shaku(interface = DateLogger)]
//! struct DateLoggerImpl {
//!     #[shaku(inject)]
//!     logger: Arc<dyn Logger>,
//!     today: String,
//!     year: usize,
//! }
//!
//! impl DateLogger for DateLoggerImpl {
//!     fn log_date(&self) {
//!         self.logger.log(&format!("Today is {}, {}", self.today, self.year));
//!     }
//! }
//!
//! module! {
//!     MyModule {
//!         components = [LoggerImpl, DateLoggerImpl],
//!         providers = []
//!     }
//! }
//!
//! fn main() {
//!     let module = MyModule::builder()
//!         .with_component_parameters::<DateLoggerImpl>(DateLoggerImplParameters {
//!             today: "Jan 26".to_string(),
//!             year: 2020
//!         })
//!         .build();
//!
//!     let date_logger: &dyn DateLogger = module.resolve_ref();
//!     date_logger.log_date();
//! }
//! ```
//!
//! [provider guide]: provider/index.html
//! [`Interface`]: ../trait.Interface.html
//! [`Component`]: ../trait.Component.html
//! [`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [`HasComponent`]: ../trait.HasComponent.html
//! [`HasComponent::build_component`]: ../trait.HasComponent.html#tymethod.build_component
//! [`Module`]: ../trait.Module.html
//! [module macro]: ../macro.module.html
//! [`ModuleBuilder::with_submodules`]: ../struct.ModuleBuilder.html#method.with_submodules
//! [`ModuleBuilder::build`]: ../struct.ModuleBuilder.html#method.build
//! [`with_component_override`]: ../struct.ModuleBuilder.html#method.with_component_override

pub mod provider;
pub mod submodules;
