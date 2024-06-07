//! Example based on the AutoFac 'getting started' example
//! (http://autofac.readthedocs.io/en/latest/getting-started/index.html)

use shaku::{module, Component, HasComponent, Interface};
use std::sync::Arc;

trait IOutput: Interface {
    fn write(&self, content: String);
}

trait IDateWriter: Interface {
    fn write_date(&self);
    fn get_date(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = IOutput)]
struct ConsoleOutput {
    prefix: String,
}

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!("{} {}", self.prefix, content);
    }
}

#[derive(Component)]
#[shaku(interface = IDateWriter)]
struct TodayWriter {
    #[shaku(inject)]
    output: Arc<dyn IOutput>,
    today: String,
    year: usize,
}

impl IDateWriter for TodayWriter {
    fn write_date(&self) {
        self.output.write(self.get_date());
    }

    fn get_date(&self) -> String {
        format!("Today is {}, {}", self.today, self.year)
    }
}

module! {
    AutoFacModule {
        components = [
            ConsoleOutput,
            TodayWriter
        ],
        providers = [],
        interfaces = []
    }
}

fn main() {
    let module = AutoFacModule::builder()
        .with_component_parameters::<ConsoleOutput>(ConsoleOutputParameters {
            prefix: "PREFIX > ".to_string(),
        })
        .with_component_parameters::<TodayWriter>(TodayWriterParameters {
            today: "June 19".to_string(),
            year: 2020,
        })
        .build();

    let writer: Arc<dyn IDateWriter> = module.resolve();
    writer.write_date();
    let date = writer.get_date();
    assert_eq!(date, "Today is June 19, 2020");
}
