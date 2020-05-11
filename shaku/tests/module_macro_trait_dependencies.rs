//! The module macro should not require any shaku traits to be in scope

use self::services::{MyComponent, MyProviderImpl, ServicesModule};
use shaku::module;

module! {
    TestModule {
        components = [],
        providers = [MyProviderImpl],

        use ServicesModule {
            components = [MyComponent],
            providers = []
        }
    }
}

mod services {
    use shaku::{module, Component, Interface, ProvidedInterface, Provider};

    pub trait MyComponent: Interface {}
    pub trait MyProvider: ProvidedInterface {}

    #[derive(Component)]
    #[shaku(interface = MyComponent)]
    pub struct MyComponentImpl;
    impl MyComponent for MyComponentImpl {}

    #[derive(Provider)]
    #[shaku(interface = MyProvider)]
    pub struct MyProviderImpl;
    impl MyProvider for MyProviderImpl {}

    module! {
        pub ServicesModule {
            components = [MyComponentImpl],
            providers = []
        }
    }
}

#[test]
fn compile_ok() {}
