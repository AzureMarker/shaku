#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use shaku::ContainerBuilder;
use shaku_rocket::Inject;

use crate::autofac::{ConsoleOutput, IDateWriter, TodayWriter};

mod autofac;

#[get("/")]
fn index(writer: Inject<dyn IDateWriter>) -> String {
    writer.write_date();
    writer.get_date()
}

fn main() {
    let mut builder = ContainerBuilder::new();

    builder.register_type::<ConsoleOutput>();
    builder
        .register_type::<TodayWriter>()
        .with_named_parameter("today", "June 19".to_string())
        .with_typed_parameter::<usize>(2020);
    let container = builder.build().unwrap();

    rocket::ignite()
        .manage(container)
        .mount("/", routes![index])
        .launch();
}
