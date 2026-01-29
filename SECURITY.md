# Security Policy

GraphBit is an open-source project licensed under the Apache License 2.0. We welcome reports from the community and take security issues seriously.

## Supported Versions

We aim to address security issues in the latest released version and, when feasible, the current maintenance branch. If you are unsure whether your version is supported, please report the issue and include the version/commit you tested.

## Reporting a Vulnerability

**Please do not open a public GitHub issue for security vulnerabilities.** Use one of the private channels below so we can coordinate a fix and disclosure.

### Preferred: GitHub Security Advisories

1. Go to the [GraphBit repository](https://github.com/InfinitiBit/graphbit)
2. Open the **Security** tab
3. Click **Report a vulnerability**
4. Submit the private advisory form

### Alternative: Email

Send details to: `info@graphbit.ai`

If you do not receive an acknowledgment within 24 hours, you may follow up via the GitHub Security Advisory flow above.

## What to Include

To help us triage quickly, please include:

- A clear description of the issue and why it is a vulnerability
- Steps to reproduce (proof-of-concept code, commands, or a minimal test case)
- Impact assessment (what an attacker can do)
- Affected versions and/or commit SHA
- Any relevant logs, stack traces, configuration notes, and environment details (sanitized)

## Response Timeline

We aim to follow these target timelines for initial handling:

- **24 hours**: Initial acknowledgment
- **72 hours**: Assessment and triage
- **7 days**: Response plan
- **30 days**: Patch development
- **Coordinated disclosure**: After a fix is released (or mitigation guidance is published)

These are targets; complex issues may take longer. We will keep reporters informed of progress.

## Responsible Disclosure Guidelines

We appreciate responsible security research. Please:

### ✅ Do

- Report vulnerabilities privately using the channels above
- Provide detailed reproduction steps and enough information to validate the report
- Allow time for coordinated disclosure after a fix is available

### ❌ Don't

- Publicly disclose details before a fix or coordinated disclosure agreement
- Test against production systems without explicit permission
- Access, modify, or exfiltrate data that does not belong to you

## Security Best Practices (Users and Contributors)

- **API keys and credentials**: Use environment variables; never hardcode secrets in source code, config files, or logs.
- **Environment management**: Use `.env` files for local development; use a secret manager for production deployments.
- **Updates**: Keep GraphBit and its dependencies up to date, especially security-related updates.

---

**Contact**: `info@graphbit.ai`  
**Last Updated**: January 2026
