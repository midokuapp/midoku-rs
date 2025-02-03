use std::sync::Arc;

use parking_lot::RwLock;
use tokio::runtime::Builder;
use tokio::sync::oneshot;
use tokio::task::LocalSet;
use wasmtime::component::{ComponentNamedList, Lift, Lower, TypedFunc};
use wasmtime::Store;

pub(crate) trait FuncExt<Params, Return>
where
    Params: Send + Sync,
    Return: Send + Sync,
    // Bounds from impl:
    Params: ComponentNamedList + Lower,
    (Return,): ComponentNamedList + Lift,
{
    /// Runs [`wasmtime::Func::call_async`] followed by [`wasmtime::Func::post_return_async`]
    /// but should not be cancelled by the runtime. If cancelled when `call_async` is running,
    /// `post_return_async` is never called and makes the `call_async` never callable again.
    async fn uncancellable_execute<T>(
        &self,
        store: &mut Store<T>,
        params: Params,
    ) -> Result<Return, ()>
    where
        T: Send + Sync + 'static;

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
    async fn uncancellable_execute<T>(
        &self,
        mut store: &mut Store<T>,
        params: Params,
    ) -> Result<Return, ()>
    where
        T: Send + Sync + 'static,
    {
        // function call
        let result = self.call_async(&mut store, params).await.map_err(|_| ())?.0;

        // mandatory cleanup after successful call
        self.post_return_async(&mut store).await.map_err(|_| ())?;

        Ok(result)
    }

    async fn execute<T>(&self, store: Arc<RwLock<Store<T>>>, params: Params) -> Result<Return, ()>
    where
        T: Send + Sync + 'static,
    {
        let func = self.clone();
        let (tx, rx) = oneshot::channel();

        let rt = Builder::new_current_thread().enable_all().build().unwrap();

        std::thread::spawn(move || {
            let local = LocalSet::new();

            local.spawn_local(async move {
                let mut store = store.write();
                tx.send(func.uncancellable_execute(&mut store, params).await)
            });

            rt.block_on(local);
        });

        rx.await.map_err(|_| ())?
    }
}
