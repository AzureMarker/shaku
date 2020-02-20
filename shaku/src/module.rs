use std::sync::Arc;

use crate::{Component, ContainerBuildContext, Interface, ProvidedInterface, Provider};

pub trait Module: Sized + 'static {
    fn build(context: &mut ContainerBuildContext<Self>) -> Self;
}

pub trait HasComponent<I: Interface + ?Sized>: Module {
    type Impl: Component<Self, Interface = I>;

    fn get_ref(&self) -> &Arc<I>;

    fn get_mut(&mut self) -> &mut Arc<I>;
}

pub trait HasProvider<I: ProvidedInterface + ?Sized>: Module {
    type Impl: Provider<Self, Interface = I>;
}

#[macro_export]
macro_rules! module {
    {
        $module:ident {
            components = [
                $($component:ident),* $(,)?
            ],
            providers = [
                $($provider:ident),* $(,)?
            ] $(,)?
        }
    } => {
        #[allow(non_snake_case)]
        struct $module {
            $(
                // It would be nice to prefix the property with something like
                // "__di_", but macro_rules does not support concatenating
                // idents on stable.
                $component: ::std::sync::Arc<<$component as $crate::Component<Self>>::Interface>
            ),*
        }

        impl $crate::Module for $module {
            fn build(context: &mut $crate::ContainerBuildContext<Self>) -> Self {
                Self { $(
                    $component: context.resolve::<<$component as $crate::Component<Self>>::Interface>()
                ),* }
            }
        }

        $(
        impl $crate::HasComponent<<$component as $crate::Component<Self>>::Interface> for $module {
            type Impl = $component;

            fn get_ref(&self) -> &::std::sync::Arc<<$component as $crate::Component<Self>>::Interface> {
                &self.$component
            }

            fn get_mut(&mut self) -> &mut ::std::sync::Arc<<$component as $crate::Component<Self>>::Interface> {
                &mut self.$component
            }
        }
        )*

        $(
        impl $crate::HasProvider<<$provider as $crate::Provider<Self>>::Interface> for $module {
            type Impl = $provider;
        }
        )*
    };
}
