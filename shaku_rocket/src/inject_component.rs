use crate::get_module_from_state;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use shaku::{HasComponent, Interface, ModuleInterface};
use std::marker::PhantomData;
use std::ops::Deref;

/// Used to retrieve a reference to a component from a shaku `Module`.
/// The module should be stored in Rocket's state, in a `Box` (It could be
/// `Box<dyn MyModule>` if the module implementation changes at runtime).
/// Use this `Inject` struct as a request guard.
///
/// # Example
/// ```rust
/// #![feature(proc_macro_hygiene, decl_macro)]
///
/// #[macro_use] extern crate rocket;
///
/// use shaku::{module, Component, Interface};
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
///     let module = HelloModule::builder().build();
///
/// # if false { // We don't actually want to launch the server in an example.
///     rocket::ignite()
///         .manage(Box::new(module))
///         .mount("/", routes![hello])
///         .launch();
/// # }
/// }
/// ```
pub struct Inject<'r, M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized>(
    &'r I,
    PhantomData<M>,
);

impl<'a, 'r, M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized>
    FromRequest<'a, 'r> for Inject<'r, M, I>
{
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let module = get_module_from_state::<M>(request)?;
        let component = module.inner().resolve_ref();

        Outcome::Success(Inject(component, PhantomData))
    }
}

impl<'r, M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized> Deref
    for Inject<'r, M, I>
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
