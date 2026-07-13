//! Tests for default parameter generation

use shaku::{Component, Interface};

trait MyTrait: Interface {}

// Notice this struct does not implement Default
#[derive(Debug, PartialEq)]
struct MyStruct(usize);

#[derive(Component)]
#[shaku(interface = MyTrait)]
#[allow(dead_code)]
struct MyComponent {
    #[shaku(default)]
    value_one: usize,
    #[shaku(default = 10)]
    value_two: usize,
    #[shaku(default = MyStruct(20))]
    value_three: MyStruct,
}
impl MyTrait for MyComponent {}

/// A parameter with `#[shaku(default)]` uses `Default::default()`
#[test]
fn simply_annotated_uses_normal_default() {
    let parameters = MyComponentParameters::default();

    assert_eq!(parameters.value_one, usize::default());
}

/// A parameter with `#[shaku(default = ...)]` uses the given default value
#[test]
fn annotated_uses_given_default() {
    let parameters = MyComponentParameters::default();

    assert_eq!(parameters.value_two, 10);
}

/// A parameter with `#[shaku(default = ...)]` can still be overridden
#[test]
fn override_annotated_default() {
    let parameters = MyComponentParameters {
        value_two: 3,
        ..Default::default()
    };

    assert_eq!(parameters.value_two, 3);
}

/// A parameter which does not implement `Default` will still work if a default value is given via
/// attribute. This also indirectly tests expression support.
#[test]
fn annotated_non_default_uses_given_default() {
    let parameters = MyComponentParameters::default();

    assert_eq!(parameters.value_three, MyStruct(20));
}
