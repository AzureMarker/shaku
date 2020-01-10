use std::ops::Deref;

use rocket::request::FromRequest;
use rocket::{http::Status, Outcome, Request, State};

use shaku::component::Interface;
use shaku::Container;

/// Used to retrieve a reference to a component from a shaku `Container`.
/// The container should be stored in Rocket's state. Use this struct as a
/// request guard.
///
/// # Example
/// ```rust
/// #![feature(proc_macro_hygiene, decl_macro)]
///
/// #[macro_use] extern crate rocket;
///
/// use shaku::{ContainerBuilder, Interface};
/// use shaku_derive::Component;
/// use shaku_rocket::Inject;
///
/// trait HelloWorld: Interface {
///     fn greet(&self) -> String;
/// }
///
/// #[derive(Component)]
/// #[interface(HelloWorld)]
/// struct HelloWorldImpl;
///
/// impl HelloWorld for HelloWorldImpl {
///     fn greet(&self) -> String {
///         "Hello, world!".to_owned()
///     }
/// }
///
/// #[get("/")]
/// fn hello(hello_world: Inject<dyn HelloWorld>) -> String {
///     hello_world.greet()
/// }
///
/// fn main() {
///     let mut builder = ContainerBuilder::new();
///     builder.register_type::<HelloWorldImpl>();
///     let container = builder.build().unwrap();
///
/// # if false { // We don't actually want to launch the server in an example.
///     rocket::ignite()
///         .manage(container)
///         .mount("/", routes![hello])
///         .launch();
/// # }
/// }
/// ```
pub struct Inject<'r, T: Interface + ?Sized>(&'r T);

impl<'a, 'r, T: Interface + ?Sized> FromRequest<'a, 'r> for Inject<'r, T> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, (Status, Self::Error), ()> {
        let container: State<'r, Container> = request.guard::<State<Container>>()?;
        let component = container.inner().resolve_ref::<T>().unwrap();

        Outcome::Success(Inject(component))
    }
}

impl<'r, T: Interface + ?Sized> Deref for Inject<'r, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}
