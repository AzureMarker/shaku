use std::ops::Deref;

use crate::get_container_from_state;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use shaku::{HasComponent, Interface, Module};
use std::marker::PhantomData;

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
/// use shaku::{module, Component, Container, ContainerBuilder, Interface};
/// use shaku_rocket::Inject;
///
/// trait HelloWorld: Interface {
///     fn greet(&self) -> String;
/// }
///
/// #[derive(Component)]
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
///         components = [HelloWorldImpl],
///         providers = []
///     }
/// }
///
/// #[get("/")]
/// fn hello(hello_world: Inject<HelloModule, dyn HelloWorld>) -> String {
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
pub struct Inject<'r, M: Module + HasComponent<I> + Send + Sync, I: Interface + ?Sized>(
    &'r I,
    PhantomData<M>,
);

impl<'a, 'r, M: Module + HasComponent<I> + Send + Sync, I: Interface + ?Sized> FromRequest<'a, 'r>
    for Inject<'r, M, I>
{
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let container = get_container_from_state::<M>(request)?;
        let component = container.inner().resolve_ref::<I>();

        Outcome::Success(Inject(component, PhantomData))
    }
}

impl<'r, M: Module + HasComponent<I> + Send + Sync, I: Interface + ?Sized> Deref
    for Inject<'r, M, I>
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
