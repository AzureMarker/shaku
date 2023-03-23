use crate::get_module_from_state;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use shaku::{HasComponent, Interface, ModuleInterface};

use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

/// Used to retrieve a reference to a component from a shaku `Module`.
/// The module should be stored in an Axum layer, wrapped in an `Arc`.
/// Use this struct as an extractor.
///
/// # Example
/// ```rust
/// use axum::{routing::get, Router};
/// use axum::extract::Extension;
/// use shaku::{module, Component, Interface};
/// use shaku_axum::Inject;
/// use std::net::SocketAddr;
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
/// #[tokio::main]
/// async fn main() {
///     let module = Arc::new(HelloModule::builder().build());
///
///     let app = Router::new()
///         .route("/", get(hello))
///         .layer(Extension(module));
///
///     # if false {
///     axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8080)))
///         .serve(app.into_make_service())
///         .await
///         .unwrap();
///     # }
/// }
/// ```
pub struct Inject<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized>(
    Arc<I>,
    PhantomData<M>,
);

#[async_trait]
impl<B, M, I> FromRequestParts<B> for Inject<M, I>
where
    B: Send,
    M: ModuleInterface + HasComponent<I> + ?Sized,
    I: Interface + ?Sized,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(req: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        let module = get_module_from_state::<M>(req)?;

        let component = module.resolve();

        Ok(Self(component, PhantomData))
    }
}

impl<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized> Deref for Inject<M, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        Arc::as_ref(&self.0)
    }
}
