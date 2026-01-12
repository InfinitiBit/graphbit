# GraphBit PyPI License Enforcement Implementation

## Summary of Changes

This document outlines all modifications made to enforce GraphBit's three-tier licensing model for PyPI distribution.

---

## 1. Python README.md Modifications

### Location: `python/README.md`

### Changes Made:

#### A. Added Prominent License Notice Section

**Position**: Immediately after language selector, before "What is GraphBit?"

**Content**:
- ✅ Intellectual property statement
- ✅ Three-tier licensing model explanation
- ✅ Redistribution prohibition warning
- ✅ Legal consequences of violations
- ✅ Contact information for enterprise licensing

**Format**: GitHub alert box (`> [!IMPORTANT]`) for high visibility on PyPI

#### B. Added Agreement Statement

**Position**: End of "What is GraphBit?" section

**Content**:
> "⚠️ By installing and using GraphBit, you agree to the terms of the GraphBit License."

This creates a binding agreement upon installation.

#### C. Changed "Open Source" to "Source-Available"

**Reason**: GraphBit's license restricts commercial use and redistribution, which violates the Open Source Initiative (OSI) definition.

**Impact**: Legally accurate, avoids misleading users

---

## 2. Post-Install Terminal Message

### Files Created:

#### A. `python/postinstall.py`

**Purpose**: Standalone script for post-installation message

**Features**:
- Large, formatted license notice box
- Three-tier model explanation
- Legal consequences warning
- Contact information
- Copyright notice

**Display**: Prints to stderr after `pip install graphbit`

#### B. `python/python-src/graphbit/_license_notice.py`

**Purpose**: Runtime license notice and compliance checking

**Features**:
1. **First Import Notice**:
   - Shows license notice on first `import graphbit`
   - Creates marker file (`~/.graphbit/.license_notice_shown`)
   - Only shows once per user to avoid annoyance

2. **Enterprise Environment Detection**:
   - Checks for Kubernetes deployment
   - Checks for AWS environment
   - Checks hostname patterns (prod, staging, k8s, etc.)
   - Shows warning if multiple enterprise indicators detected

3. **Compliance Warning**:
   - Alerts users if environment suggests enterprise use
   - Provides contact information for licensing

#### C. Modified `python/python-src/graphbit/__init__.py`

**Change**: Added import of `_license_notice` module at the top

**Effect**: License notice displays automatically on first `import graphbit`

---

## 3. Configuration Updates

### File: `python/pyproject.toml`

**Added**:
```toml
[project.scripts]
graphbit-postinstall = "postinstall:show_license_notice"
```

**Purpose**: Registers post-install script (optional execution)

---

## 4. How It Works

### Installation Flow:

```
User runs: pip install graphbit
    ↓
Package downloads from PyPI
    ↓
Installation completes
    ↓
(Optional) Post-install script runs
    ↓
User sees license notice in terminal
```

### First Import Flow:

```
User runs: import graphbit
    ↓
__init__.py loads
    ↓
_license_notice module imports
    ↓
Checks for marker file (~/.graphbit/.license_notice_shown)
    ↓
If not found:
    - Shows full license notice
    - Creates marker file
    ↓
Checks for enterprise environment indicators
    ↓
If detected:
    - Shows enterprise license warning
    ↓
GraphBit loads normally
```

---

## 5. License Notice Content

### Terminal Message Includes:

1. **Header**: "GRAPHBIT LICENSE NOTICE"
2. **IP Statement**: "GraphBit is the INTELLECTUAL PROPERTY of InfinitiBit GmbH"
3. **Three-Tier Model**:
   - Model A (Free Use): ≤10 employees AND ≤10 users
   - Model B (Free Trial): 30-day evaluation
   - Model C (Enterprise): Required for commercial use
4. **Redistribution Warning**: "REDISTRIBUTION IS PROHIBITED"
5. **Legal Consequences**:
   - License termination
   - Retroactive fees
   - Legal action under German law
   - Injunctive relief and damages
   - Criminal prosecution for copyright infringement
6. **Contact Information**:
   - License URL
   - Enterprise email
   - Website
7. **Agreement Statement**: "By installing and using GraphBit, you agree..."
8. **Copyright**: "Copyright © 2023–2026 InfinitiBit GmbH"

---

## 6. Enterprise Environment Detection

### Indicators Checked:

| Indicator | Detection Method |
|-----------|------------------|
| **Kubernetes** | `KUBERNETES_SERVICE_HOST` environment variable |
| **AWS** | `AWS_EXECUTION_ENV` environment variable |
| **Production hostname** | Hostname contains: prod, production, staging, k8s, cluster |

**Threshold**: If ≥2 indicators detected, show enterprise warning

---

## 7. Legal Protection Mechanisms

### A. Explicit Agreement

