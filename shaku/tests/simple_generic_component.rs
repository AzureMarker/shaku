//! Modules and services can be generic. Based off of issue #2:
//! https://github.com/Mcat12/shaku/issues/2

use shaku::{module, Component, Interface, HasComponent};

trait Animal {
   fn audible_identification(&self) -> &str;
}

#[derive(Default)]
struct Cat;
impl Animal for Cat {
    fn audible_identification(&self) -> &str {
        "Meow"
    }
}

#[derive(Default)]
struct Dog;
impl Animal for Dog {
    fn audible_identification(&self) -> &str {
        "Woof"
    }
}

trait AnimalService: Interface {
    fn get_identification(&self) -> &str;
}

#[derive(Component)]
#[shaku(interface = AnimalService)]
struct AnimalServiceImpl<A> where A: Animal + Default + Interface {
    animal: A,
}

impl<A> AnimalService for AnimalServiceImpl<A> where A: Animal + Default + Interface {
    fn get_identification(&self) -> &str {
        self.animal.audible_identification()
    }
}

module! {
    MyModule<A: Animal + Default + Interface> {
        components = [AnimalServiceImpl<A>],
        providers = []
    }
}

#[test]
fn can_make_2_generic_modules() {
    // Create a module from a concrete
    let cat_module = MyModule::<Cat>::builder().build();
    let dog_module = MyModule::<Dog>::builder().build();

    let cat_service: &dyn AnimalService = cat_module.resolve_ref();
    let dog_service: &dyn AnimalService = dog_module.resolve_ref();

    assert_eq!(cat_service.get_identification(), "Meow");
    assert_eq!(dog_service.get_identification(), "Woof");
}
