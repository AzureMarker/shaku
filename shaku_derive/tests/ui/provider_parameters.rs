//! Providers cannot have parameters

use shaku::{ProvidedInterface, Provider};

trait ProviderTrait: ProvidedInterface {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    value: usize,
}
impl ProviderTrait for ProviderImpl {}

fn main() {}
