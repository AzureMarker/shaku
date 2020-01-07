//! Traits and types used by the #derive macro to build Component objects

use crate::container::Container;
use crate::parameter::ParameterMap;

pub trait Component {
    type Interface: ?Sized + 'static;

    fn build(_: &mut Container, _: &mut ParameterMap) -> super::Result<Box<Self::Interface>>;
}

pub(crate) type ComponentBuildFn<I> =
    fn(&mut Container, &mut ParameterMap) -> super::Result<Box<I>>;
