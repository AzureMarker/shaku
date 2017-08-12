//! Examples based on AutoFac 'getting started' example
//! (http://autofac.readthedocs.io/en/latest/getting-started/index.html)
extern crate shaku;
#[macro_use] extern crate shaku_derive;

use shaku::{ Container, ContainerBuilder };

// IOutput & ConsoleOutput implementation
// ---------------------------------------------------------------------
trait IOutput : Send {
    fn write(&self, content: String);
    fn get_date(&self, content: String) -> String;
}

#[derive(Component)]
#[interface(IOutput)]
struct ConsoleOutput {
    prefix: String,
    other_param: usize,
}

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!(
            "[Outputting to the console] {} #{} {}",
            self.prefix,
            self.other_param,
            content
        );
    }

    fn get_date(&self, content: String) -> String {
        format!(
            "{}#{} {}",
            self.prefix,
            self.other_param,
            content
        )
    }
}

// IDateWriter & TodayWriter implementation
// ---------------------------------------------------------------------
trait IDateWriter : Send {
    fn write_date(&self);
    fn get_date(&self) -> String;
}

#[derive(Component)]
#[interface(IDateWriter)]
struct TodayWriter {
    #[inject]
    output: Box<IOutput>,
    today: String,
    year: usize,
}

impl IDateWriter for TodayWriter {
    fn write_date(&self) {
        let mut content = "Today is ".to_string();
        content.push_str(self.today.as_str());
        content.push_str(" ");
        content.push_str(self.year.to_string().as_str());
        self.output.write(content);
    }

    fn get_date(&self) -> String {
        let mut content = "Today is ".to_string();
        content.push_str(self.today.as_str());
        self.output.get_date(content)
    }
}

fn main() {
    // Create your builder.
    let mut builder = ContainerBuilder::new();

    builder
        .register_type::<ConsoleOutput>()
        .as_type::<IOutput>()
        .with_named_parameter("prefix", "PREFIX >".to_string())
        .with_typed_parameter(117 as usize);
    builder
        .register_type::<TodayWriter>()
        .as_type::<IDateWriter>();
    let mut container = builder.build().unwrap();

    // The write_date method is where we'll make use
    // of our dependency injection. We'll define that
    // in a bit.
    write_date(&mut container);
}

fn write_date(container: &mut Container) {
    let writer = container
        .with_typed_parameter::<IDateWriter, String>("June 22".to_string())
        .with_named_parameter::<IDateWriter, usize>("year", 2017 as usize)
        .resolve::<IDateWriter>()
        .unwrap();
    writer.write_date();
    let date = writer.get_date();
    println!("date = {}", date);
}
