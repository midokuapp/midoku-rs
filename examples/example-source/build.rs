use anyhow::{Context, Result};
use wit_deps::lock_sync;

fn main() -> Result<()> {
    lock_sync!("wit").context("failed to lock root WIT dependencies")?;

    println!("cargo:rerun-if-changed=wit/deps");
    println!("cargo:rerun-if-changed=wit/deps.lock");
    println!("cargo:rerun-if-changed=wit/deps.toml");

    Ok(())
}
