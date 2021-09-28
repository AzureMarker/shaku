//! The module macro should not require any shaku traits to be in scope

use self::services::{MyComponent, MyProvider, MyProviderImpl, ServicesModule};
use shaku::module;

module! {
    TestModule {
        components = [],
        providers = [MyProviderImpl as dyn MyProvider],

        use ServicesModule {
            components = [MyComponent],
            providers = []
        }
    }
}

mod services {
    use shaku::{module, Component, Interface, Provider};

    pub trait MyComponent: Interface {}
    pub trait MyProvider {}

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
            components = [MyComponentImpl as dyn MyComponent],
            providers = []
        }
    }
}

#[test]
fn compile_ok() {}
