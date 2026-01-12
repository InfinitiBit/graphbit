"""
GraphBit License Notice Module
Displays license information on first import and provides license validation.
"""

import os
import sys
from pathlib import Path


LICENSE_NOTICE = """
╔══════════════════════════════════════════════════════════════════════════════╗
║                        GRAPHBIT LICENSE NOTICE                               ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  GraphBit is the INTELLECTUAL PROPERTY of InfinitiBit GmbH                   ║
║  Protected by copyright law. Licensed under a three-tier model.              ║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║                                                                              ║
║  🤝 CONTRIBUTIONS WELCOME!                                                   ║
║     We welcome bug fixes, improvements, and new features via pull requests.  ║
║     By contributing, you agree your contributions will be licensed under     ║
║     the same GraphBit License.                                               ║
║                                                                              ║
║     📖 Contributing: https://github.com/InfinitiBit/graphbit/CONTRIBUTING.md║
║                                                                              ║
║  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  ║
║  THREE-TIER LICENSING:                                                       ║
║                                                                              ║
║  ✅ MODEL A (Free): ≤10 employees AND ≤10 users, non-commercial             ║
║  ⏱️  MODEL B (Trial): 30-day evaluation, no production use                  ║
║  💼 MODEL C (Enterprise): REQUIRED for commercial/production use             ║
║                                                                              ║
║  ⚠️  REDISTRIBUTION PROHIBITED without written permission                    ║
║     Exception: Fork on GitHub for PRs only (no distribution/commercial use) ║
║                                                                              ║
║  ⚖️  VIOLATIONS MAY RESULT IN:                                               ║
║     • License termination & retroactive fees                                 ║
║     • Legal action under German law (Munich jurisdiction)                    ║
║     • Injunctive relief and damages                                          ║
║                                                                              ║
║  📄 License: https://github.com/InfinitiBit/graphbit/blob/main/LICENSE.md   ║
║  📧 Enterprise: accounting@infinitibit.com                                   ║
║  🌐 Website: https://graphbit.ai                                             ║
║                                                                              ║
║  By using GraphBit, you agree to the license terms.                          ║
║  Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.                ║
╚══════════════════════════════════════════════════════════════════════════════╝
"""


def show_license_notice_once():
    """
    Show license notice on first import.
    Creates a marker file to avoid showing on every import.
    """
    # Check if notice has been shown
    marker_file = Path.home() / ".graphbit" / ".license_notice_shown"
    
    if not marker_file.exists():
        print(LICENSE_NOTICE, file=sys.stderr)
        
        # Create marker file
        marker_file.parent.mkdir(parents=True, exist_ok=True)
        marker_file.write_text("License notice shown")


def check_license_compliance():
    """
    Basic license compliance check.
    Shows warning if environment suggests enterprise use.
    """
    # Check for enterprise environment indicators
    enterprise_indicators = []
    
    # Check environment variables
    if os.getenv("KUBERNETES_SERVICE_HOST"):
        enterprise_indicators.append("Kubernetes deployment detected")
    
    if os.getenv("AWS_EXECUTION_ENV"):
        enterprise_indicators.append("AWS environment detected")
    
    # Check hostname patterns
    import socket
    hostname = socket.gethostname().lower()
    enterprise_patterns = ['prod', 'production', 'staging', 'k8s', 'cluster']
    
    if any(pattern in hostname for pattern in enterprise_patterns):
        enterprise_indicators.append(f"Enterprise hostname pattern: {hostname}")
    
    # If multiple indicators, show warning
    if len(enterprise_indicators) >= 2:
        warning = f"""
╔══════════════════════════════════════════════════════════════════════════════╗
║                     ⚠️  ENTERPRISE LICENSE REQUIRED                          ║
╠══════════════════════════════════════════════════════════════════════════════╣
║                                                                              ║
║  GraphBit detected enterprise/production environment indicators:             ║
"""
        for indicator in enterprise_indicators:
            warning += f"║  • {indicator:<75}║\n"
        
        warning += """║                                                                              ║
║  Free tier is limited to ≤10 users for non-commercial use.                  ║
║  Enterprise license REQUIRED for production/commercial deployments.          ║
║                                                                              ║
║  📧 Contact: accounting@infinitibit.com                                      ║
║  🌐 Website: https://graphbit.ai/enterprise                                  ║
╚══════════════════════════════════════════════════════════════════════════════╝
"""
        print(warning, file=sys.stderr)


# Show notice on module import (first time only)
show_license_notice_once()

# Check license compliance
check_license_compliance()
