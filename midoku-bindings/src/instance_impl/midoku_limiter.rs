use wasmtime::component::{Linker, LinkerInstance};

use crate::state::State;

pub fn map_midoku_limiter(linker: &mut Linker<State>) -> Result<(), Box<dyn std::error::Error>> {
    let mut midoku_limiter_rate_limiter_instance: LinkerInstance<'_, State> =
        linker.instance("midoku:limiter/rate-limiter@0.1.0")?;
    map_midoku_limiter_rate_limiter(&mut midoku_limiter_rate_limiter_instance)?;

    Ok(())
}

fn map_midoku_limiter_rate_limiter(
    instance: &mut LinkerInstance<'_, State>,
) -> Result<(), Box<dyn std::error::Error>> {
    instance.func_wrap("burst", |store, _: ()| {
        let limiter = store.data().limiter();
        let burst = limiter.map(|limiter| limiter.burst());
        Ok((burst,))
    })?;
    instance.func_wrap("period-ms", |store, _: ()| {
        let limiter = store.data().limiter();
        let period_ms = limiter.map(|limiter| limiter.period_ms());
        Ok((period_ms,))
    })?;
    instance.func_wrap("set-burst", |mut store, (burst,): (u32,)| {
        let limiter = store.data_mut().limiter_mut();
        let result = limiter
            .map(|limiter| limiter.set_burst(burst))
            .unwrap_or(Ok(()));
        Ok((result,))
    })?;
    instance.func_wrap("set-period-ms", |mut store, (period_ms,): (u32,)| {
        let limiter = store.data_mut().limiter_mut();
        let result = limiter
            .map(|limiter| limiter.set_period_ms(period_ms))
            .unwrap_or(Ok(()));
        Ok((result,))
    })?;
    instance.func_wrap("ready", |store, _: ()| {
        let limiter = store.data().limiter();
        let ready = limiter.map(|limiter| limiter.ready()).unwrap_or(true);
        Ok((ready,))
    })?;
    instance.func_wrap("block", |store, _: ()| {
        let limiter = store.data().limiter();
        limiter.map(|limiter| limiter.block());
        Ok(())
    })?;
    Ok(())
}
