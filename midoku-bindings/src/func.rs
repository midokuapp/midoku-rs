use std::sync::Arc;

use tokio::sync::{oneshot, RwLock};
use tokio::task::spawn;
use wasmtime::component::{ComponentNamedList, Lift, Lower, TypedFunc};
use wasmtime::{AsContextMut, Store};

pub(crate) trait FuncExt<Params, Return>
where
    Params: Send + Sync,
    Return: Send + Sync,
    // Bounds from impl:
    Params: ComponentNamedList + Lower,
    (Return,): ComponentNamedList + Lift,
{
    /// Runs [`wasmtime::Func::call_async`] followed by [`wasmtime::Func::post_return_async`].
    async fn execute<T>(&self, store: Arc<RwLock<Store<T>>>, params: Params) -> Result<Return, ()>
    where
        T: Send + Sync + 'static;
}

impl<Params, Return> FuncExt<Params, Return> for TypedFunc<Params, (Return,)>
where
    Params: Send + Sync + 'static,
    Return: Send + Sync + 'static,
    // Bounds from impl:
    Params: ComponentNamedList + Lower,
    (Return,): ComponentNamedList + Lift,
{
    async fn execute<T>(&self, store: Arc<RwLock<Store<T>>>, params: Params) -> Result<Return, ()>
    where
        T: Send + Sync + 'static,
    {
        let func = self.clone();
        let (tx, rx) = oneshot::channel();

        spawn(async move {
            let mut store = store.write().await;

            let result = func
                .call_async(store.as_context_mut(), params)
                .await
                .map(|(ret,)| ret);

            let result = match result {
                Ok(ret) => func
                    .post_return_async(store.as_context_mut())
                    .await
                    .map(|_| ret),
                Err(err) => Err(err),
            };

            _ = tx.send(result);
        });

        rx.await.unwrap().map_err(|_| ())
    }
}
