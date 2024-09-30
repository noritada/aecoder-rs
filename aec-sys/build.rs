use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let dst = cmake::build("libaec-1.1.3");
    println!("cargo:rustc-link-search=native={}", dst.display());

    let out_dir = env::var("OUT_DIR")?;
    let bindings = bindgen::Builder::default()
        .header(format!("{out_dir}/include/libaec.h"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;
    let out_path = PathBuf::from(&out_dir);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
