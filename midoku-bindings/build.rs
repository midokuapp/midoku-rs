use wit_deps::lock_sync;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    lock_sync!("wit").expect("failed to lock root WIT dependencies");

    println!("cargo:rerun-if-changed=wit/deps");
    println!("cargo:rerun-if-changed=wit/deps.lock");
    println!("cargo:rerun-if-changed=wit/deps.toml");

    Ok(())
}
