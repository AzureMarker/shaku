//! `ContainerBuilder` and `Container` structs used respectively to register and resolve
//! Components.
//!
//! # Application startup
//! At application startup, you need to create a [ContainerBuilder](struct.ContainerBuilder.html#method.new) and register your components with it.
//!
//! ```rust,ignore
//! let mut builder = shaku::ContainerBuilder::new();
//!
//! // Register `FooImpl` as a `Foo` Component
//! // Requires that `Foo` was marked as a Component using the `#[derive(Component)]` macro
//! builder
//!     .register_type::<FooImpl>()
//!     .as_type::<Foo>()
//! ```
//!
//! `register()` accepts any Trait or Struct (`?Sized` bound) but is meant to
//! be used to register a Component i.e. a `Sized` struct as being an implementation of a
//! given interface set in the `as_type()` chained method.
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
//! - [Container::resolve()](struct.Container.html#method.resolve): remove the Component from the Container and return it (same as a `HashMap::remove()` method)
//! - [Container::resolve_ref()](struct.Container.html#method.resolve_ref): instantiate a Component from the Container and return a reference to it (same as a `HashMap::get()` method). The Component is not consumed and you can call `resolve_ref()` or `resolve_mut()` as many times as you want, you'll get a reference to the same object every time. Calling `resolve()` after a `resolve_ref()` or `resolve_mut()` returns the object but prevents any further calls to `resolve_ref()`.
//! - [Container::resolve_mut()](struct.Container.html#method.resolve_mut): same as `resolve_ref()` but returns a mutable reference (same as `HashMap::get_mut()`)
//!
//! # Passing parameters
//! Passing parameters can be done when registering (i.e. when calling [ContainerBuilder::register()](struct.ContainerBuilder.html#method.register_type)) or when resolving a Component (i.e. when calling [Container::resolve()](struct.Container.html#method.resolve), [Container::resolve_ref()](struct.Container.html#method.resolve_ref) or [Container::resolve_mut()](struct.Container.html#method.resolve_mut))
//!
//! In both case you just have to chain a `with_name_parameter()` or `with_type_parameter()` call.
//!
//! ## When registering a Component
//!
//! ```rust,ignore
//!     builder
//!         .register_type::<FooImpl>()
//!         .as_type::<Foo>()
//!         .with_named_parameter("name", "fooooo".to_string());
//!     //  .with_type_parameter::<String>("fooooo".to_string()); // alternative
//! # }
//! ```
//!
//! ## When resolving a Component
//!
//! ```rust,ignore
//!     let foo = container
//!         .with_named_parameter::<Foo, String>("name", "fooooooo".to_string())
//!     //  .with_typed_parameter::<Foo, String>("fooooooo".to_string()) // alternative
//!         .resolve::<Foo>()
//!         .unwrap();
//! # }
//! ```

pub use self::container_builder::*;
pub use self::map_container::Container;
pub use self::registered_type::RegisteredType;

mod map_container;
mod container_builder;
mod registered_type;

