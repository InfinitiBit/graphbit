"""
Post-installation script for GraphBit.
Displays license notice after pip install.
"""

import sys


def show_license_notice():
    """Display license notice in terminal after installation"""
    
    notice = """
╔══════════════════════════════════════════════════════════════════════════════╗
║                                                                              ║
║                        GRAPHBIT LICENSE NOTICE                               ║
║                                                                              ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  GraphBit is the INTELLECTUAL PROPERTY of InfinitiBit GmbH                   ║
║  Protected by copyright law and licensed under a three-tier model.           ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  THREE-TIER LICENSING MODEL:                                                 ║
║                                                                              ║
║  ✅ MODEL A (Free Use)                                                       ║
║     • Individuals, academic institutions                                     ║
║     • Teams with ≤10 employees AND ≤10 active users                          ║
║     • Non-commercial use only                                                ║
║                                                                              ║
║  ⏱️  MODEL B (Free Trial)                                                    ║
║     • 30-day evaluation period for enterprises                               ║
║     • Internal testing only, no production use                               ║
║                                                                              ║
║  💼 MODEL C (Enterprise) - REQUIRED FOR:                                     ║
║     • Commercial use or production deployments                               ║
║     • Teams with >10 employees or >10 active users                           ║
║     • SaaS offerings or hosted services                                      ║
║     • Embedding in commercial products                                       ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  ⚠️  REDISTRIBUTION IS PROHIBITED under all license tiers                    ║
║      without explicit written permission from InfinitiBit GmbH.              ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  ⚖️  LEGAL CONSEQUENCES OF LICENSE VIOLATIONS:                               ║
║                                                                              ║
║     • Immediate termination of license rights                                ║
║     • Retroactive license fees and financial penalties                       ║
║     • Legal action under German law (Munich jurisdiction)                    ║
║     • Injunctive relief and damages                                          ║
║     • Potential criminal prosecution for copyright infringement              ║
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
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  By installing and using GraphBit, you acknowledge that you have read,       ║
║  understood, and agree to be bound by the GraphBit License terms.            ║
║                                                                              ║
║  Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.                ║
║                                                                              ║
╚══════════════════════════════════════════════════════════════════════════════╝

"""
    
    print(notice, file=sys.stderr)


if __name__ == "__main__":
    show_license_notice()
