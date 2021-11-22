use crate::get_module_from_state;
use axum::async_trait;
use axum::extract::{FromRequest, RequestParts};
use axum::http::StatusCode;
use serde_json::{json, Value};
use shaku::{HasProvider, Interface, ModuleInterface};
use std::marker::PhantomData;
use std::ops::Deref;

/// Used to create a provided service from a shaku `Module`.
/// The module should be stored in Axum layer, wrapped in an `Arc`.
/// Use this struct as an extractor.
///
/// # Example
/// ```rust
/// use axum::{routing::get, AddExtensionLayer, Router};
/// use shaku::{module, Interface, Provider};
/// use shaku_axum::InjectProvided;
/// use std::net::SocketAddr;
/// use std::sync::Arc;
///
/// trait HelloWorld: Interface {
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
/// #[tokio::main]
/// async fn main() {
///     let module = Arc::new(HelloModule::builder().build());
///
///     let app = Router::new()
///         .route("/", get(hello))
///         .layer(AddExtensionLayer::new(module));
///
///     # if false {
///     axum::Server::bind(&SocketAddr::from(([127, 0, 0, 1], 8080)))
///         .serve(app.into_make_service())
///         .await
///         .unwrap();
///     }
/// }
/// ```
pub struct InjectProvided<M: ModuleInterface + HasProvider<I> + ?Sized, I: ?Sized>(
    Box<I>,
    PhantomData<M>,
);

#[async_trait]
impl<B, M, I> FromRequest<B> for InjectProvided<M, I>
where
    B: Send,
    M: ModuleInterface + HasProvider<I> + ?Sized,
    I: ?Sized,
{
    type Rejection = (StatusCode, axum::Json<Value>);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let module = get_module_from_state::<M, B>(req)?;
        let service = module.provide().map_err(|e| {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(json!({ "error": format!("{}", e) })),
            )
        })?;

        Ok(Self(service, PhantomData))
    }
}

impl<M: ModuleInterface + HasProvider<I> + ?Sized, I: Interface + ?Sized> Deref
    for InjectProvided<M, I>
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
