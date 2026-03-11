use std::env;
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR");
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let default_lib_dir = {
        let base = Path::new(&manifest_dir).join("../vendor/guardrail");
        if target_os == "linux" {
            let triple = match target_arch.as_str() {
                "x86_64"  => "x86_64-unknown-linux-gnu",
                "aarch64" => "aarch64-unknown-linux-gnu",
                other => panic!("unsupported Linux arch: {other}"),
            };
            base.join(triple).to_string_lossy().into_owned()
        } else {
            base.to_string_lossy().into_owned()
        }
    };

    let lib_dir = env::var("GUARDRAIL_LIB_DIR").unwrap_or(default_lib_dir);
    let lib_path = Path::new(&lib_dir);

    if !lib_path.exists() {
        eprintln!(
            "cargo:warning=GuardRail lib dir not found: {} (set GUARDRAIL_LIB_DIR or add vendor/guardrail/)",
            lib_dir
        );
        return;
    }

    println!("cargo:rustc-link-search=native={}", lib_path.display());

    match target_os.as_str() {
        "windows" => println!("cargo:rustc-link-lib=dylib=guardrail_ffi"),
        _         => println!("cargo:rustc-link-lib=static=guardrail_ffi"), // macOS + Linux
    }
}