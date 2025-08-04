use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");

    let dst = cmake::build("libaec-1.1.3");
    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    // libaec >=1.0.5 always builds both of static and shared libraries, and
    // installs following files on Windows:
    // - lib/aec-static.lib
    // - lib/aec.lib
    // - bin/aec.dll
    // - include/libaec.h
    // - lib/szip-static.lib
    // - lib/szip.lib
    // - bin/szip.dll
    // - include/szlib.h
    // - cmake/libaec-config.cmake
    // - cmake/libaec-config-version.cmake
    let lib_name = if env::var("CARGO_CFG_TARGET_OS")? == "windows" {
        "aec-static"
    } else {
        "aec"
    };
    println!("cargo:rustc-link-lib=static={lib_name}");

    let out_dir = env::var("OUT_DIR")?;
    let bindings = bindgen::Builder::default()
        .header(format!("{out_dir}/include/libaec.h"))
        .blocklist_var("^LIBAEC_.*")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;
    let out_path = PathBuf::from(&out_dir);
    bindings.write_to_file(out_path.join("bindings.rs"))?;

    Ok(())
}
