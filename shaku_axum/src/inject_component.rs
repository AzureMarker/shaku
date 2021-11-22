use axum::async_trait;
use axum::extract::rejection::ExtensionRejection;
use axum::extract::{FromRequest, RequestParts};

use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

use shaku::{HasComponent, Interface, ModuleInterface};

pub struct Inject<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized>(
    Arc<I>,
    PhantomData<M>,
);

#[async_trait]
impl<B, M, I> FromRequest<B> for Inject<M, I>
where
    B: Send,
    M: ModuleInterface + HasComponent<I> + ?Sized,
    I: Interface + ?Sized,
{
    type Rejection = ExtensionRejection;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let module = crate::get_module_from_state::<M, B>(req)?;
        let component = module.resolve();

        Ok(Self(component, PhantomData))
    }
}

impl<M: ModuleInterface + HasComponent<I> + ?Sized, I: Interface + ?Sized> Deref for Inject<M, I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        Arc::as_ref(&self.0)
    }
}
