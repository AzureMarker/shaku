[![Current version](https://img.shields.io/crates/v/shaku.svg)](https://crates.io/crates/shaku)
[![Current documentation](https://docs.rs/shaku/badge.svg)][docs]

# Shaku

Shaku is a Rust dependency injection library. See the [docs] for more details,
including a getting started guide.

## Example
```rust
use shaku::{Component, ContainerBuilder, Interface};
use std::sync::Arc;

trait IOutput: Interface {
    fn write(&self, content: String);
}

impl IOutput for ConsoleOutput {
    fn write(&self, content: String) {
        println!("{}", content);
    }
}

#[derive(Component)]
#[shaku(interface = IOutput)]
struct ConsoleOutput;

trait IDateWriter: Interface {
    fn write_date(&self);
}

impl IDateWriter for TodayWriter {
    fn write_date(&self) {
        self.output.write(format!("Today is {} {}", self.today, self.year));
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

fn main() {
    let mut builder = ContainerBuilder::new();
    builder.register_type::<ConsoleOutput>();
    builder
        .register_type::<TodayWriter>()
        .with_named_parameter("today", "Jan 26".to_string())
        .with_typed_parameter::<usize>(2020);
    let container = builder.build().unwrap();

    let writer: &dyn IDateWriter = container.resolve_ref().unwrap();
    writer.write_date();
}
```

## Acknowledgements
This library started off as "he_di" (later renamed to "shaku") under the
guidance of [@bgbahoue] and [@U007D]. Their work inspired the current maintainer
([@Mcat12]) to continue the library from where they left off.

[docs]: https://docs.rs/crate/shaku
[@bgbahoue]: https://github.com/bgbahoue
[@U007D]: https://github.com/U007D
[@Mcat12]: https://github.com/Mcat12
