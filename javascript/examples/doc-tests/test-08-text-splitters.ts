import 'dotenv/config';
import { init, TextSplitter } from '@infinitibit_gmbh/graphbit';

/**
 * Test 08: Text Splitters
 * Tests text splitting functionality
 */

async function testTextSplitters() {
    console.log('=== Test 08: Text Splitters ===\n');

    try {
        init();

        const longText = 'This is a sample text. '.repeat(100);

        // Character splitter
        const charSplitter = TextSplitter.character({ chunkSize: 100, chunkOverlap: 20 });
        const charChunks = await charSplitter.split(longText);
        console.log(`✅ Character splitter: ${charChunks.length} chunks`);

        // Recursive splitter
        const recursiveSplitter = TextSplitter.recursive({ chunkSize: 100, chunkOverlap: 20 });
        const recursiveChunks = await recursiveSplitter.split(longText);
        console.log(`✅ Recursive splitter: ${recursiveChunks.length} chunks`);

        console.log('\n✅ Text Splitters test passed!');
    } catch (error) {
        console.error('❌ Text Splitters test failed:', error);
        throw error;
    }
}

testTextSplitters();
