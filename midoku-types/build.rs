use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::Result;
use cargo_witgen::Witgen;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/*");

    let mut buffer = String::new();

    buffer.push_str("// auto-generated file by the build script from midoku-types\n");
    buffer.push_str(format!("package midoku:types@{};", env!("CARGO_PKG_VERSION")).as_str());
    buffer.push_str("\n\ninterface types {\n");
    buffer.push_str(&Witgen::gen_static_from_path(&PathBuf::from(""))?);
    buffer.push_str("}\n\nworld prelude {\n    export types;\n}\n");

    let mut file = File::create("witgen/types.wit")?;
    file.write(buffer.as_bytes())?;

    Ok(())
}
