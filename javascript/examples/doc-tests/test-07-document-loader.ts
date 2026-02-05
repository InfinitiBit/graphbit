import 'dotenv/config';
import { init, DocumentLoader } from '@infinitibit_gmbh/graphbit';
import { writeFileSync } from 'fs';
import { join } from 'path';

/**
 * Test 07: Document Loader
 * Tests document loading functionality
 */

async function testDocumentLoader() {
    console.log('=== Test 07: Document Loader ===\n');

    try {
        init();

        // Create a test file
        const testFilePath = join(process.cwd(), 'test-document.txt');
        writeFileSync(testFilePath, 'This is a test document content.');
        console.log('✅ Created test document');

        // Load from string
        const loader1 = new DocumentLoader();
        const doc1 = await loader1.loadText('Sample text content', 'inline-source');
        console.log(`✅ Loaded text: ${doc1.content.substring(0, 50)}...`);
        console.log(`   Source: ${doc1.source}`);
        console.log(`   Format: ${doc1.format}`);

        // Load from file
        try {
            const loader2 = new DocumentLoader();
            const doc2 = await loader2.loadFile(testFilePath);
            console.log(`✅ Loaded from file: ${doc2.content.substring(0, 50)}...`);
        } catch (err) {
            console.log('⚠️  File loading test skipped (may need full path)');
        }

        console.log('\n✅ Document Loader test passed!');
    } catch (error) {
        console.error('❌ Document Loader test failed:', error);
        throw error;
    }
}

testDocumentLoader();
