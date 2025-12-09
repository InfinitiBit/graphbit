const { TextSplitter } = require('../javascript/index');

async function verifyTextSplitterUsage() {
    console.log('Verifying TextSplitter for Documentation...');

    const testText = `This is the first sentence. This is the second sentence! And here is a third one? 
Finally, this is the fourth sentence. We need enough text to properly test splitting.
This paragraph has multiple sentences to verify the splitter works correctly.`;

    // 1. Test character-based splitter
    try {
        const characterSplitter = TextSplitter.character(50, 10);
        console.log('✅ TextSplitter.character() created');

        const charChunks = characterSplitter.split(testText);
        console.log(`✅ Character splitter produced ${charChunks.length} chunks`);

        // Verify chunk structure
        const firstChunk = charChunks[0];
        if (firstChunk.content && typeof firstChunk.startIndex === 'number' &&
            typeof firstChunk.endIndex === 'number' && typeof firstChunk.chunkIndex === 'number') {
            console.log('✅ TextChunk structure verified');
        } else {
            throw new Error('TextChunk structure invalid');
        }
    } catch (error) {
        console.error('❌ Character splitter failed:', error);
        process.exit(1);
    }

    // 2. Test recursive splitter
    try {
        const recursiveSplitter = TextSplitter.recursive(100, 20);
        console.log('✅ TextSplitter.recursive() created');

        const recursiveChunks = recursiveSplitter.split(testText);
        console.log(`✅ Recursive splitter produced ${recursiveChunks.length} chunks`);
    } catch (error) {
        console.error('❌ Recursive splitter failed:', error);
        process.exit(1);
    }

    // 3. Test sentence splitter
    try {
        const sentenceSplitter = TextSplitter.sentence(2, 0);
        console.log('✅ TextSplitter.sentence() created');

        const sentenceChunks = sentenceSplitter.split(testText);
        console.log(`✅ Sentence splitter produced ${sentenceChunks.length} chunks`);
    } catch (error) {
        console.error('❌ Sentence splitter failed:', error);
        process.exit(1);
    }

    // 4. Test token splitter
    try {
        const tokenSplitter = TextSplitter.token(20, 5);
        console.log('✅ TextSplitter.token() created');

        const tokenChunks = tokenSplitter.split(testText);
        console.log(`✅ Token splitter produced ${tokenChunks.length} chunks`);
    } catch (error) {
        console.error('❌ Token splitter failed:', error);
        process.exit(1);
    }

    // 5. Test config() method
    try {
        const splitter = TextSplitter.character(100);
        const config = splitter.config();
        console.log('✅ TextSplitter.config() works');
        console.log('  Config:', JSON.stringify(config, null, 2));
    } catch (error) {
        console.error('❌ config() method failed:', error);
        process.exit(1);
    }

    // 6. Test chunk metadata
    try {
        const splitter = TextSplitter.character(50, 10);
        const chunks = splitter.split(testText);

        console.log('\n📊 Sample chunk details:');
        chunks.slice(0, 2).forEach((chunk, idx) => {
            console.log(`  Chunk ${idx}:`);
            console.log(`    Content: "${chunk.content.substring(0, 30)}..."`);
            console.log(`    Start: ${chunk.startIndex}, End: ${chunk.endIndex}`);
            console.log(`    Index: ${chunk.chunkIndex}`);
            console.log(`    Metadata:`, chunk.metadata);
        });
    } catch (error) {
        console.error('❌ Chunk metadata test failed:', error);
        process.exit(1);
    }

    console.log('\n✨ All TextSplitter methods verified successfully!');
}

verifyTextSplitterUsage().catch(console.error);
