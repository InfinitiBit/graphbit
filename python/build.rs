//! Build script for GraphBit Python bindings
//! Shows license notice after successful build

// build.rs - Rust build script for GraphBit Python bindings
// Shows license notice after successful build

fn main() {
    // This runs during cargo build
    println!("cargo:rerun-if-changed=build.rs");

    // Show license notice after build
    show_license_notice();
}

fn show_license_notice() {
    eprintln!(
        r#"
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                    ✅ GRAPHBIT BUILD SUCCESSFUL                              ║
║                                                                              ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  GraphBit is the intellectual property of InfinitiBit GmbH.                  ║
║  Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.                ║
║                                                                              ║
║  🤝 CONTRIBUTIONS WELCOME via pull requests!                                 ║
║  ⚠️  But redistribution is PROHIBITED without written permission.            ║
║                                                                              ║
║  📄 License: https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md   ║
║  📧 Enterprise: accounting@infinitibit.com                                   ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
"#
    );
}
