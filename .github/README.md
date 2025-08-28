# GitHub Actions Workflows

This repository includes several GitHub Actions workflows for automated CI/CD:

## Workflows

### 1. **CI** (`.github/workflows/ci.yml`)
- **Triggers**: Push to main/develop, Pull Requests to main
- **Purpose**: Continuous Integration testing
- **Actions**:
  - Tests on multiple OS (Ubuntu, Windows, macOS)
  - Tests on stable and beta Rust
  - Code formatting checks (`cargo fmt`)
  - Linting with Clippy
  - Security audit
  - Documentation build verification
  - Example execution

### 2. **Publish** (`.github/workflows/publish.yml`)
- **Triggers**: Git tags starting with 'v', Manual dispatch
- **Purpose**: Automated publishing to crates.io
- **Actions**:
  - Full CI checks
  - Package verification
  - Dry-run publish
  - Actual publish to crates.io
  - GitHub release creation

### 3. **Release** (`.github/workflows/release.yml`)
- **Triggers**: Manual dispatch only
- **Purpose**: Version bumping and release creation
- **Actions**:
  - Version bumping (patch/minor/major/custom)
  - Git tagging
  - Changelog generation
  - GitHub release creation

### 4. **Documentation** (`.github/workflows/docs.yml`)
- **Triggers**: Push to main, Git tags, Manual dispatch
- **Purpose**: Build and deploy documentation to GitHub Pages
- **Actions**:
  - Generate Rust documentation
  - Deploy to GitHub Pages

## Setup Requirements

### 1. **Secrets Configuration**
Add these secrets to your GitHub repository settings:

- `CARGO_REGISTRY_TOKEN`: Your crates.io API token
  - Get from: https://crates.io/me
  - Go to: Repository Settings → Secrets and variables → Actions

### 2. **GitHub Pages**
Enable GitHub Pages in repository settings:
- Go to: Repository Settings → Pages
- Source: GitHub Actions

### 3. **Branch Protection** (Recommended)
- Require PR reviews for main branch
- Require status checks to pass
- Require branches to be up to date

## Usage

### Publishing a New Version

#### Method 1: Automatic (Recommended)
1. Go to: Actions → Release → Run workflow
2. Choose version bump type (patch/minor/major/custom)
3. The workflow will:
   - Bump version in Cargo.toml
   - Create git tag
   - Push changes
   - Trigger publish workflow automatically

#### Method 2: Manual Git Tags
```bash
# Bump version manually in Cargo.toml
git add Cargo.toml
git commit -m "chore: bump version to v1.2.3"
git tag v1.2.3
git push origin main
git push origin v1.2.3
```

### Development Workflow
1. Create feature branch
2. Make changes
3. Push branch (triggers CI)
4. Create Pull Request (triggers CI)
5. Merge to main (triggers CI + docs)
6. Use Release workflow when ready to publish

## Workflow Files Location
```
.github/
└── workflows/
    ├── ci.yml       # Continuous Integration
    ├── publish.yml  # Publish to crates.io
    ├── release.yml  # Version management
    └── docs.yml     # Documentation deployment
```
