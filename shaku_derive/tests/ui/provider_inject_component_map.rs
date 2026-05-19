//! Providers cannot inject keyed component maps yet

use shaku::{Component, Interface, Provider};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum DependencyKind {
    Main,
}

trait DependencyTrait: Interface {}
trait ProviderTrait {}

#[derive(Component)]
#[shaku(interface = DependencyTrait)]
struct DependencyImpl;
impl DependencyTrait for DependencyImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    #[shaku(inject)]
    dependencies: HashMap<DependencyKind, Arc<dyn DependencyTrait>>,
}

impl ProviderTrait for ProviderImpl {}

fn main() {}
