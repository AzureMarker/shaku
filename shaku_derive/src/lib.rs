//! This crate provides shaku's derive macros.

extern crate proc_macro;
#[macro_use]
extern crate quote;

use crate::error::Error;
use crate::structures::module::ModuleData;
use proc_macro::TokenStream;

mod consts;
mod debug;
mod error;
mod macros;
mod parser;
mod structures;

#[proc_macro_derive(Component, attributes(shaku))]
pub fn component(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    macros::component::expand_derive_component(&input)
        .unwrap_or_else(make_compile_error)
        .into()
}

#[proc_macro_derive(Provider, attributes(shaku))]
pub fn provider(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    macros::provider::expand_derive_provider(&input)
        .unwrap_or_else(make_compile_error)
        .into()
}

/// Create a [`Module`] which is associated with some components and providers.
///
/// ## Builder
/// A `fn builder(submodules...) -> ModuleBuilder<Self>` associated function will be created to make
/// instantiating the module convenient. The arguments are the submodules the module uses.
///
/// ## Module interfaces
/// After the module name, you can add `: MyModuleInterface` where `MyModuleInterface` is the trait
/// that you want this module to implement (ex. `trait MyModuleInterface: HasComponent<MyComponent> {}`).
/// The macro will implement this trait for the module automatically. That is, it is the same as
/// manually adding the line: `impl MyModuleInterface for MyModule {}`. See `MyModuleImpl` in the
/// example below. See also [`ModuleInterface`].
///
/// ## Submodules
/// A module can use components/providers from other modules by explicitly listing the interfaces
/// from each submodule they want to use. Submodules can be abstracted by depending on traits
/// instead of implementations. See `MySecondModule` in the example below.
///
/// ## Generics
/// This macro supports generics at the module level:
/// ```rust
/// use shaku::{module, Component, Interface, HasComponent};
///
/// trait MyComponent<T: Interface>: Interface {}
///
/// #[derive(Component)]
/// #[shaku(interface = MyComponent<T>)]
/// struct MyComponentImpl<T: Interface + Default> {
///     value: T
/// }
/// impl<T: Interface + Default> MyComponent<T> for MyComponentImpl<T> {}
///
/// // MyModuleImpl implements Module and HasComponent<dyn MyComponent<T>>
/// module! {
///     MyModule<T: Interface> where T: Default {
///         components = [MyComponentImpl<T>],
///         providers = []
///     }
/// }
/// # fn main() {}
/// ```
///
/// ## Circular dependencies
/// This macro will detect circular dependencies at compile time. The error that is thrown will be
/// something like
/// "overflow evaluating the requirement `Component2: shaku::component::Component<TestModule>`".
///
/// It is still possible to compile with a circular dependency if the module is manually implemented
/// in a certain way. In that case, there will be a panic during module creation with more details.
///
/// # Examples
/// ```
/// use shaku::{module, Component, Interface, HasComponent};
///
/// trait MyComponent: Interface {}
/// trait MyModule: HasComponent<dyn MyComponent> {}
///
/// #[derive(Component)]
/// #[shaku(interface = MyComponent)]
/// struct MyComponentImpl;
/// impl MyComponent for MyComponentImpl {}
///
/// // MyModuleImpl implements Module, MyModule, and HasComponent<dyn MyComponent>
/// module! {
///     MyModuleImpl: MyModule {
///         components = [MyComponentImpl],
///         providers = []
///     }
/// }
///
/// // MySecondModule implements HasComponent<dyn MyComponent> by using
/// // MyModule's implementation.
/// module! {
///     MySecondModule {
///         components = [],
///         providers = [],
///
///         use MyModule {
///             components = [MyComponent],
///             providers = []
///         }
///     }
/// }
/// # fn main() {}
/// ```
///
/// [`Module`]: trait.Module.html
/// [`ModuleInterface`]: trait.ModuleInterface.html
#[proc_macro]
pub fn module(input: TokenStream) -> TokenStream {
    let module = syn::parse_macro_input!(input as ModuleData);

    macros::module::expand_module_macro(module)
        .unwrap_or_else(make_compile_error)
        .into()
}

fn make_compile_error(error: Error) -> proc_macro2::TokenStream {
    let msg = error.to_string();
    quote! {
        compile_error!(#msg);
    }
}
