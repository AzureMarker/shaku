//! A non-Box property cannot be provided

use shaku::{Component, Interface, Provider};
use std::sync::Arc;

trait DependencyTrait: Interface {}
trait ProviderTrait {}

#[derive(Component)]
#[shaku(interface = DependencyTrait)]
struct DependencyImpl;
impl DependencyTrait for DependencyImpl {}

#[derive(Provider)]
#[shaku(interface = ProviderTrait)]
struct ProviderImpl {
    #[shaku(provide)]
    dependency: Arc<dyn DependencyTrait>,
}
impl ProviderTrait for ProviderImpl {}

fn main() {}
