//! This module handles registering, building, and resolving services.
//!
//! # Application startup
//! At application startup, create a [`ContainerBuilder`] and register your services with it.
//!
//! ```
//! # use shaku::{
//! #     Component, ContainerBuilder, Interface, ProvidedInterface, Provider, Container, Error,
//! #     Dependency
//! # };
//! #
//! # trait Foo: Interface {}
//! # impl Foo for FooImpl {}
//! #
//! # trait Bar: ProvidedInterface {}
//! # impl Bar for BarImpl {}
//! #
//! #[derive(Component)]
//! #[shaku(interface = Foo)]
//! struct FooImpl;
//!
//! #[derive(Provider)]
//! #[shaku(interface = Bar)]
//! struct BarImpl;
//!
//! let mut builder = ContainerBuilder::new();
//!
//! // Register `FooImpl` as a `Foo` component
//! builder.register_type::<FooImpl>();
//! // Register `BarImpl` as a provider of `Bar` services
//! builder.register_provider::<BarImpl>();
//! ```
//!
//! Once you are done registering all your services, use [`ContainerBuilder::build`] to create
//! the [`Container`] instance that will allow you to resolve the services. The component
//! instances themselves will be created during [`ContainerBuilder::build`], and dependencies for
//! all services will be verified, so check the result for configuration errors.
//!
//! # Resolving services
//! During application execution, you’ll need to make use of the services you registered.
//! You do this by resolving them from a [`Container`] with one of the following methods:
//!
//! - [`resolve`]\: get a shared ownership reference (`Arc`) to the component.
//! - [`resolve_ref`]\: get a normal reference (`&dyn`) to the component.
//! - [`resolve_mut`]\: same as `resolve_ref` but returns a mutable reference
//!   (`&dyn mut`).
//! - [`provide`]\: create a provided service (wrapped in a `Box`)
//!
//! # Passing parameters
//! Passing parameters can be done when registering components, just chain a
//! [`with_named_parameter`] or [`with_typed_parameter`] call.
//!
//! ```
//! # use shaku::{Component, ContainerBuilder, Interface};
//! #
//! # trait Foo: Interface {}
//! # impl Foo for FooImpl {}
//! #
//! # #[derive(Component)]
//! # #[shaku(interface = Foo)]
//! # struct FooImpl;
//! #
//! # let mut builder = ContainerBuilder::new();
//! #
//! builder
//!     .register_type::<FooImpl>()
//!     .with_named_parameter("name", "foo".to_string());
//! //  .with_typed_parameter::<String>("foo".to_string()); // alternative
//! ```
//!
//! [`ContainerBuilder`]: struct.ContainerBuilder.html
//! [`Container`]: struct.Container.html
//! [`ContainerBuilder::build`]: struct.ContainerBuilder.html#method.build
//! [`resolve`]: struct.Container.html#method.resolve
//! [`resolve_ref`]: struct.Container.html#method.resolve_ref
//! [`resolve_mut`]: struct.Container.html#method.resolve_mut
//! [`provide`]: struct.Container.html#method.provide
//! [`with_named_parameter`]: struct.ComponentRegistration.html#method.with_named_parameter
//! [`with_typed_parameter`]: struct.ComponentRegistration.html#method.with_typed_parameter

mod component_registration;
#[allow(clippy::module_inception)]
mod container;
mod container_build_context;
mod container_builder;
mod dependency;
mod provider_registration;

pub use self::component_registration::ComponentRegistration;
pub use self::container::Container;
pub use self::container_build_context::ContainerBuildContext;
pub use self::container_builder::ContainerBuilder;
pub use self::dependency::Dependency;

#[cfg(not(feature = "thread_safe"))]
type AnyType = dyn anymap::any::Any;
#[cfg(feature = "thread_safe")]
type AnyType = dyn anymap::any::Any + Send + Sync;

type ComponentMap = anymap::Map<AnyType>;
