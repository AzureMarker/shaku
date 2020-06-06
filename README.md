[![Current version][crate-badge]][crates-io]
[![Current documentation][doc-badge]][docs]
[![Build status][build-badge]][builds]

# Shaku

Shaku is a compile time dependency injection Rust library. See the [docs] for
more details, including a getting started guide.

## Example
```rust
use shaku::{module, Component, Interface, HasComponent};
use std::sync::Arc;

trait Output: Interface {
    fn write(&self, content: String);
}

trait DateWriter: Interface {
    fn write_date(&self);
}

#[derive(Component)]
#[shaku(interface = Output)]
struct ConsoleOutput;

impl Output for ConsoleOutput {
    fn write(&self, content: String) {
        println!("{}", content);
    }
}

#[derive(Component)]
#[shaku(interface = DateWriter)]
struct TodayWriter {
    #[shaku(inject)]
    output: Arc<dyn Output>,
    today: String,
    year: usize,
}

impl DateWriter for TodayWriter {
    fn write_date(&self) {
        self.output.write(format!("Today is {}, {}", self.today, self.year));
    }
}

module! {
    MyModule {
        components = [ConsoleOutput, TodayWriter],
        providers = []
    }
}

fn main() {
    let module = MyModule::builder()
        .with_component_parameters::<TodayWriter>(TodayWriterParameters {
            today: "Jan 26".to_string(),
            year: 2020
        })
        .build();

    let writer: &dyn DateWriter = module.resolve_ref();
    writer.write_date();
}
```

## Minimum Supported Rust Version
Shaku supports the latest stable release of Rust, plus the previous two versions
at minimum (but possibly more). Changes to the minimum supported version will be
noted in the changelog.

Latest stable version: 1.43.1

Minimum supported version: 1.38.0

## Acknowledgements
This library started off as "he_di" (later renamed to "shaku") under the
guidance of [@bgbahoue] and [@U007D]. Their work inspired the current maintainer
([@Mcat12]) to continue the library from where they left off.

[crates-io]: https://crates.io/crates/shaku
[docs]: https://docs.rs/shaku
[builds]: https://circleci.com/gh/Mcat12/shaku
[crate-badge]: https://img.shields.io/crates/v/shaku.svg
[doc-badge]: https://docs.rs/shaku/badge.svg
[build-badge]: https://circleci.com/gh/Mcat12/shaku.svg?style=shield
[@bgbahoue]: https://github.com/bgbahoue
[@U007D]: https://github.com/U007D
[@Mcat12]: https://github.com/Mcat12
