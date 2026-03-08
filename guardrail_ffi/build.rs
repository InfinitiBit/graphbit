//! Link the prebuilt GuardRail library: static on Unix (libguardrail_ffi.a), import lib on Windows (guardrail_ffi.lib for guardrail_ffi.dll).

use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let lib_dir = env::var("GUARDRAIL_LIB_DIR")
        .unwrap_or_else(|_| Path::new(&manifest_dir).join("../vendor/guardrail").to_string_lossy().into_owned());

    let lib_path = Path::new(&lib_dir);
    if !lib_path.exists() {
        eprintln!("cargo:warning=GuardRail lib dir not found: {} (set GUARDRAIL_LIB_DIR or add vendor/guardrail/)", lib_dir);
        return;
    }

    println!("cargo:rustc-link-search=native={}", lib_path.display());

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    if target_os == "windows" {
        // Link the DLL's import library; guardrail_ffi.dll must be shipped next to the .pyd (see workflow / python-src).
        println!("cargo:rustc-link-lib=dylib=guardrail_ffi");
    } else {
        println!("cargo:rustc-link-lib=static=guardrail_ffi");
    }
}
