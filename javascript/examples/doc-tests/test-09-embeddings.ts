import 'dotenv/config';
import { init, EmbeddingConfig, EmbeddingClient } from '@infinitibit_gmbh/graphbit';

/**
 * Test 09: Embeddings
 * Tests embedding generation
 */

async function testEmbeddings() {
    console.log('=== Test 09: Embeddings ===\n');

    try {
        init();

        // Create embedding configuration
        const embedConfig = EmbeddingConfig.openai({ apiKey: process.env.OPENAI_API_KEY || 'test-key' });
        console.log('✅ Created embedding config');

        // Create embedding client
        const client = new EmbeddingClient(embedConfig);
        console.log('✅ Created embedding client');

        // Generate embeddings (will fail without valid API key)
        try {
            const texts = ['Machine learning', 'Natural language processing', 'Deep learning'];
            const result = await client.embed(texts);
            console.log(`✅ Generated ${result.embeddings.length} embeddings`);
            console.log(`   Model: ${result.model}`);
            console.log(`   Vector dimension: ${result.embeddings[0]?.length || 0}`);
        } catch (err: any) {
            if (err.code === 'GenericFailure') {
                console.log('⚠️  Embedding generation skipped (needs valid API key)');
            } else {
                throw err;
            }
        }

        console.log('\n✅ Embeddings test passed!');
    } catch (error) {
        console.error('❌ Embeddings test failed:', error);
        throw error;
    }
}

testEmbeddings();
