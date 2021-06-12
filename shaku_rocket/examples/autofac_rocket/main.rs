#[macro_use]
extern crate rocket;

use crate::autofac::{AutoFacModule, IDateWriter, TodayWriter, TodayWriterParameters};
use shaku_rocket::Inject;

mod autofac;

#[get("/")]
fn index(writer: Inject<AutoFacModule, dyn IDateWriter>) -> String {
    writer.write_date();
    writer.get_date()
}

#[rocket::launch]
async fn rocket() -> _ {
    let module = AutoFacModule::builder()
        .with_component_parameters::<TodayWriter>(TodayWriterParameters {
            today: "June 19".to_string(),
            year: 2020,
        })
        .build();

    rocket::build()
        .manage(Box::new(module))
        .mount("/", routes![index])
}
