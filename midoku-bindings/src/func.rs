use std::sync::Arc;

use parking_lot::RwLock;
use tokio::task::{spawn_local, LocalSet};
use wasmtime::component::{ComponentNamedList, Lift, Lower, TypedFunc};
use wasmtime::Store;

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
    async fn call_async<T>(
        store: &mut Store<T>,
        func: &TypedFunc<Params, (Return,)>,
        params: Params,
    ) -> Result<Return, ()>
    where
        T: Send + 'static,
    {
        func.call_async(store, params)
            .await
            .map_err(|_| ())
            .map(|res| res.0)
    }

    async fn post_return_async<T>(
        store: &mut Store<T>,
        func: &TypedFunc<Params, (Return,)>,
    ) -> Result<(), ()>
    where
        T: Send + 'static,
    {
        func.post_return_async(store).await.map_err(|_| ())
    }

    pub async fn call<T: Send + 'static>(
        &self,
        store: Arc<RwLock<Store<T>>>,
        params: Params,
    ) -> Result<Return, ()> {
        let func = self.0.clone();

        let local = LocalSet::new();
        local
            .run_until(async move {
                spawn_local(async move {
                    let mut store = store.write();
                    let result = Self::call_async(&mut store, &func, params).await?;
                    Self::post_return_async(&mut store, &func).await?;
                    Ok(result)
                })
                .await
            })
            .await
            .expect("could not join handle")
    }
}
