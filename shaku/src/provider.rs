use std::any::Any;

use crate::{Container, Dependency};

pub trait Provider: 'static {
    type Interface: ProvidedInterface + ?Sized;

    /// The services/providers which this provider depends on.
    fn dependencies() -> Vec<Dependency>;

    fn provide(container: &Container) -> super::Result<Box<Self::Interface>>;
}

#[cfg(not(feature = "thread_safe"))]
pub(crate) type ProviderFn<I> = Box<dyn (Fn(&Container) -> super::Result<Box<I>>)>;
#[cfg(feature = "thread_safe")]
pub(crate) type ProviderFn<I> = Box<dyn (Fn(&Container) -> super::Result<Box<I>>) + Send + Sync>;

#[cfg(not(feature = "thread_safe"))]
trait_alias!(pub ProvidedInterface = Any);
#[cfg(feature = "thread_safe")]
trait_alias!(pub ProvidedInterface = Any + Send);
