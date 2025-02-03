use std::sync::Arc;

use parking_lot::RwLock;
use tokio::task::{spawn_local, LocalSet};
use wasmtime::component::{ComponentNamedList, Lift, Lower, TypedFunc};
use wasmtime::{AsContextMut, Store};

pub(crate) struct Func<Params, Return>(Arc<TypedFunc<Params, (Return,)>>)
where
    Params: Send + Sync,
    Return: Send + Sync,
    // Bounds from [`TypedFunc`]:
    Params: ComponentNamedList + Lower,
    (Return,): ComponentNamedList + Lift;

impl<Params, Return> From<TypedFunc<Params, (Return,)>> for Func<Params, Return>
where
    Params: Send + Sync,
    Return: Send + Sync,
    // Bounds from [`TypedFunc`]:
    Params: ComponentNamedList + Lower,
    (Return,): ComponentNamedList + Lift,
{
    fn from(value: TypedFunc<Params, (Return,)>) -> Self {
        Func(Arc::new(value))
    }
}

impl<Params, Return> Func<Params, Return>
where
    Params: Send + Sync + 'static,
    Return: Send + Sync + 'static,
    // Bounds from [`TypedFunc`]:
    Params: ComponentNamedList + Lower,
    (Return,): ComponentNamedList + Lift,
{
    async fn call_<T: Send + 'static>(
        store: Arc<RwLock<Store<T>>>,
        func: Arc<TypedFunc<Params, (Return,)>>,
        params: Params,
    ) -> Result<Return, ()> {
        let mut store = store.write();

        // function call
        let result = func
            .call_async(store.as_context_mut(), params)
            .await
            .map_err(|_| ())?
            .0;

        // mandatory cleanup after successful call
        func.post_return_async(store.as_context_mut())
            .await
            .map_err(|_| ())?;

        Ok(result)
    }

    pub async fn call<T: Send + 'static>(
        &self,
        store: Arc<RwLock<Store<T>>>,
        params: Params,
    ) -> Result<Return, ()> {
        let func = self.0.clone();

        let local = LocalSet::new();
        local
            .run_until(async move { spawn_local(Self::call_(store, func, params)).await })
            .await
            .expect("could not join handle")
    }
}
