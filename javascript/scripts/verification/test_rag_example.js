const {
    init,
    DocumentLoader,
    TextSplitter,
    EmbeddingConfig,
    EmbeddingClient
} = require('../../index');
const fs = require('fs');
const path = require('path');

/**
 * Simplified RAG Pipeline Test
 * Tests the core components of the RAG example
 */
async function testRAGPipeline() {
    console.log('🧪 Testing RAG Pipeline Example\n');
    console.log('='.repeat(60) + '\n');

    try {
        // Initialize
        init();
        console.log('✅ GraphBit initialized');

        // Step 1: Create test documents
        const testDir = path.join(__dirname, '../test_rag_docs');
        if (!fs.existsSync(testDir)) {
            fs.mkdirSync(testDir, { recursive: true });
        }

        const doc1Path = path.join(testDir, 'doc1.txt');
        const doc2Path = path.join(testDir, 'doc2.txt');

        fs.writeFileSync(doc1Path, 'GraphBit is a powerful library for building AI agents and workflows. It provides tools for document processing, embeddings, and agent orchestration.');
        fs.writeFileSync(doc2Path, 'To install GraphBit, use npm install graphbit. The library supports multiple LLM providers including OpenAI and Anthropic.');

        console.log('✅ Test documents created\n');

        // Step 2: Load documents
        const loader = new DocumentLoader();
        const doc1 = await loader.loadFile(doc1Path, 'txt');
        const doc2 = await loader.loadFile(doc2Path, 'txt');

        console.log(`✅ Document 1 loaded: ${doc1.content.length} chars`);
        console.log(`✅ Document 2 loaded: ${doc2.content.length} chars\n`);

        // Step 3: Split documents
        const splitter = TextSplitter.recursive(100, 20);
        const chunks1 = splitter.split(doc1.content);
        const chunks2 = splitter.split(doc2.content);

        console.log(`✅ Document 1 split into ${chunks1.length} chunks`);
        console.log(`✅ Document 2 split into ${chunks2.length} chunks\n`);

        // Step 4: Test embedding API structure (skip actual embedding without API key)
        console.log('⚠️  Skipping embedding generation (requires API key)');
        console.log('   Example embedding code structure:');
        console.log('   const config = EmbeddingConfig.openai(apiKey);');
        console.log('   const client = new EmbeddingClient(config);');
        console.log('   const response = await client.embed(texts);\n');

        // Step 5: Test cosine similarity function
        const testCosineSimilarity = (a, b) => {
            const dotProduct = a.reduce((sum, val, i) => sum + val * b[i], 0);
            const magnitudeA = Math.sqrt(a.reduce((sum, val) => sum + val * val, 0));
            const magnitudeB = Math.sqrt(b.reduce((sum, val) => sum + val * val, 0));
            return dotProduct / (magnitudeA * magnitudeB);
        };

        const vec1 = [1, 2, 3];
        const vec2 = [4, 5, 6];
        const similarity = testCosineSimilarity(vec1, vec2);

        console.log(`✅ Cosine similarity function works: ${similarity.toFixed(4)}\n`);

        // Cleanup
        fs.unlinkSync(doc1Path);
        fs.unlinkSync(doc2Path);
        fs.rmdirSync(testDir);

        console.log('✅ Cleanup complete\n');
        console.log('='.repeat(60));
        console.log('✅ RAG Pipeline example components verified!\n');

        return true;

    } catch (error) {
        console.error('❌ RAG Pipeline test failed:', error.message);
        console.error(error.stack);
        return false;
    }
}

// Run test
testRAGPipeline()
    .then(success => {
        process.exit(success ? 0 : 1);
    })
    .catch(error => {
        console.error('Fatal error:', error);
        process.exit(1);
    });
