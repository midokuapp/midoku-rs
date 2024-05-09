use wasmtime::component::{Linker, LinkerInstance};

use crate::state::State;

pub fn map_midoku_settings(linker: &mut Linker<State>) -> Result<(), Box<dyn std::error::Error>> {
    let mut midoku_limiter_rate_limiter_instance: LinkerInstance<'_, State> =
        linker.instance("midoku:settings/settings@0.1.0")?;
    map_midoku_settings_get(&mut midoku_limiter_rate_limiter_instance)?;

    Ok(())
}

fn map_midoku_settings_get(
    instance: &mut LinkerInstance<'_, State>,
) -> Result<(), Box<dyn std::error::Error>> {
    instance.func_wrap("get", |store, (key,): (String,)| {
        let settings = store.data().settings();
        let value = settings.get(&key).cloned().ok_or(());
        Ok((value,))
    })?;
    Ok(())
}
