#!/usr/bin/env python3
"""
Build script for GraphBit - Shows license notice after build
This runs automatically after `maturin build` or `maturin develop`
"""

import sys


def show_build_license_notice():
    """Display license notice after successful build"""
    
    notice = """
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                    ✅ GRAPHBIT BUILD SUCCESSFUL                              ║
║                                                                              ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  🎉 Thank you for building GraphBit!                                         ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  📋 LICENSE & INTELLECTUAL PROPERTY NOTICE                                   ║
║                                                                              ║
║  GraphBit is the intellectual property of InfinitiBit GmbH.                  ║
║  Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.                ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  🤝 CONTRIBUTIONS WELCOME!                                                   ║
║                                                                              ║
║  We welcome bug fixes, improvements, and new features via pull requests.     ║
║  By contributing, you agree that your contributions will be licensed         ║
║  under the same GraphBit License.                                            ║
║                                                                              ║
║  📖 Contributing Guide:                                                      ║
║     https://github.com/InfinitiBit/graphbit/blob/main/CONTRIBUTING.md       ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  ⚠️  IMPORTANT RESTRICTIONS (Three-Tier License)                             ║
║                                                                              ║
║  ✅ FREE USE (Model A):                                                      ║
║     • Individuals, academic institutions                                     ║
║     • Teams with ≤10 employees AND ≤10 active users                          ║
║     • Non-commercial use only                                                ║
║                                                                              ║
║  ⏱️  FREE TRIAL (Model B):                                                   ║
║     • 30-day evaluation for enterprises                                      ║
║     • Internal testing only, no production use                               ║
║                                                                              ║
║  💼 ENTERPRISE LICENSE REQUIRED (Model C):                                   ║
║     • Commercial use or production deployments                               ║
║     • Teams with >10 employees or >10 active users                           ║
║     • SaaS offerings or hosted services                                      ║
║     • Embedding in commercial products                                       ║
║                                                                              ║
║  ⛔ REDISTRIBUTION PROHIBITED:                                                ║
║     You may NOT redistribute GraphBit (modified or unmodified) without       ║
║     explicit written permission from InfinitiBit GmbH.                       ║
║                                                                              ║
║     Exception: You may fork on GitHub solely for contributing via PRs.       ║
║     Such forks must not be used for distribution or commercial purposes.     ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  ⚖️  LEGAL CONSEQUENCES OF VIOLATIONS:                                       ║
║                                                                              ║
║     • Immediate termination of license rights                                ║
║     • Retroactive license fees and financial penalties                       ║
║     • Legal action under German law (Munich jurisdiction)                    ║
║     • Injunctive relief and damages                                          ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  📄 Full License:                                                            ║
║     https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md            ║
║                                                                              ║
║  📧 Enterprise Licensing:                                                    ║
║     accounting@infinitibit.com                                               ║
║                                                                              ║
║  🌐 Website:                                                                 ║
║     https://graphbit.ai                                                      ║
║                                                                              ║
║  💬 Discord Community:                                                       ║
║     https://discord.com/invite/huVJwkyu                                      ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

"""
    
    print(notice, file=sys.stderr)


if __name__ == "__main__":
    show_build_license_notice()
