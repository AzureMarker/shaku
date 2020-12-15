use crate::get_module_from_state;
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket::{http::Status, Request};
use shaku::{HasProvider, Module};
use std::marker::PhantomData;
use std::ops::Deref;

/// Used to create a provided service from a shaku `Module`.
/// The module should be stored in Rocket's state. Use this struct as a
/// request guard.
///
/// # Example
/// ```rust
/// #![feature(proc_macro_hygiene, decl_macro)]
///
/// #[macro_use] extern crate rocket;
///
/// use shaku::{module, Provider};
/// use shaku_rocket::InjectProvided;
///
/// trait HelloWorld {
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
///     let module = HelloModule::builder().build();
///
/// # if false { // We don't actually want to launch the server in an example.
///     rocket::ignite()
///         .manage(module)
///         .mount("/", routes![hello])
///         .launch();
/// # }
/// }
/// ```
pub struct InjectProvided<M: Module + HasProvider<I>, I: ?Sized>(Box<I>, PhantomData<M>);

impl<'a, 'r, M: Module + HasProvider<I>, I: ?Sized> FromRequest<'a, 'r> for InjectProvided<M, I> {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let module = get_module_from_state::<M>(request)?;
        let service = module
            .inner()
            .provide()
            .map_err(|e| e.to_string())
            .into_outcome(Status::InternalServerError)?;

        Outcome::Success(InjectProvided(service, PhantomData))
    }
}

impl<M: Module + HasProvider<I>, I: ?Sized> Deref for InjectProvided<M, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
