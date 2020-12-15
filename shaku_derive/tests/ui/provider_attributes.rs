//! Providers cannot have module attributes

use shaku::{module, Provider};

trait ProviderTrait {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl;
impl ProviderTrait for ProviderImpl {}

module! {
    TestModule {
        components = [],
        providers = [#[lazy] ProviderImpl]
    }
}

fn main() {}
