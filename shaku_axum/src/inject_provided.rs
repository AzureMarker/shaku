use axum::async_trait;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use shaku::{HasProvider, ModuleInterface};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

/// Used to create a provided service from a shaku `Module`.
/// The module should be stored in Axum layer, wrapped in an `Arc`.
/// Use this struct as an extractor.
///
/// # Example
/// ```rust
/// use axum::{routing::get, Router};
/// use axum::extract::Extension;
/// use shaku::{module, Interface, Provider};
/// use shaku_axum::InjectProvided;
/// use std::net::SocketAddr;
/// use std::sync::Arc;
///
/// trait HelloWorld: Send + Sync {
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
///         .with_state(module);
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
impl<S, M, I> FromRequestParts<S> for InjectProvided<M, I>
where
    S: Send + Sync,
    M: ModuleInterface + HasProvider<I> + ?Sized,
    I: ?Sized,
    Arc<M>: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let service = Arc::<M>::from_ref(state)
            .provide()
            .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Self(service, PhantomData))
    }
}

impl<M: ModuleInterface + HasProvider<I> + ?Sized, I: ?Sized> Deref for InjectProvided<M, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
