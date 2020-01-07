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
//! resolve the components you registered from the Container instance.
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
//! [ContainerBuilder::register()](struct.ContainerBuilder.html#method.register_type)) or when
//! resolving a Component (i.e. when calling
//! [Container::resolve()](struct.Container.html#method.resolve),
//! [Container::resolve_ref()](struct.Container.html#method.resolve_ref) or
//! [Container::resolve_mut()](struct.Container.html#method.resolve_mut))
//!
//! In both case you just have to chain a `with_name_parameter()` or `with_type_parameter()` call.
//!
//! ## When registering a Component
//!
//! ```rust,ignore
//! builder
//!     .register_type::<FooImpl>()
//!     .with_named_parameter("name", "fooooo".to_string());
//! //  .with_type_parameter::<String>("fooooo".to_string()); // alternative
//! ```
//!
//! ## When resolving a Component
//! Note: The component must not have been resolved beforehand, or the new
//! parameters will be ignored.
//!
//! ```rust,ignore
//! let foo = container
//!     .with_named_parameter::<dyn Foo, String>("name", "fooooooo".to_string())
//! //  .with_typed_parameter::<dyn Foo, String>("fooooooo".to_string()) // alternative
//!     .resolve::<dyn Foo>()
//!     .unwrap();
//! ```

pub use self::container_builder::*;
pub use self::map_container::Container;
pub use self::registered_type::RegisteredType;

mod container_builder;
mod map_container;
mod registered_type;

#[cfg(not(feature = "thread_safe"))]
type Map = anymap::Map<dyn anymap::any::Any>;
#[cfg(feature = "thread_safe")]
type Map = anymap::Map<dyn anymap::any::Any + Send>;
