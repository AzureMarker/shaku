use crate::get_module_from_state;
use actix_web::dev::Payload;
use actix_web::{Error, FromRequest, HttpRequest};
use futures_util::future;
use shaku::{HasComponent, Interface, ModuleInterface};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

/// Used to retrieve a reference to a component from a shaku `Module`.
/// The module should be stored in Actix's app data, wrapped in an `Arc`.
/// Use this struct as an extractor.
///
/// # Example
/// ```rust
/// use actix_web::{App, HttpServer, web};
/// use shaku::{module, Component, Interface};
/// use shaku_actix::Inject;
/// use std::sync::Arc;
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
/// async fn hello(hello_world: Inject<HelloModule, dyn HelloWorld>) -> String {
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
pub struct Inject<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized>(
    Arc<I>,
    PhantomData<M>,
);

impl<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized> FromRequest
    for Inject<M, I>
{
    type Error = Error;
    type Future = future::Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let module = match get_module_from_state::<M>(req) {
            Ok(module) => module,
            Err(e) => return future::err(e),
        };
        let component = module.resolve();

        future::ok(Inject(component, PhantomData))
    }
}

impl<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized> Deref for Inject<M, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        Arc::as_ref(&self.0)
    }
}
