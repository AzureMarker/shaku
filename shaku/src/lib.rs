// <click to unfold doc>
    //! # What is Dependency Injection (aka. Dependency Inversion)?
    //! 
    //! The idea behind inversion of control is that, rather than tie the classes in your application together and let classes “new up” their dependencies, you switch it around so dependencies are instead passed in during class construction. It's one of the 5 core principles of [SOLID programming](https://en.wikipedia.org/wiki/SOLID_(object-oriented_design))
    //!
    //! If you want to read more on that:
    //! <ul>
    //! <li> [Martin Fowler has an excellent article explaining dependency injection/inversion of control](http://martinfowler.com/articles/injection.html)
    //! <li> [Wikipedia article on dependency inversion principle](https://en.wikipedia.org/wiki/Dependency_inversion_principle)
    //! </ul>
    //! 
    //! # Getting started
    //! ## Structure your application
    //! Start by writing a classical application with struct & types (in homage to [AutoFac] (https://autofac.org/) I ported their classical "getting started" example).
    //! Code excerpts are used below to illustrate this little guide, the complete example is available [here](https://github.com/bgbahoue/he-di/blob/master/examples/autofac/src/main.rs).
    //! 
    //! ```rust
    //! trait IOutput : Send {
    //!     fn write(&self, content: String);
    //! }
    //! 
    //! struct ConsoleOutput {
    //!     prefix: String,
    //!     other_param: usize,
    //! }
    //! 
    //! impl IOutput for ConsoleOutput {
    //!     fn write(&self, content: String) {
    //!         println!("{} #{} {}", self.prefix, self.other_param, content);
    //!     }
    //! }
    //! 
    //! trait IDateWriter : Send {
    //!     fn write_date(&self);
    //! }
    //! 
    //! struct TodayWriter {
    //!     output: Box<IOutput>,
    //!     today: String,
    //!     year: String,
    //! }
    //! 
    //! impl IDateWriter for TodayWriter {
    //!     fn write_date(&self) {
    //!        let mut content = "Today is ".to_string();
    //!        content.push_str(self.today.as_str());
    //!        content.push_str(" ");
    //!        content.push_str(self.year.to_string().as_str());
    //!        self.output.write(content);
    //!     }
    //! }
    //! ```
    //! 
    //! ## Mark structs as Component
    //! A component is an expression or other bit of code that exposes one or more services and can take in other dependencies.
    //! 
    //! In our example, we have 2 components:
    //! 
    //! - `TodayWriter` of type `IDateWriter` 
    //! - `ConsoleOutput` of type `IOutput` 
    //! 
    //! To be able to identify them as components [shaku](https://crates.io/crates/shaku) exposes a `#[derive()]` macro (though the [shaku_derive](https://crates.io/crates/shaku_derive) crate).
    //! It is simply done using the following attributes:
    //!
    //! ```rust,ignore
    //! #[derive(Component)] // <--- mark as a Component
    //! #[interface(IOutput)] // <--- specify the type of this Component
    //! struct ConsoleOutput {
    //!     prefix: String,
    //!     other_param: usize,
    //! }
    //! ```
    //! 
    //! In the current version, you alos need to specify the type of your Component using the `#[interface()]` attribute.
    //! 
    //! ## Express dependencies
    //! Some components can have dependencies to other components, which allows the DI logic to also inject these components with another Component.
    //! 
    //! In our example, `ConsoleOuput` is a Component with no dependency and `TodayWriter` a Component with a dependency to a `IOutput` Component. 
    //! 
    //! To express this dependency, use the `#[inject]` attribute within your struct to flag the property and declare the property as a [trait object](https://doc.rust-lang.org/book/first-edition/trait-objects.html).
    //!
    //! In our example:
    //! 
    //! ```rust,ignore
    //! use shaku_derive::Component;
    //!
    //! #[derive(Component)] // <--- mark a struct as a Component that can be registered & resolved
    //! #[interface(IDateWriter)] // <--- specify which interface it implements
    //! struct TodayWriter {
    //!     #[inject] // <--- flag 'output' as a property which can be injected
    //!     output: Box<IOutput>, // <--- trait object using the interface `IOutput`
    //!     today: String,
    //!     year: usize,
    //! }
    //! ```
    //! 
    //! ## Application startup
    //! At application startup, you need to create a [ContainerBuilder](struct.ContainerBuilder.html) and register your components with it. 
    //! 
    //! In our example, we register `ConsoleOutput` and `TodayWriter` with a `ContainerBuilder` doing something like this:
    //! 
    //! ```rust,ignore
    //! // Create your builder.
    //! let mut builder = ContainerBuilder::new();
    //! 
    //! builder
    //!     .register_type::<ConsoleOutput>()
    //!     .as_type::<IOutput>();
    //! 
    //! builder
    //!     .register_type::<TodayWriter>()
    //!     .as_type::<IDateWriter>();
    //! 
    //! // Create a Container
    //! let mut container = builder.build().unwrap();
    //! ```
    //! 
    //! The `Container` reference is what you will use to resolve types & components later. It can then be stored as you see fit.
    //! 
    //! ## Application execution
    //! During application execution, you’ll need to make use of the components you registered. You do this by resolving them from a `Container` with one of the 3 `resolve()` methods.
    //! 
    //! ### Passing parameters
    //! In most cases you need to pass parameters to a Component. This can be done either when registring a Component into a [ContainerBuilder](struct.ContainerBuilder.html) or when resolving a Component from a [Container](struct.Container.html).
    //!
    //! You can register parameters either using their property name or their property type. In the later case, you need to ensure that it is unique.
    //! 
    //! #### When registering components
    //! Passing parameters at registration time is done using the `with_named_parameter()` or `with_typed_parameter()` chained methods like that:
    //!
    //! ```rust,ignore
    //! builder
    //!     .register_type::<ConsoleOutput>()
    //!     .as_type::<IOutput>()
    //!     .with_named_parameter("prefix", "PREFIX >".to_string())
    //!     .with_typed_parameter::<usize>(117 as usize);
    //! ```
    //! 
    //! #### When resolving components
    //! Passing parameters at resolve time uses the same `with_named_parameter()` or `with_typed_parameter()` methods from your [Container](struct.Container.html) instance.
    //!
    //! For our sample app, we created a `write_date()` method to resolve the writer from a Container and illustrate how to pass parameters with its name or type:
    //!
    //! ```rust,ignore
    //! fn write_date(container: &mut Container) {
    //!     let writer = container
    //!         .with_typed_parameter::<IDateWriter, String>("June 20".to_string())
    //!         .with_named_parameter::<IDateWriter, usize>("year", 2017 as usize)
    //!         .resolve::<IDateWriter>()
    //!         .unwrap();
    //!     writer.write_date();
    //! }
    //! ```
    //! 
    //! Now when you run your program...
    //! 
    //! - The `write_date()` method asks shaku for an `IDateWriter`.
    //! - shaku sees that `IDateWriter` maps to `TodayWriter` so starts creating a `TodayWriter`.
    //! - shaku sees that the `TodayWriter` needs an `IOutput` in its constructor.
    //! - shaku sees that `IOutput` maps to `ConsoleOutput` so creates a new `ConsoleOutput` instance.
    //! - Since `ConsoleOutput` doesn't have any more dependency, it uses this instance to finish constructing the `TodayWriter`.
    //! - shaku returns the fully-constructed `TodayWriter` for `write_date()` to use.
    //! 
    //! Later, if we wanted our application to write a different date, we would just have to implement a different `IDateWriter` and then change the registration at app startup. We won’t have to change any other classes. Yay, inversion of control!
    //! 
    //! ## Roadmap
    //! The current implementation of this crate is still WIP. A few identified usefull to know limitations (being further explorer) are:
    //!
    //! - `#[derive(Component)]` should be tested against complex cases & more tests are to be written (e.g, struct with lifetime, generics, ...)
    //! - we should support closures as a way to create parameters (at register or resolve time)

// Linting
#![deny(unused_must_use)]
#![allow(clippy::new_without_default)]

// Reexport of [anymap](https://crates.io/crates/anymap)
#[doc(hidden)]
pub extern crate anymap;
#[macro_use] extern crate log;

// Reexport Error type from shaku_internals
pub use shaku_internals::error::Error;

#[doc(hidden)]
pub use crate::component::Component;
#[doc(hidden)]
pub use crate::component::ComponentBuilderImpl;
// Shortcut to main types / traits
pub use crate::container::Container;
pub use crate::container::ContainerBuilder;
pub use crate::result::Result;

// Hide modules from public API
pub mod container;
#[doc(hidden)]
pub mod component;
pub mod parameter;

// Main DI Result type mapping
#[doc(hidden)]
pub mod result {
    /// Alias for a `Result` with the error type [shaku::Error](enum.Error.html)
    pub type Result<T> = ::std::result::Result<T, shaku_internals::error::Error>;
}
