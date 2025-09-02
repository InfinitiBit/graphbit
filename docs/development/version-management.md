# GraphBit Version Management

This document describes the automated version management system for the GraphBit repository, which ensures consistent version numbers across all components and file formats.

## Overview

The GraphBit repository contains multiple components written in different languages (Rust, Python, TypeScript/JavaScript) with various configuration file formats. The version management system automatically updates version numbers across all these files while maintaining data integrity and following semantic versioning principles.

## Components

### 1. Python Script (`scripts/update_version.py`)

The core version management script written in Python that:

- **Validates semantic version format** (MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD])
- **Discovers version files** automatically across the repository
- **Updates versions** using context-aware pattern matching
- **Creates backups** before making changes (optional)
- **Generates reports** of all changes made
- **Provides rollback capability** in case of errors

### 2. Shell Script (`scripts/update_version.sh`)

A Unix/Linux shell wrapper that provides:

- **Prerequisites checking** (Python availability, repository validation)
- **Enhanced user interface** with colored output
- **Additional validation** before executing the Python script
- **Cross-platform compatibility** for Unix-like systems

### 3. Batch Script (`scripts/update_version.bat`)

A Windows batch wrapper that provides:

- **Windows-specific implementation** of the shell script functionality
- **Command-line argument parsing** compatible with Windows
- **ANSI color support** for modern Windows terminals
- **Same validation and safety features** as the shell script

## Supported File Types

The system automatically detects and updates version numbers in:

### Rust Configuration Files
- **`Cargo.toml`** files (root, core/, python/, nodejs/)
  - Updates only the `version` field in the `[package]` section
  - Ignores dependency versions to avoid conflicts

### Python Configuration Files
- **`pyproject.toml`** - Updates the `version` field in `[tool.poetry]` section
- **`__init__.py`** files - Updates `__version__ = "x.x.x"` variables

### JavaScript/TypeScript Configuration Files
- **`package.json`** files - Updates the `"version"` field
  - Ignores script commands that contain "version" in their names

## Usage

### Basic Usage

```bash
# Update to a new stable version
python scripts/update_version.py 0.2.0

# Preview changes without applying them
python scripts/update_version.py 0.2.0 --dry-run

# Update with backup and detailed report
python scripts/update_version.py 0.2.0 --backup --report
```

### Cross-Platform Wrappers

```bash
# Unix/Linux/macOS
./scripts/update_version.sh 1.0.0-beta.1 --dry-run

# Windows
scripts\update_version.bat 1.0.0-beta.1 --dry-run
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Show what would be changed without making actual changes |
| `--backup` | Create timestamped backup before making changes |
| `--report` | Generate detailed markdown report of all changes |
| `--force` | Skip repository state validation (git status check) |
| `--help` | Show detailed help message with examples |

## Semantic Versioning Support

The system fully supports semantic versioning (SemVer) format:

### Stable Releases
- `1.0.0` - Major release
- `1.2.3` - Minor/patch release

### Pre-Release Versions
- `1.0.0-alpha.1` - Alpha release
- `1.0.0-beta.2` - Beta release  
- `2.0.0-rc.1` - Release candidate

### Build Metadata
- `1.0.0+build.123` - With build information
- `1.0.0-beta.1+build.456` - Pre-release with build info

## Safety Features

### 1. Repository State Validation
- Checks for uncommitted changes in git
- Warns about potential conflicts
- Can be bypassed with `--force` flag

### 2. Backup System
- Creates timestamped backups in `target/version_backup_YYYYMMDD_HHMMSS/`
- Preserves directory structure
- Enables rollback functionality

### 3. Context-Aware Pattern Matching
- **Cargo.toml**: Only updates versions in `[package]` sections
- **package.json**: Ignores script commands containing "version"
- **File filtering**: Excludes virtual environments and build directories

### 4. Atomic Updates
- All changes are validated before applying
- Rollback capability if any file update fails
- Comprehensive error handling and reporting

## Integration with Development Workflow

### Recommended Release Process

1. **Prepare for release**
   ```bash
   # Ensure clean working directory
   git status
   
   # Run tests
   make test
   ```

2. **Update version**
   ```bash
   # Preview changes
   python scripts/update_version.py 0.2.0 --dry-run
   
   # Apply changes with backup
   python scripts/update_version.py 0.2.0 --backup --report
   ```

3. **Verify and commit**
   ```bash
   # Review changes
   git diff
   
   # Test build
   make build
   
   # Commit changes
   git add .
   git commit -m "chore: bump version to 0.2.0"
   
   # Tag release
   git tag v0.2.0
   ```

### CI/CD Integration

The scripts can be integrated into CI/CD pipelines:

```yaml
# Example GitHub Actions workflow
- name: Update Version
  run: |
    python scripts/update_version.py ${{ github.event.inputs.version }} --force
    
- name: Commit Changes
  run: |
    git config --local user.email "action@github.com"
    git config --local user.name "GitHub Action"
    git add .
    git commit -m "chore: bump version to ${{ github.event.inputs.version }}"
```

## Troubleshooting

### Common Issues

1. **"Repository has uncommitted changes"**
   - Commit or stash changes before running
   - Use `--force` to bypass (not recommended)

2. **"Python is not installed"**
   - Install Python 3.7+ and ensure it's in PATH
   - Use `python3` command if `python` is not available

3. **"Invalid version format"**
   - Ensure version follows semantic versioning: `MAJOR.MINOR.PATCH`
   - Pre-release identifiers must be alphanumeric with dots/hyphens

### Recovery

If something goes wrong:

1. **Check for backups**
   ```bash
   ls target/version_backup_*/
   ```

2. **Manual rollback**
   ```bash
   # Restore from git (if committed)
   git checkout HEAD~1 -- .
   
   # Or restore from backup
   cp -r target/version_backup_TIMESTAMP/* .
   ```

## File Structure

```
scripts/
├── update_version.py      # Core Python script
├── update_version.sh      # Unix/Linux wrapper
└── update_version.bat     # Windows wrapper

docs/development/
└── version-management.md  # This documentation

target/
└── version_backup_*/      # Automatic backups (when --backup used)
```

## Contributing

When adding new file types or patterns:

1. Update the `patterns` dictionary in `VersionManager.__init__()`
2. Add file type mappings in `file_patterns`
3. Update `_get_applicable_patterns()` method if needed
4. Add tests for the new pattern
5. Update this documentation

## Security Considerations

- **Backup sensitive files**: The backup system preserves all file permissions
- **Git integration**: Repository state checks help prevent conflicts
- **Pattern validation**: Context-aware matching prevents accidental changes
- **Rollback capability**: Failed updates can be reverted automatically
