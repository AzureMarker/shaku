use crate::autofac::{AutoFacModule, IDateWriter, TodayWriter, TodayWriterParameters};
use axum::extract::FromRef;
use axum::{routing::get, Router};
use shaku_axum::Inject;
use std::sync::Arc;
use tokio::net::TcpListener;

mod autofac;

async fn index(writer: Inject<AutoFacModule, dyn IDateWriter>) -> String {
    writer.write_date();
    writer.get_date()
}

#[derive(Clone)]
struct AppState {
    module: Arc<AutoFacModule>,
}

impl FromRef<AppState> for Arc<AutoFacModule> {
    fn from_ref(app_state: &AppState) -> Arc<AutoFacModule> {
        app_state.module.clone()
    }
}

#[tokio::main]
async fn main() {
    let module = Arc::new(
        AutoFacModule::builder()
            .with_component_parameters::<TodayWriter>(TodayWriterParameters {
                today: "November 5".to_string(),
                year: 2020,
            })
            .build(),
    );

    let state = AppState { module };

    let app = Router::new().route("/", get(index)).with_state(state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
