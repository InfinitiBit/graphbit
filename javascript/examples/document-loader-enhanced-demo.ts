/**
 * Enhanced DocumentLoader Demo
 * 
 * Demonstrates the new helper methods:
 * - supportedTypes() - Get list of supported formats
 * - detectDocumentType() - Auto-detect file type
 *
 * Run with: npx ts-node examples/document-loader-enhanced-demo.ts
 */

import { DocumentLoader } from '../index';

async function main() {
  console.log('='.repeat(70));
  console.log(' GraphBit Enhanced DocumentLoader Demo');
  console.log('='.repeat(70));
  console.log();

  // Example 1: Supported Types
  console.log('Example 1: Check Supported Document Types');
  console.log('-'.repeat(70));
  const supportedTypes = DocumentLoader.supportedTypes();
  console.log('Supported document types:');
  supportedTypes.forEach(type => {
    console.log(`  ✅ .${type}`);
  });
  console.log(`\nTotal: ${supportedTypes.length} types supported\n`);

  // Example 2: Auto-detect Document Type
  console.log('Example 2: Auto-detect Document Type');
  console.log('-'.repeat(70));
  const testFiles = [
    'document.pdf',
    'data.json',
    'report.docx',
    'notes.txt',
    'spreadsheet.csv',
    'config.xml',
    'page.html',
    'unknown.xyz',
    'README',
    'my.file.name.pdf'
  ];

  console.log('Testing file type detection:\n');
  testFiles.forEach(file => {
    const type = DocumentLoader.detectDocumentType(file);
    if (type) {
      console.log(`  ✅ ${file.padEnd(25)} → ${type}`);
    } else {
      console.log(`  ❌ ${file.padEnd(25)} → not supported`);
    }
  });
  console.log();

  // Example 3: Smart Loading Pattern
  console.log('Example 3: Smart Loading Pattern');
  console.log('-'.repeat(70));
  console.log('// Automatically detect and load any supported file\n');
  console.log('async function smartLoad(filePath: string) {');
  console.log('  // Auto-detect type');
  console.log('  const type = DocumentLoader.detectDocumentType(filePath);');
  console.log('  ');
  console.log('  if (!type) {');
  console.log('    throw new Error(`Unsupported file type: ${filePath}`);');
  console.log('  }');
  console.log('  ');
  console.log('  // Load with detected type');
  console.log('  const loader = new DocumentLoader();');
  console.log('  return await loader.loadFile(filePath, type);');
  console.log('}');
  console.log();
  console.log('// Usage:');
  console.log('const doc = await smartLoad("report.pdf");');
  console.log('console.log(doc?.content);');
  console.log();

  // Example 4: Batch Validation
  console.log('Example 4: Batch File Validation');
  console.log('-'.repeat(70));
  const batchFiles = [
    'report1.pdf',
    'report2.pdf',
    'data.csv',
    'invalid.xyz',
    'notes.txt',
    'config.unknown'
  ];

  console.log('Validating batch of files:\n');
  const validFiles = batchFiles.filter(file => {
    const type = DocumentLoader.detectDocumentType(file);
    return type !== null;
  });

  const invalidFiles = batchFiles.filter(file => {
    const type = DocumentLoader.detectDocumentType(file);
    return type === null;
  });

  console.log(`✅ Valid files (${validFiles.length}):`);
  validFiles.forEach(file => console.log(`   - ${file}`));
  
  console.log(`\n❌ Invalid files (${invalidFiles.length}):`);
  invalidFiles.forEach(file => console.log(`   - ${file}`));
  console.log();

  // Example 5: Type-specific Processing
  console.log('Example 5: Type-specific Processing');
  console.log('-'.repeat(70));
  console.log('async function processFiles(files: string[]) {');
  console.log('  const loader = new DocumentLoader();');
  console.log('  ');
  console.log('  for (const file of files) {');
  console.log('    const type = DocumentLoader.detectDocumentType(file);');
  console.log('    ');
  console.log('    if (!type) {');
  console.log('      console.warn(`Skipping unsupported file: ${file}`);');
  console.log('      continue;');
  console.log('    }');
  console.log('    ');
  console.log('    const doc = await loader.loadFile(file, type);');
  console.log('    ');
  console.log('    // Type-specific processing');
  console.log('    switch (type) {');
  console.log('      case "pdf":');
  console.log('        await processPDF(doc);');
  console.log('        break;');
  console.log('      case "json":');
  console.log('        await processJSON(doc);');
  console.log('        break;');
  console.log('      case "csv":');
  console.log('        await processCSV(doc);');
  console.log('        break;');
  console.log('      default:');
  console.log('        await processGeneric(doc);');
  console.log('    }');
  console.log('  }');
  console.log('}');
  console.log();

  // Example 6: Validation Before Upload
  console.log('Example 6: Validation Before Upload');
  console.log('-'.repeat(70));
  console.log('function validateUpload(file: File) {');
  console.log('  const type = DocumentLoader.detectDocumentType(file.name);');
  console.log('  ');
  console.log('  if (!type) {');
  console.log('    return {');
  console.log('      valid: false,');
  console.log('      error: "Unsupported file type"');
  console.log('    };');
  console.log('  }');
  console.log('  ');
  console.log('  const supported = DocumentLoader.supportedTypes();');
  console.log('  if (!supported.includes(type)) {');
  console.log('    return {');
  console.log('      valid: false,');
  console.log('      error: `Type ${type} not supported`');
  console.log('    };');
  console.log('  }');
  console.log('  ');
  console.log('  return { valid: true, type };');
  console.log('}');
  console.log();

  console.log('='.repeat(70));
  console.log(' Summary');
  console.log('='.repeat(70));
  console.log();
  console.log('✅ New DocumentLoader Features:');
  console.log('   - supportedTypes() - Get list of supported formats');
  console.log('   - detectDocumentType() - Auto-detect from file path');
  console.log();
  console.log('💡 Benefits:');
  console.log('   - No need to manually specify document type');
  console.log('   - Validate files before processing');
  console.log('   - Build smart file handling systems');
  console.log('   - Better error messages for users');
  console.log();
  console.log('📚 Use Cases:');
  console.log('   - File upload validation');
  console.log('   - Batch document processing');
  console.log('   - Dynamic file handling');
  console.log('   - User-friendly error messages');
  console.log();
}

main().catch(console.error);

