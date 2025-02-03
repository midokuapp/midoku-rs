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
        T: Send + 'static;
}

impl<Params, Return> FuncExt<Params, Return> for TypedFunc<Params, (Return,)>
where
    Params: Send + Sync,
    Return: Send + Sync,
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
        T: Send + 'static,
    {
        // function call
        let result = self.call_async(&mut store, params).await.map_err(|_| ())?.0;

        // mandatory cleanup after successful call
        self.post_return_async(&mut store).await.map_err(|_| ())?;

        Ok(result)
    }
}
