//! A simple example of generic services and modules

use shaku::{module, Component, HasComponent, Interface};

trait Animal {
    fn audible_sound(&self) -> &str;
}

#[derive(Default)]
struct Cat;
impl Animal for Cat {
    fn audible_sound(&self) -> &str {
        "Meow"
    }
}

#[derive(Default)]
struct Dog;
impl Animal for Dog {
    fn audible_sound(&self) -> &str {
        "Woof"
    }
}

trait AnimalService: Interface {
    fn get_sound(&self) -> &str;
}

#[derive(Component)]
#[shaku(interface = AnimalService)]
struct AnimalServiceImpl<A>
where
    A: Animal + Default + Interface,
{
    animal: A,
}

impl<A> AnimalService for AnimalServiceImpl<A>
where
    A: Animal + Default + Interface,
{
    fn get_sound(&self) -> &str {
        self.animal.audible_sound()
    }
}

module! {
    MyModule<A: Animal + Default + Interface> {
        components = [AnimalServiceImpl<A>],
        providers = []
    }
}

fn main() {
    // Create a module from a concrete
    let cat_module = MyModule::<Cat>::builder().build();
    let dog_module = MyModule::<Dog>::builder().build();

    let cat_service: &dyn AnimalService = cat_module.resolve_ref();
    let dog_service: &dyn AnimalService = dog_module.resolve_ref();

    println!("Cat service sound: {}", cat_service.get_sound());
    println!("Dog service sound: {}", dog_service.get_sound());
}
