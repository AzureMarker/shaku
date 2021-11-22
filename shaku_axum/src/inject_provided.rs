use std::marker::PhantomData;
use std::ops::Deref;

use axum::async_trait;
use axum::extract::rejection::{ExtensionRejection, MissingExtension};
use axum::extract::{FromRequest, RequestParts};
use shaku::{HasProvider, Interface, ModuleInterface};

pub struct InjectProvided<M: ModuleInterface + HasProvider<I> + ?Sized, I: ?Sized>(
    Box<I>,
    PhantomData<M>,
);

#[async_trait]
impl<B, M, I> FromRequest<B> for InjectProvided<M, I>
where
    B: Send,
    M: ModuleInterface + HasProvider<I> + ?Sized,
    I: ?Sized,
{
    type Rejection = ExtensionRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let module = crate::get_module_from_state::<M, B>(req)?;
        let service = module.provide().map_err(|e| {
            MissingExtension::from_err(format!("Extension {}, {}", std::any::type_name::<M>(), e))
        })?;

        Ok(Self(service, PhantomData))
    }
}

impl<M: ModuleInterface + HasProvider<I> + ?Sized, I: Interface + ?Sized> Deref
    for InjectProvided<M, I>
{
    type Target = I;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}
