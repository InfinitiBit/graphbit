// ! Build script for GraphBit Rust Core
//! Shows license notice after successful build

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
║                    ✅ GRAPHBIT CORE BUILD SUCCESSFUL                         ║
║                                                                              ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  GraphBit is the intellectual property of InfinitiBit GmbH.                  ║
║  Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.                ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  🤝 CONTRIBUTIONS WELCOME!                                                   ║
║     We welcome bug fixes, improvements, and new features via pull requests.  ║
║     By contributing, you agree your contributions will be licensed under     ║
║     the same GraphBit License.                                               ║
║                                                                              ║
║     📖 Contributing Guide:                                                   ║
║        https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md    ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  ⚠️  LICENSE RESTRICTIONS (Three-Tier Model)                                 ║
║                                                                              ║
║  ✅ FREE USE (Model A): ≤10 employees AND ≤10 users, non-commercial         ║
║  ⏱️  FREE TRIAL (Model B): 30-day evaluation for enterprises                ║
║  💼 ENTERPRISE (Model C): REQUIRED for commercial/production use             ║
║                                                                              ║
║  ⛔ REDISTRIBUTION PROHIBITED without written permission                     ║
║     Exception: Fork on GitHub for PRs only (no distribution/commercial use) ║
║                                                                              ║
║  ⚖️  VIOLATIONS: License termination, retroactive fees, legal action         ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  📄 License: https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md   ║
║  📧 Enterprise: accounting@infinitibit.com                                   ║
║  🌐 Website: https://graphbit.ai                                             ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝
"#
    );
}
