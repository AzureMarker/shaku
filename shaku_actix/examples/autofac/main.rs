use crate::autofac::{AutoFacModule, IDateWriter, TodayWriter, TodayWriterParameters};
use actix_web::{web, App, HttpServer};
use shaku_actix::Inject;
use std::sync::Arc;

mod autofac;

async fn index(writer: Inject<AutoFacModule, dyn IDateWriter>) -> String {
    writer.write_date();
    writer.get_date()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let module = Arc::new(
        AutoFacModule::builder()
            .with_component_parameters::<TodayWriter>(TodayWriterParameters {
                today: "June 19".to_string(),
                year: 2020,
            })
            .build(),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(module.clone())
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
