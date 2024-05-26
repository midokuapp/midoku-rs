use wasmtime::component::{Linker, LinkerInstance};

use crate::state::State;

pub fn map_midoku_limiter(linker: &mut Linker<State>) -> Result<(), Box<dyn std::error::Error>> {
    let mut rate_limiter_instance: LinkerInstance<'_, State> =
        linker.instance("midoku:limiter/rate-limiter@0.1.0")?;
    rate_limiter_instance.func_wrap("burst", host_burst)?;
    rate_limiter_instance.func_wrap("period-ms", host_period_ms)?;
    rate_limiter_instance.func_wrap("set-burst", host_set_burst)?;
    rate_limiter_instance.func_wrap("set-period-ms", host_set_period_ms)?;
    rate_limiter_instance.func_wrap("ready", host_ready)?;
    rate_limiter_instance.func_wrap("block", host_block)?;

    Ok(())
}

fn host_burst(
    store: wasmtime::StoreContextMut<State>,
    _: (),
) -> Result<(Option<u32>,), wasmtime::Error> {
    let limiter = store.data().limiter();
    let burst = limiter.map(|limiter| limiter.burst());
    Ok((burst,))
}

fn host_period_ms(
    store: wasmtime::StoreContextMut<State>,
    _: (),
) -> Result<(Option<u32>,), wasmtime::Error> {
    let limiter = store.data().limiter();
    let period_ms = limiter.map(|limiter| limiter.period_ms());
    Ok((period_ms,))
}

fn host_set_burst(
    mut store: wasmtime::StoreContextMut<State>,
    (burst,): (u32,),
) -> Result<(Result<(), ()>,), wasmtime::Error> {
    let limiter = store.data_mut().limiter_mut();
    let result = limiter
        .map(|limiter| limiter.set_burst(burst))
        .unwrap_or(Ok(()));
    Ok((result,))
}

fn host_set_period_ms(
    mut store: wasmtime::StoreContextMut<State>,
    (period_ms,): (u32,),
) -> Result<(Result<(), ()>,), wasmtime::Error> {
    let limiter = store.data_mut().limiter_mut();
    let result = limiter
        .map(|limiter| limiter.set_period_ms(period_ms))
        .unwrap_or(Ok(()));
    Ok((result,))
}

fn host_ready(store: wasmtime::StoreContextMut<State>, _: ()) -> Result<(bool,), wasmtime::Error> {
    let limiter = store.data().limiter();
    let ready = limiter.map(|limiter| limiter.ready()).unwrap_or(true);
    Ok((ready,))
}

fn host_block(store: wasmtime::StoreContextMut<State>, _: ()) -> Result<(), wasmtime::Error> {
    let limiter = store.data().limiter();
    limiter.map(|limiter| limiter.block());
    Ok(())
}