**README Statement**:
> "By installing and using GraphBit, you agree to the terms of the GraphBit License."

**Legal Effect**: Creates binding contract upon installation

### B. Notice on First Use

**Implementation**: `_license_notice.py` shows full terms on first import

**Legal Effect**: Ensures user cannot claim ignorance of terms

### C. Prominent PyPI Display

**Implementation**: License notice in README with `> [!IMPORTANT]` alert

**Legal Effect**: Visible to all PyPI visitors before installation

### D. Compliance Warnings

**Implementation**: Enterprise environment detection

**Legal Effect**: Proactive notification of potential violations

---

## 8. Enforcement Strategy

### Phase 1: Education (Passive)

- ✅ Clear README documentation
- ✅ Terminal notices on install/import
- ✅ Enterprise environment warnings

**Goal**: Voluntary compliance through awareness

### Phase 2: Detection (Active)

- Monitor GitHub for GraphBit usage
- Track job postings mentioning GraphBit
- Search for blog posts/conference talks
- Analyze PyPI download statistics

**Goal**: Identify potential violators

### Phase 3: Outreach (Friendly)

- Email companies using GraphBit
- Offer enterprise licensing
- Explain benefits of compliance

**Goal**: Convert to paying customers

### Phase 4: Legal (Last Resort)

- Formal cease & desist letters
- Claim retroactive license fees
- File copyright infringement lawsuits

**Goal**: Enforce compliance legally

---

## 9. Limitations & Realities

### What We CANNOT Do:

| Action | Possible? | Reason |
|--------|-----------|--------|
| Prevent installation | ❌ NO | PyPI doesn't support pre-install validation |
| Track all users | ❌ NO | Privacy concerns + technical limits |
| Enforce user limits | ❌ NO | No remote counting mechanism |
| Block specific companies | ❌ NO | No authentication on PyPI |

### What We CAN Do:

| Action | Possible? | Method |
|--------|-----------|--------|
| Show license terms | ✅ YES | README + terminal notices |
| Detect violations | ⚠️ PARTIAL | Public monitoring + telemetry |
| Legal enforcement | ✅ YES | Sue violators when caught |
| Voluntary compliance | ✅ YES | Clear docs + fair pricing |

---

## 10. Best Practices for Enforcement

### A. Make Compliance Easy

- Clear pricing on website
- Simple enterprise license purchase
- Good support for paying customers

### B. Make Violations Obvious

- Prominent license notices
- Enterprise environment warnings
- Regular compliance reminders

### C. Focus on Big Fish

- Ignore individual developers
- Target companies with >50 employees
- Prioritize high-revenue violators

### D. Be Fair and Transparent

- Friendly first contact
- Reasonable pricing
- Clear escalation process

---

## 11. Next Steps

### Immediate Actions:

1. ✅ Test post-install message locally
2. ✅ Verify license notice displays on first import
3. ✅ Update LICENSE.md copyright year to 2026
4. ✅ Create enterprise pricing page on graphbit.ai

### Short-term (This Month):

1. Set up monitoring for GraphBit usage
2. Create enterprise license template
3. Establish sales process for enterprise customers
4. Document enforcement procedures

### Long-term (This Quarter):

1. Build customer database
2. Implement telemetry (opt-in)
3. Hire/assign license compliance person
4. Create enterprise support infrastructure

---

## 12. Testing the Implementation

### Test Post-Install Message:

```bash
cd python
pip install -e .
# Should see license notice
```

### Test First Import Notice:

```bash
python -c "import graphbit"
# Should see license notice

python -c "import graphbit"
# Should NOT see notice again (marker file exists)
```

### Test Enterprise Detection:

```bash
export KUBERNETES_SERVICE_HOST=localhost
export AWS_EXECUTION_ENV=AWS_ECS_FARGATE
python -c "import graphbit"
# Should see enterprise warning
```

---

## 13. Files Modified/Created

### Modified:
- ✅ `python/README.md` - Added license notice section
- ✅ `python/pyproject.toml` - Added post-install script
- ✅ `python/python-src/graphbit/__init__.py` - Added license notice import

### Created:
- ✅ `python/postinstall.py` - Post-install message script
- ✅ `python/python-src/graphbit/_license_notice.py` - License notice module

---

## 14. Legal Disclaimer

**This implementation provides**:
- ✅ Clear notice of license terms
- ✅ Binding agreement mechanism
- ✅ Detection of potential violations
- ✅ Legal foundation for enforcement

**This implementation does NOT**:
- ❌ Prevent unauthorized use technically
- ❌ Guarantee detection of all violations
- ❌ Replace need for legal counsel
- ❌ Eliminate all free riders

**Recommendation**: Consult with a lawyer specializing in software licensing to review the implementation and enforcement strategy.

---

**Copyright © 2023–2026 InfinitiBit GmbH. All rights reserved.**
