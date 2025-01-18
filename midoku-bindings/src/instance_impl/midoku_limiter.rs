use wasmtime::component::{Linker, LinkerInstance};
use wasmtime::StoreContextMut;

use crate::state::State;

pub fn map_midoku_limiter(linker: &mut Linker<State>) -> Result<(), Box<dyn std::error::Error>> {
    let mut rate_limiter_instance: LinkerInstance<'_, State> =
        linker.instance("midoku:limiter/rate-limiter@0.1.0")?;
    rate_limiter_instance.func_wrap("burst", host_burst)?;
    rate_limiter_instance.func_wrap("period-ms", host_period_ms)?;
    rate_limiter_instance.func_wrap("set-burst", host_set_burst)?;
    rate_limiter_instance.func_wrap("set-period-ms", host_set_period_ms)?;
    rate_limiter_instance.func_wrap("ready", host_ready)?;
    rate_limiter_instance.func_wrap_async("block", |store, params| {
        Box::new(async move { host_block(store, params).await })
    })?;

    Ok(())
}

/// Host function implementation for the `burst` function.
fn host_burst(store: StoreContextMut<State>, _: ()) -> Result<(Option<u32>,), wasmtime::Error> {
    let limiter = store.data().limiter();
    let burst = limiter.map(|limiter| limiter.burst());
    Ok((burst,))
}

/// Host function implementation for the `period-ms` function.
fn host_period_ms(store: StoreContextMut<State>, _: ()) -> Result<(Option<u32>,), wasmtime::Error> {
    let limiter = store.data().limiter();
    let period_ms = limiter.map(|limiter| limiter.period_ms());
    Ok((period_ms,))
}

/// Helper macro to get the limiter or insert a default limiter into the store.
#[doc(hidden)]
macro_rules! get_or_insert_default_limiter {
    ($store:expr) => {{
        match $store.data_mut().limiter_mut() {
            Some(limiter) => limiter,
            None => {
                $store.data_mut().set_limiter(Default::default());
                $store.data_mut().limiter_mut().unwrap()
            }
        }
    }};
}

/// Host function implementation for the `set-burst` function.
fn host_set_burst(
    mut store: StoreContextMut<State>,
    (burst,): (u32,),
) -> Result<(Result<(), ()>,), wasmtime::Error> {
    let limiter = get_or_insert_default_limiter!(store);
    let result = limiter.set_burst(burst);
    Ok((result,))
}

/// Host function implementation for the `set-period-ms` function.
fn host_set_period_ms(
    mut store: StoreContextMut<State>,
    (period_ms,): (u32,),
) -> Result<(Result<(), ()>,), wasmtime::Error> {
    let limiter = get_or_insert_default_limiter!(store);
    let result = limiter.set_period_ms(period_ms);
    Ok((result,))
}

/// Host function implementation for the `ready` function.
fn host_ready(store: StoreContextMut<State>, _: ()) -> Result<(bool,), wasmtime::Error> {
    let limiter = store.data().limiter();
    let ready = limiter.map(|limiter| limiter.ready()).unwrap_or(true);
    Ok((ready,))
}

/// Host function implementation for the `block` function.
async fn host_block(store: StoreContextMut<'_, State>, _: ()) -> Result<(), wasmtime::Error> {
    let Some(limiter) = store.data().limiter() else {
        return Ok(());
    };
    limiter.block().await;
    Ok(())
}
