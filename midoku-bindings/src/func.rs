use wasmtime::component::{ComponentNamedList, Lift, Lower, TypedFunc};
use wasmtime::{AsContextMut, Store};

pub(crate) struct Func<Params, Return>(TypedFunc<Params, (Return,)>)
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
        Func(value)
    }
}

impl<Params, Return> Func<Params, Return>
where
    Params: Send + Sync,
    Return: Send + Sync,
    // Bounds from [`TypedFunc`]:
    Params: ComponentNamedList + Lower,
    (Return,): ComponentNamedList + Lift,
{
    pub async fn call<T: Send>(&self, store: &mut Store<T>, params: Params) -> Result<Return, ()> {
        // function call
        let result = self
            .0
            .call_async(store.as_context_mut(), params)
            .await
            .map_err(|_| ())?
            .0;

        // mandatory cleanup after successful call
        self.0
            .post_return_async(store.as_context_mut())
            .await
            .map_err(|_| ())?;

        Ok(result)
    }
}
