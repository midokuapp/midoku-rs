use midoku_settings::types::Value;
use wasmtime::component::{Linker, LinkerInstance};
use wasmtime::StoreContextMut;

use crate::state::State;

pub fn map_midoku_settings(linker: &mut Linker<State>) -> Result<(), Box<dyn std::error::Error>> {
    let mut settings_instance: LinkerInstance<'_, State> =
        linker.instance("midoku:settings/settings@0.1.0")?;
    settings_instance.func_wrap("get", host_get)?;

    Ok(())
}

/// Host function implementation for the `get` function.
fn host_get(
    store: StoreContextMut<State>,
    (key,): (String,),
) -> Result<(Result<Value, ()>,), wasmtime::Error> {
    let settings = store.data().settings();
    let value = settings.get(&key).cloned().ok_or(());
    Ok((value,))
}
