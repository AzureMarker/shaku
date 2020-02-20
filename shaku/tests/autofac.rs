//! Examples based on AutoFac 'getting started' example
//! (http://autofac.readthedocs.io/en/latest/getting-started/index.html)

use std::sync::Arc;

use shaku::{module, Component, Container, ContainerBuilder, Interface};

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
        providers = []
    }
}

#[test]
fn main_test() {
    let container: Container<AutoFacModule> = ContainerBuilder::new()
        .with_component_parameters::<ConsoleOutput>(ConsoleOutputParameters {
            prefix: "PREFIX > ".to_string(),
        })
        .with_component_parameters::<TodayWriter>(TodayWriterParameters {
            today: "June 19".to_string(),
            year: 2020,
        })
        .build();

    let writer = container.resolve::<dyn IDateWriter>();
    writer.write_date();
    let date = writer.get_date();
    assert_eq!(date, "Today is June 19, 2020");
}
