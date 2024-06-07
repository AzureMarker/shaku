//! Requesting a provider which is not supported by the module will fail to compile

use shaku::{module, HasProvider, Provider};

trait ServiceTrait {}

#[derive(Provider)]
#[shaku(interface = ServiceTrait)]
struct ServiceImpl;
impl ServiceTrait for ServiceImpl {}

module! {
    TestModule {
        components = [],
        providers = [],
        interfaces = []
    }
}

fn main() {
    let module = TestModule::builder().build();
    let _service = HasProvider::<dyn ServiceTrait>::provide(&module);
}
