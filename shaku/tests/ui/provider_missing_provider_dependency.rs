//! A module which does not satisfy a provider's provider dependency will fail to compile

use shaku::{module, Provider};

trait DependencyTrait {}
trait ProviderTrait {}

#[derive(Provider)]
#[shaku(interface = DependencyTrait)]
struct DependencyImpl;
impl DependencyTrait for DependencyImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    #[shaku(provide)]
    dependency: Box<dyn DependencyTrait>,
}
impl ProviderTrait for ProviderImpl {}

module! {
    TestModule {
        components = [],
        providers = [ProviderImpl]
    }
}

fn main() {}
