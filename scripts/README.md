# GraphBit Scripts Directory

This directory contains automation scripts and utilities for the GraphBit repository.

## Version Management Scripts

### Core Script: `update_version.py`

The main Python script that handles automated version updates across the entire GraphBit repository.

**Features:**
- ✅ Semantic versioning validation
- ✅ Multi-language support (Rust, Python, JavaScript/TypeScript)
- ✅ Context-aware pattern matching
- ✅ Automatic backup creation
- ✅ Detailed reporting
- ✅ Rollback capability
- ✅ Git repository validation

**Quick Start:**
```bash
# Preview changes
python scripts/update_version.py 0.2.0 --dry-run

# Apply changes with backup
python scripts/update_version.py 0.2.0 --backup

# Get help
python scripts/update_version.py --help
```

### Platform Wrappers

#### Unix/Linux/macOS: `update_version.sh`
```bash
# Make executable (first time only)
chmod +x scripts/update_version.sh

# Use the script
./scripts/update_version.sh 1.0.0-beta.1 --dry-run
```

#### Windows: `update_version.bat`
```cmd
REM Use directly from command prompt
scripts\update_version.bat 1.0.0-beta.1 --dry-run
```

## Supported Version Patterns

The scripts automatically detect and update version numbers in these file types:

### Rust Files
- **Cargo.toml** - `version = "0.1.0"` in `[package]` sections
- Handles multiple Cargo.toml files (root, core/, python/, nodejs/)

### Python Files  
- **pyproject.toml** - `version = "0.1.0"` in `[tool.poetry]` section
- **__init__.py** - `__version__ = "0.1.0"` variables

### JavaScript/TypeScript Files
- **package.json** - `"version": "0.1.0"` fields

## Command-Line Options

| Option | Description | Example |
|--------|-------------|---------|
| `<version>` | Target version (required) | `1.2.3`, `2.0.0-beta.1` |
| `--dry-run` | Preview changes only | `--dry-run` |
| `--backup` | Create timestamped backup | `--backup` |
| `--report` | Generate detailed report | `--report` |
| `--force` | Skip git validation | `--force` |
| `--help` | Show help message | `--help` |

## Examples

### Basic Version Updates
```bash
# Stable release
python scripts/update_version.py 1.0.0

# Pre-release
python scripts/update_version.py 2.0.0-alpha.1

# With build metadata
python scripts/update_version.py 1.0.0+build.123
```

### Development Workflow
```bash
# 1. Preview changes
python scripts/update_version.py 0.2.0 --dry-run

# 2. Apply with safety features
python scripts/update_version.py 0.2.0 --backup --report

# 3. Verify changes
git diff

# 4. Test build
make build && make test

# 5. Commit
git add .
git commit -m "chore: bump version to 0.2.0"
git tag v0.2.0
```

### CI/CD Integration
```yaml
# GitHub Actions example
- name: Update Version
  run: python scripts/update_version.py ${{ inputs.version }} --force --report

- name: Upload Report
  uses: actions/upload-artifact@v3
  with:
    name: version-update-report
    path: version_update_report_*.md
```

## Safety Features

### 1. Repository Validation
- Checks for uncommitted changes
- Validates git repository state
- Warns about potential conflicts

### 2. Backup System
```
target/
└── version_backup_20240102_143022/
    ├── Cargo.toml
    ├── core/Cargo.toml
    ├── pyproject.toml
    └── ...
```

### 3. Context-Aware Updates
- **Cargo.toml**: Only updates `[package]` section versions
- **package.json**: Ignores script commands
- **File filtering**: Excludes build directories and virtual environments

### 4. Error Handling
- Validates all changes before applying
- Automatic rollback on failure
- Comprehensive error messages

## File Discovery

The script automatically finds version files using these patterns:

### Known Configuration Files
- `Cargo.toml` (root level)
- `core/Cargo.toml`
- `python/Cargo.toml` 
- `nodejs/Cargo.toml`
- `pyproject.toml`
- `nodejs/package.json`

### Pattern-Based Discovery
- `**/__init__.py` files containing `__version__`
- Excludes: `.venv/`, `target/`, `node_modules/`, etc.

## Troubleshooting

### Common Issues

**"Repository has uncommitted changes"**
```bash
# Solution 1: Commit changes
git add . && git commit -m "save work"

# Solution 2: Use --force (not recommended)
python scripts/update_version.py 0.2.0 --force
```

**"Invalid version format"**
```bash
# ❌ Invalid
python scripts/update_version.py v1.0.0    # No 'v' prefix
python scripts/update_version.py 1.0       # Missing patch version

# ✅ Valid  
python scripts/update_version.py 1.0.0
python scripts/update_version.py 1.0.0-beta.1
```

**"Python is not installed"**
```bash
# Check Python installation
python --version
python3 --version

# Install Python if needed
# Windows: Download from python.org
# macOS: brew install python
# Ubuntu: sudo apt install python3
```

### Recovery Options

**Restore from backup:**
```bash
# List available backups
ls target/version_backup_*/

# Restore manually
cp -r target/version_backup_20240102_143022/* .
```

**Restore from git:**
```bash
# Undo last commit
git reset --hard HEAD~1

# Restore specific files
git checkout HEAD~1 -- Cargo.toml pyproject.toml
```

## Development

### Adding New File Types

1. **Update patterns** in `VersionManager.__init__()`:
```python
self.patterns['new_format'] = VersionPattern(
    r'^(\s*version\s*:\s*["\'])([^"\']+)(["\'].*?)$',
    r'\g<1>{new_version}\g<3>',
    'New format version field'
)
```

2. **Add file mapping**:
```python
self.file_patterns['config.yaml'] = ['new_format']
```

3. **Update discovery** in `find_version_files()` if needed

4. **Test thoroughly** with `--dry-run`

### Testing

```bash
# Test with dry run
python scripts/update_version.py 9.9.9 --dry-run --force

# Test different version formats
python scripts/update_version.py 1.0.0-alpha.1 --dry-run --force
python scripts/update_version.py 2.0.0+build.123 --dry-run --force

# Test error conditions
python scripts/update_version.py invalid.version  # Should fail
python scripts/update_version.py 1.0.0 # Should warn about git status
```

## Dependencies

### Python Script
- **Python 3.7+** (required)
- **Standard library only** (no external dependencies)
- **Optional**: `toml` package for enhanced TOML parsing

### Shell Scripts
- **Bash 4.0+** (Linux/macOS)
- **Windows 10+** (for ANSI color support in batch script)
- **Git** (for repository validation)

## Contributing

1. **Test changes** thoroughly with `--dry-run`
2. **Update documentation** when adding features
3. **Follow semantic versioning** for script versions
4. **Add error handling** for new functionality
5. **Maintain cross-platform compatibility**

## License

These scripts are part of the GraphBit project and follow the same license terms.
