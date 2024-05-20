use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use shaku::{HasComponent, Interface, ModuleInterface};

use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

/// Used to retrieve a reference to a component from a shaku `Module`.
/// The module should be stored in Axum state, wrapped in an `Arc` (`Arc<MyModule>`).
/// This `Arc<MyModule>` must implement `FromRef<S>` where `S` is the Axum state type.
///
/// Use this struct as an extractor.
///
/// # Example
/// ```rust
/// use axum::{routing::get, Router};
/// use axum::extract::{Extension, FromRef};
/// use shaku::{module, Component, Interface};
/// use shaku_axum::Inject;
/// use std::net::SocketAddr;
/// use std::sync::Arc;
/// use tokio::net::TcpListener;
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
/// #[derive(Clone)]
/// struct AppState {
///     module: Arc<HelloModule>,
/// }
///
/// impl FromRef<AppState> for Arc<HelloModule> {
///     fn from_ref(app_state: &AppState) -> Arc<HelloModule> {
///         app_state.module.clone()
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
///     let state = AppState { module };
///
///     let app = Router::new()
///         .route("/", get(hello))
///         .with_state(state);
///
///     # if false {
///     let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
///     axum::serve(listener, app.into_make_service())
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
impl<S, M, I> FromRequestParts<S> for Inject<M, I>
where
    S: Send + Sync,
    M: ModuleInterface + HasComponent<I> + ?Sized,
    I: Interface + ?Sized,
    Arc<M>: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let component = Arc::<M>::from_ref(state).resolve();

        Ok(Self(component, PhantomData))
    }
}

impl<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized> Deref for Inject<M, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        Arc::as_ref(&self.0)
    }
}
