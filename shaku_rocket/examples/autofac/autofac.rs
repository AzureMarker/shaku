//! Example based on the AutoFac 'getting started' example
//! (http://autofac.readthedocs.io/en/latest/getting-started/index.html)

use shaku::{module, Component, Interface};
use std::sync::Arc;

module! {
    pub AutoFacModule {
        components = [ConsoleOutput, TodayWriter],
        providers = []
    }
}

pub trait IOutput: Interface {
    fn write(&self, content: String);
}

#[derive(Component)]
#[shaku(interface = IOutput)]
pub struct ConsoleOutput;

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!("{}", content);
    }
}

pub trait IDateWriter: Interface {
    fn write_date(&self);
    fn get_date(&self) -> String;
}

#[derive(Component)]
#[shaku(interface = IDateWriter)]
pub struct TodayWriter {
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
