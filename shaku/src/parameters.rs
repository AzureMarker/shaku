use std::marker::PhantomData;

/// Used to store the parameters of a component. This is used instead of
/// directly storing the parameters to avoid mixing up parameters of the same
/// type.
///
/// Example: Component1 and Component2 both use String as their parameter type.
/// If the parameter was stored directly in the parameter map, one of the
/// strings would overwrite the other string.
pub(crate) struct ComponentParameters<C, P: Default> {
    pub(crate) value: P,
    pub(crate) _component: PhantomData<*const C>,
}

impl<C, P: Default> ComponentParameters<C, P> {
    pub(crate) fn new(value: P) -> Self {
        Self {
            value,
            _component: PhantomData,
        }
    }
}

// The Default derive isn't smart enough to handle this, so it's manually implemented
impl<C, P: Default> Default for ComponentParameters<C, P> {
    fn default() -> Self {
        Self {
            value: P::default(),
            _component: PhantomData,
        }
    }
}
