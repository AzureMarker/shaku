use std::ops::Deref;

use crate::get_container_from_state;
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket::{http::Status, Request};
use shaku::{HasProvider, Module, ProvidedInterface};
use std::marker::PhantomData;

/// Used to create a provided service using a provider from a shaku `Container`.
/// The container should be stored in Rocket's state. Use this struct as a
/// request guard.
///
/// # Example
/// ```rust
/// #![feature(proc_macro_hygiene, decl_macro)]
///
/// #[macro_use] extern crate rocket;
///
/// use shaku::{module, Container, ContainerBuilder, ProvidedInterface, Provider};
/// use shaku_rocket::InjectProvided;
///
/// trait HelloWorld: ProvidedInterface {
///     fn greet(&self) -> String;
/// }
///
/// #[derive(Provider)]
/// #[shaku(interface = HelloWorld)]
/// struct HelloWorldImpl;
///
/// impl HelloWorld for HelloWorldImpl {
///     fn greet(&self) -> String {
///         "Hello, world!".to_owned()
///     }
/// }
///
/// module! {
///     HelloModule {
///         components = [],
///         providers = [HelloWorldImpl]
///     }
/// }
///
/// #[get("/")]
/// fn hello(hello_world: InjectProvided<HelloModule, dyn HelloWorld>) -> String {
///     hello_world.greet()
/// }
///
/// fn main() {
///     let container = Container::<HelloModule>::default();
///
/// # if false { // We don't actually want to launch the server in an example.
///     rocket::ignite()
///         .manage(container)
///         .mount("/", routes![hello])
///         .launch();
/// # }
/// }
/// ```
pub struct InjectProvided<M: Module + HasProvider<I> + Send + Sync, I: ProvidedInterface + ?Sized>(
    Box<I>,
    PhantomData<M>,
);

impl<'a, 'r, M: Module + HasProvider<I> + Send + Sync, I: ProvidedInterface + ?Sized>
    FromRequest<'a, 'r> for InjectProvided<M, I>
{
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let container = get_container_from_state::<M>(request)?;
        let component = container
            .inner()
            .provide::<I>()
            .map_err(|e| e.to_string())
            .into_outcome(Status::InternalServerError)?;

        Outcome::Success(InjectProvided(component, PhantomData))
    }
}

impl<M: Module + HasProvider<I> + Send + Sync, I: ProvidedInterface + ?Sized> Deref
    for InjectProvided<M, I>
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
