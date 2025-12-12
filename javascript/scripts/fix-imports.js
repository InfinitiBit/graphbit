/**
 * Fix Import Paths Script
 * 
 * Automatically fixes incorrect import paths across the codebase.
 * Converts:
 * - require('../../index') → require('../../index')
 * - require('../index') → require('../index')
 * - from '@graphbit/core' → from '../index' (in examples/tests)
 */

const fs = require('fs');
const path = require('path');

const replacements = [
  {
    pattern: /require\(['"]\.\.\/javascript\/index['"]\)/g,
    replacement: "require('../../index')",
    description: "Fix ../javascript/index → ../../index"
  },
  {
    pattern: /require\(['"]\.\/javascript\/index['"]\)/g,
    replacement: "require('../index')",
    description: "Fix ./javascript/index → ../index"
  }
];

function fixFile(filePath) {
  try {
    let content = fs.readFileSync(filePath, 'utf8');
    let modified = false;
    let changes = [];

    replacements.forEach(({ pattern, replacement, description }) => {
      const matches = content.match(pattern);
      if (matches) {
        content = content.replace(pattern, replacement);
        modified = true;
        changes.push(`${description} (${matches.length} occurrence(s))`);
      }
    });

    if (modified) {
      fs.writeFileSync(filePath, content, 'utf8');
      console.log(`✅ Fixed: ${filePath}`);
      changes.forEach(change => console.log(`   - ${change}`));
      return true;
    }
    
    return false;
  } catch (error) {
    console.error(`❌ Error fixing ${filePath}:`, error.message);
    return false;
  }
}

function scanDirectory(dir, extensions = ['.js', '.ts']) {
  const files = [];
  
  function scan(currentDir) {
    const entries = fs.readdirSync(currentDir, { withFileTypes: true });
    
    for (const entry of entries) {
      const fullPath = path.join(currentDir, entry.name);
      
      if (entry.isDirectory()) {
        // Skip node_modules, build, target directories
        if (!['node_modules', 'build', 'target', 'dist'].includes(entry.name)) {
          scan(fullPath);
        }
      } else if (entry.isFile()) {
        const ext = path.extname(entry.name);
        if (extensions.includes(ext)) {
          files.push(fullPath);
        }
      }
    }
  }
  
  scan(dir);
  return files;
}

function main() {
  console.log('='.repeat(70));
  console.log(' GraphBit Import Path Fixer');
  console.log('='.repeat(70));
  console.log();

  const directories = [
    path.join(__dirname, '../scripts'),
    path.join(__dirname, '../examples'),
    path.join(__dirname, '../tests')
  ];

  let totalFiles = 0;
  let fixedFiles = 0;

  directories.forEach(dir => {
    console.log(`\n📁 Scanning: ${path.basename(dir)}/`);
    console.log('-'.repeat(70));
    
    const files = scanDirectory(dir);
    console.log(`Found ${files.length} files to check\n`);
    
    files.forEach(file => {
      totalFiles++;
      if (fixFile(file)) {
        fixedFiles++;
      }
    });
  });

  console.log();
  console.log('='.repeat(70));
  console.log(' Summary');
  console.log('='.repeat(70));
  console.log(`Total files scanned: ${totalFiles}`);
  console.log(`Files fixed: ${fixedFiles}`);
  console.log(`Files unchanged: ${totalFiles - fixedFiles}`);
  console.log();
  
  if (fixedFiles > 0) {
    console.log('✅ Import paths fixed successfully!');
  } else {
    console.log('ℹ️  No files needed fixing');
  }
}

if (require.main === module) {
  main();
}

module.exports = { fixFile, scanDirectory };

