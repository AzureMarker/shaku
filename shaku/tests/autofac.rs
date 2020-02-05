//! Examples based on AutoFac 'getting started' example
//! (http://autofac.readthedocs.io/en/latest/getting-started/index.html)

use std::sync::Arc;

use shaku::{Component, Interface};

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

#[test]
fn main_test() {
    let mut builder = shaku::ContainerBuilder::new();

    builder
        .register_type::<ConsoleOutput>()
        .with_named_parameter("prefix", "PREFIX > ".to_string());
    builder
        .register_type::<TodayWriter>()
        .with_typed_parameter::<String>("June 19".to_string())
        .with_typed_parameter::<usize>(2020);
    let container = builder.build().unwrap();

    let writer = container.resolve::<dyn IDateWriter>().unwrap();
    writer.write_date();
    let date = writer.get_date();
    assert_eq!(date, "Today is June 19, 2020");
}
