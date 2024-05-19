use std::marker::PhantomData;
use std::ops::Deref;

use rocket::outcome::{try_outcome};
use rocket::request::{FromRequest, Outcome};
use rocket::{http::Status, Request};

use shaku::{HasProvider, ModuleInterface};

use crate::get_module_from_state;

/// Used to create a provided service from a shaku `Module`.
/// The module should be stored in Rocket's state, in a `Box` (It could be
/// `Box<dyn MyModule>` if the module implementation changes at runtime).
/// Use this `InjectProvided` struct as a request guard.
///
/// # Example
/// ```rust
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
/// # fn main() { // We don't actually want to launch the server in an example.
/// #[rocket::launch]
/// fn rocket() -> _ {
///     let module = HelloModule::builder().build();
///
///     rocket::build()
///         .manage(Box::new(module))
///         .mount("/", routes![hello])
/// }
/// # }
/// ```
pub struct InjectProvided<M: ModuleInterface + HasProvider<I> + ?Sized, I: ?Sized>(
    Box<I>,
    PhantomData<M>,
);

#[rocket::async_trait]
impl<'r, M: ModuleInterface + HasProvider<I> + ?Sized, I: ?Sized> FromRequest<'r>
    for InjectProvided<M, I>
{
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let module = try_outcome!(get_module_from_state::<M>(request).await);

        let service_result = module.inner().provide();

        let outcome = match service_result {
            Ok(service) => Outcome::Success(InjectProvided(service, PhantomData)),
            Err(e) => Outcome::Error((Status::InternalServerError, e.to_string())),
        };

        outcome
    }
}

impl<M: ModuleInterface + HasProvider<I> + ?Sized, I: ?Sized> Deref for InjectProvided<M, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
