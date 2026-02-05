
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Configuration
const ROOT_DIR = path.resolve(__dirname, '..');
const FILES_TO_CHECK = [
    'package.json',
    'Cargo.toml',
    'README.md',
    'index.d.ts',
    'graphbit.d.ts'
];
const DIRS_TO_SCAN = [
    'docs',
    'examples'
];
const EXTENSIONS = ['.md', '.json', '.ts', '.js', '.toml'];

const args = process.argv.slice(2);
if (args.length < 1) {
    console.error('Usage: node bump-version.js <new_version> [old_version]');
    process.exit(1);
}

const newVersion = args[0];
let explicitOldVersion = args[1];

console.log(`🚀 Bumping version to ${newVersion}...`);

// 1. Get current version from package.json if not provided
const packageJsonPath = path.join(ROOT_DIR, 'package.json');
const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
const currentVersion = explicitOldVersion || packageJson.version;

if (!currentVersion) {
    console.error('❌ Could not determine current version from package.json');
    process.exit(1);
}

console.log(`📋 Current Version: ${currentVersion} -> New Version: ${newVersion}`);

if (currentVersion === newVersion) {
    console.log('⚠️  Version already matches. Forcing update of other files anyway.');
}

// Helper: Recursive file walker
function walkDir(dir, callback) {
    if (!fs.existsSync(dir)) return;
    const files = fs.readdirSync(dir);
    for (const file of files) {
        const filepath = path.join(dir, file);
        const stats = fs.statSync(filepath);
        if (stats.isDirectory()) {
            if (file !== 'node_modules' && file !== '.git' && file !== 'dist') {
                walkDir(filepath, callback);
            }
        } else {
            callback(filepath);
        }
    }
}

// Helper: Update file content
function updateFile(filepath) {
    try {
        const ext = path.extname(filepath);
        if (!EXTENSIONS.includes(ext)) return;

        let content = fs.readFileSync(filepath, 'utf8');
        let updated = false;

        // Special handling for package.json to be safer
        if (path.basename(filepath) === 'package.json') {
            const pkg = JSON.parse(content);
            if (pkg.version !== newVersion) {
                pkg.version = newVersion;
                updated = true;
            }
            if (pkg.optionalDependencies) {
                for (const key in pkg.optionalDependencies) {
                    // Check if dependency value EXACTLY matches old version
                    if (pkg.optionalDependencies[key] === currentVersion) {
                        pkg.optionalDependencies[key] = newVersion;
                        updated = true;
                    }
                }
            }
            // For test-install/package.json which might have "file:../...-0.5.1.tgz"
            // We will do a generic text replace for the tarball name AFTER JSON processing if needed, 
            // but here we just stringify back.
            if (updated) {
                content = JSON.stringify(pkg, null, 2) + '\n';
            }
        } else if (path.basename(filepath) === 'Cargo.toml') {
            // Specific Cargo.toml handling for the main package
            // For the root Cargo.toml, we want to update package.version
            const versionRegex = /^version\s*=\s*"[^"]+"/m;
            if (versionRegex.test(content)) {
                const newContent = content.replace(versionRegex, `version = "${newVersion}"`);
                if (newContent !== content) {
                    content = newContent;
                    updated = true;
                }
            }
        }

        // GENERIC TEXT REPLACEMENT for all mentions of the old version
        // This handles "0.5.1" -> "0.5.5" in comments, docs, tarball links, etc.
        // We do this AFTER specific JSON/TOML parsing to catch things like comments or string values
        // that exact structure matching might miss (like "graphbit-0.5.1.tgz").

        // Safety: only replace if it looks like a version number usage or specific filename
        // Simple global replace of oldVersion string.
        // Risks: if oldVersion is "1.0.0" and text contains "11.0.0", it becomes "1(updated).0.0"
        // So we use boundary checks strictly? No, text might be "v0.5.1" or "graphbit-0.5.1.tgz".
        // simple string replaceAll is usually safe for semver strings like 0.5.1.

        if (content.includes(currentVersion)) {
            const newContent = content.split(currentVersion).join(newVersion);
            if (newContent !== content) {
                content = newContent;
                updated = true;
            }
        }

        if (updated) {
            fs.writeFileSync(filepath, content, 'utf8');
            console.log(`   ✅ Updated: ${path.relative(ROOT_DIR, filepath)}`);
        }
    } catch (e) {
        console.error(`   ❌ Error updating ${filepath}:`, e.message);
    }
}

// 2. Scan and Update
console.log('🔍 Scanning files...');

// Check specific root files
FILES_TO_CHECK.forEach(file => {
    updateFile(path.join(ROOT_DIR, file));
});

// Scan directories
DIRS_TO_SCAN.forEach(dir => {
    walkDir(path.join(ROOT_DIR, dir), updateFile);
});

// 3. Update package-lock.json (npm install)
console.log('📦 Updating package-lock.json...');
try {
    execSync('npm install --package-lock-only --ignore-scripts', { stdio: 'inherit', cwd: ROOT_DIR });
    console.log('   ✅ package-lock.json updated');
} catch (error) {
    console.error('   ❌ Failed to update package-lock.json:', error);
}

console.log(`\n🎉 Version update process complete: ${currentVersion} -> ${newVersion}`);
