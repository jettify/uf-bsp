use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let out =
        PathBuf::from(env::var_os("OUT_DIR").ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::NotFound, "OUT_DIR must be set")
        })?);
    fs::copy("memory.x", out.join("memory.x"))?;

    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

    Ok(())
}
