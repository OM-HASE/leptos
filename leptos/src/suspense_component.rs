use crate::{
    children::{ToChildren, ViewFn},
    IntoView,
};
use leptos_macro::component;
use std::{future::Future, sync::Arc};
use tachys::prelude::FutureViewExt;

/// An async, typed equivalent to [`Children`], which takes a generic but preserves
/// type information to allow the compiler to optimize the view more effectively.
pub struct AsyncChildren<T, F, Fut>(pub(crate) F)
where
    F: Fn() -> Fut,
    Fut: Future<Output = T>;

impl<T, F, Fut> AsyncChildren<T, F, Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = T>,
{
    pub fn into_inner(self) -> F {
        self.0
    }
}

impl<T, F, Fut> ToChildren<F> for AsyncChildren<T, F, Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = T>,
{
    fn to_children(f: F) -> Self {
        AsyncChildren(f)
    }
}

/// TODO docs!
#[component]
pub fn Suspense<Chil, ChilFn, ChilFut>(
    #[prop(optional, into)] fallback: ViewFn,
    children: AsyncChildren<Chil, ChilFn, ChilFut>,
) -> impl IntoView
where
    Chil: IntoView + 'static,
    ChilFn: Fn() -> ChilFut + Send + Clone + 'static,
    ChilFut: Future<Output = Chil> + Send + 'static,
{
    let children = children.into_inner();
    // TODO check this against islands
    move || {
        (children.clone())()
            .suspend()
            .with_fallback(fallback.run())
            .track()
    }
}
