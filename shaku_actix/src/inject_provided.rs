use crate::get_module_from_state;
use actix_web::dev::{Payload, PayloadStream};
use actix_web::error::ErrorInternalServerError;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future;
use shaku::{HasProvider, ModuleInterface};
use std::marker::PhantomData;
use std::ops::Deref;

/// Used to create a provided service from a shaku `Module`.
/// The module should be stored in Actix's app data, wrapped in an `Arc`.
/// Use this struct as an extractor.
///
/// # Example
/// ```rust
/// use actix_web::{App, HttpServer, web};
/// use shaku::{module, Provider};
/// use shaku_actix::InjectProvided;
/// use std::sync::Arc;
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
/// async fn hello(hello_world: InjectProvided<HelloModule, dyn HelloWorld>) -> String {
///     hello_world.greet()
/// }
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     let module = Arc::new(HelloModule::builder().build());
///
/// # if false { // We don't actually want to launch the server in an example.
///     HttpServer::new(move || {
///         App::new()
///             .app_data(module.clone())
///             .route("/", web::get().to(hello))
///     })
///     .bind("127.0.0.1:8080")?
///     .run()
///     .await
/// # } else { Ok(()) }
/// }
/// ```
pub struct InjectProvided<M: ModuleInterface + HasProvider<I> + ?Sized, I: ?Sized>(
    Box<I>,
    PhantomData<M>,
);

impl<M: ModuleInterface + HasProvider<I> + ?Sized, I: ?Sized> FromRequest for InjectProvided<M, I> {
    type Error = Error;
    type Future = future::Ready<Result<Self, Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut Payload<PayloadStream>) -> Self::Future {
        let module = match get_module_from_state::<M>(req) {
            Ok(module) => module,
            Err(e) => return future::err(e),
        };
        let service = match module.provide() {
            Ok(service) => service,
            Err(e) => return future::err(ErrorInternalServerError(e)),
        };

        future::ok(InjectProvided(service, PhantomData))
    }
}

impl<M: ModuleInterface + HasProvider<I> + ?Sized, I: ?Sized> Deref for InjectProvided<M, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
