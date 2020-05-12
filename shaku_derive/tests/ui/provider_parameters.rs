//! Providers cannot have parameters

use shaku::Provider;

trait ProviderTrait {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    value: usize,
}
impl ProviderTrait for ProviderImpl {}

fn main() {}
