# GitHub Actions Workflows for Tapsilat SDK

This repository includes optimized GitHub Actions workflows for SDK development and publishing.

## 🎯 **Why Simplified Platform Matrix?**

**Original approach**: Multi-platform matrix (Ubuntu + Windows + macOS)  
**Current approach**: Ubuntu-focused with optional cross-platform checks

### Reasoning:
- **Pure Rust SDKs** typically work identically across platforms
- **HTTP/JSON libraries** like `ureq` and `serde` are cross-platform by design
- **Faster CI** - reduces job count from 6 to 2 for daily development
- **Resource efficient** - saves GitHub Actions minutes
- **Cross-platform validation** still available when needed (production-ready workflow)

## Workflows

### 1. **CI** (`.github/workflows/ci.yml`)
- **Triggers**: Push to main/develop, Pull Requests to main
- **Purpose**: Development-friendly CI for SDK library
- **Platform**: Ubuntu only (sufficient for pure Rust SDK)
- **Jobs**:
  - **Code Quality**: Formatting and linting checks
  - **Library Tests**: Compilation and tests on stable & beta Rust (allows failures)
  - **Examples**: Compilation checks (allows failures during development)
  - **Documentation**: Build attempts (allows failures)
  - **Security**: Dependency audit
- **Note**: Uses `continue-on-error: true` to allow development iterations

### 2. **Production Ready Check** (`.github/workflows/production-ready.yml`)
- **Triggers**: Manual dispatch only
- **Purpose**: Strict validation before release
- **Jobs**:
  - **Main Validation**: Comprehensive checks on Ubuntu (stable & beta Rust)
  - **Cross-Platform Check**: Quick compilation test on Windows/macOS
- **All checks must pass** (no `continue-on-error`)

### 3. **Publish** (`.github/workflows/publish.yml`)
- **Triggers**: Git tags starting with 'v*', Manual dispatch
- **Purpose**: Automated SDK library publishing to crates.io
- **Features**: Pre-validation + emergency force-publish option

### 4. **Release** (`.github/workflows/release.yml`)
- **Triggers**: Manual dispatch only
- **Purpose**: Version bumping and release creation

### 5. **Documentation** (`.github/workflows/docs.yml`)
- **Triggers**: Push to main, Git tags, Manual dispatch
- **Purpose**: Build and deploy documentation to GitHub Pages

## 🚀 **Usage Patterns**

### **Daily Development**
```bash
git push origin main  # → CI runs (Ubuntu only, allows failures)
```
**Result**: Fast feedback, doesn't block on compilation issues

### **Pre-Release Validation**
1. **Actions** → **Production Ready Check** → **Run workflow**
2. Validates on multiple Rust versions + quick cross-platform check
3. All checks must pass ✅

### **Publishing**
1. **Actions** → **Release** → **Run workflow** → Choose version bump
2. Auto-triggers publish workflow with strict validation

## 🔧 **Optimization Benefits**

| Aspect | Before | After | Benefit |
|--------|--------|-------|---------|
| **CI Jobs** | 6 (3 OS × 2 Rust) | 2 (1 OS × 2 Rust) | 3x faster |
| **Development** | Strict multi-platform | Ubuntu + lenient | Less blocking |
| **Pre-release** | Same as CI | Dedicated strict workflow | Clear quality gate |
| **Cross-platform** | Always | When needed | Resource efficient |

## 🎯 **When You Might Need Multi-Platform Matrix**

You should consider adding back multi-platform matrix if your SDK:
- ✅ Uses platform-specific dependencies
- ✅ Has different behavior on Windows vs Unix
- ✅ Includes native bindings or system calls
- ✅ Has platform-specific configuration

For HTTP/JSON SDKs like Tapsilat, Ubuntu-only is typically sufficient.

## Setup Requirements

### **Secrets Configuration**
Add to GitHub repository settings → Secrets and variables → Actions:
- `CARGO_REGISTRY_TOKEN`: Your crates.io API token

### **GitHub Pages**
Repository Settings → Pages → Source: GitHub Actions

## Workflow Files
```
.github/workflows/
├── ci.yml               # Daily development (Ubuntu, lenient)
├── production-ready.yml # Pre-release validation (strict + cross-platform)
├── publish.yml          # Publish to crates.io
├── release.yml          # Version management
└── docs.yml             # Documentation deployment
```
