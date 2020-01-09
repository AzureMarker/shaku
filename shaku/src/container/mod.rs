//! `ContainerBuilder` and `Container` structs used respectively to register and resolve
//! Components.
//!
//! # Application startup
//! At application startup, you need to create a
//! [ContainerBuilder](struct.ContainerBuilder.html#method.new) and register your components with
//! it.
//!
//! ```rust,ignore
//! let mut builder = shaku::ContainerBuilder::new();
//!
//! // Register `FooImpl` as a `Foo` Component
//! // Requires that `Foo` was marked as a Component using the `#[derive(Component)]` macro
//! // and `#[interface(Foo)]`
//! builder.register_type::<FooImpl>();
//! ```
//!
//! Once you are done registering all your components,
//! use [ContainerBuilder::build()](struct.ContainerBuilder.html#method.build)
//! to create the [Container](struct.Container.html) instance that will allow you to
//! resolve the components you registered from the Container instance. The component instances
//! themselves will be created during the container build step, so watch out for configuration
//! errors during that step.
//!
//! # Application execution
//! During application execution, you’ll need to make use of the components you registered.
//! You do this by resolving them from a `Container` with one of the following `resolve` method:
//!
//! - [Container::resolve()](struct.Container.html#method.resolve): get a shared ownership reference
//!   (`Arc`) to the component.
//! - [Container::resolve_ref()](struct.Container.html#method.resolve_ref): get a normal reference
//!   (`&dyn`) to the component.
//! - [Container::resolve_mut()](struct.Container.html#method.resolve_mut): same as `resolve_ref()`
//!   but returns a mutable reference (`&dyn mut`).
//!
//! # Passing parameters
//! Passing parameters can be done when registering (i.e. when calling
//! [ContainerBuilder::register()](struct.ContainerBuilder.html#method.register_type)), just chain
//! a `with_name_parameter()` or `with_type_parameter()` call.
//!
//! ```rust,ignore
//! builder
//!     .register_type::<FooImpl>()
//!     .with_named_parameter("name", "fooooo".to_string());
//! //  .with_type_parameter::<String>("fooooo".to_string()); // alternative
//! ```

pub use self::container_builder::*;
pub use self::dependency::Dependency;
pub use self::map_container::Container;
pub use self::registered_type::RegisteredType;

mod container_builder;
mod dependency;
mod map_container;
mod registered_type;

#[cfg(not(feature = "thread_safe"))]
type AnyType = dyn anymap::any::Any;
#[cfg(feature = "thread_safe")]
type AnyType = dyn anymap::any::Any + Send;

type Map = anymap::Map<AnyType>;
