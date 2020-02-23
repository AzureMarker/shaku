use crate::{Component, HasComponent, Module};

/// Used to store the parameters of a component. This is used instead of
/// directly storing the parameters to avoid mixing up parameters of the same
/// type.
///
/// Example: Component1 and Component2 both use String as their parameter type.
/// If the parameter was stored directly in the parameter map, one of the
/// strings would overwrite the other string.
pub(crate) struct ComponentParameters<
    M: Module + HasComponent<C::Interface, Impl = C>,
    C: Component<M>,
> {
    pub(crate) value: C::Parameters,
}

// The Default derive isn't smart enough to handle this, so it's manually implemented
impl<M: Module + HasComponent<C::Interface, Impl = C>, C: Component<M>> Default
    for ComponentParameters<M, C>
{
    fn default() -> Self {
        Self {
            value: C::Parameters::default(),
        }
    }
}
