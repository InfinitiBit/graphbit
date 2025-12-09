const {
    init,
    DocumentLoader,
    TextSplitter,
    EmbeddingConfig,
    EmbeddingClient,
    AgentBuilder,
    LlmConfig
} = require('../javascript/index');
const fs = require('fs');
const path = require('path');

/**
 * Live RAG Pipeline Test
 * Tests the complete RAG system with real OpenAI API
 */

const OPENAI_API_KEY = process.env.OPENAI_API_KEY;

class SimpleRAG {
    constructor(apiKey) {
        this.apiKey = apiKey;
        this.chunks = [];
        this.embeddings = [];
    }

    async ingestDocuments(docs) {
        console.log('\n📂 Step 1: Loading documents...');

        const loader = new DocumentLoader();
        const loadedDocs = [];

        for (const doc of docs) {
            const content = await loader.loadText(doc.text, doc.source);
            loadedDocs.push(content);
            console.log(`  ✅ Loaded: ${doc.source} (${doc.text.length} chars)`);
        }

        console.log(`\n  Total documents loaded: ${loadedDocs.length}`);
        return loadedDocs;
    }

    async splitDocuments(docs) {
        console.log('\n✂️  Step 2: Splitting documents...');

        const splitter = TextSplitter.recursive(200, 50);

        for (const doc of docs) {
            const docChunks = splitter.split(doc.content);

            docChunks.forEach(chunk => {
                this.chunks.push({
                    text: chunk.content,
                    source: doc.source,
                    chunkIndex: chunk.chunkIndex
                });
            });

            console.log(`  ✅ ${doc.source}: ${docChunks.length} chunks`);
        }

        console.log(`\n  Total chunks: ${this.chunks.length}`);
    }

    async generateEmbeddings() {
        console.log('\n🔢 Step 3: Generating embeddings...');

        const config = EmbeddingConfig.openai(this.apiKey, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const texts = this.chunks.map(c => c.text);

        console.log(`  Processing ${texts.length} chunks...`);

        const response = await client.embed(texts);
        this.embeddings = response.embeddings;

        console.log(`\n  ✅ Generated ${this.embeddings.length} embeddings`);
        console.log(`  Embedding dimension: ${this.embeddings[0].length}`);
    }

    cosineSimilarity(a, b) {
        const dotProduct = a.reduce((sum, val, i) => sum + val * b[i], 0);
        const magnitudeA = Math.sqrt(a.reduce((sum, val) => sum + val * val, 0));
        const magnitudeB = Math.sqrt(b.reduce((sum, val) => sum + val * val, 0));
        return dotProduct / (magnitudeA * magnitudeB);
    }

    async search(query, topK = 3) {
        console.log(`\n🔍 Step 4: Searching for: "${query}"`);

        const config = EmbeddingConfig.openai(this.apiKey, 'text-embedding-3-small');
        const client = new EmbeddingClient(config);

        const queryResponse = await client.embed([query]);
        const queryEmbedding = queryResponse.embeddings[0];

        const results = this.embeddings.map((embedding, idx) => ({
            chunk: this.chunks[idx],
            score: this.cosineSimilarity(queryEmbedding, embedding)
        }));

        results.sort((a, b) => b.score - a.score);
        const topResults = results.slice(0, topK);

        console.log('\n📊 Top results:');
        topResults.forEach((result, idx) => {
            console.log(`\n  ${idx + 1}. Score: ${result.score.toFixed(4)}`);
            console.log(`     Source: ${result.chunk.source}`);
            console.log(`     Text: ${result.chunk.text.substring(0, 80)}...`);
        });

        return topResults;
    }

    async generateAnswer(query, context) {
        console.log('\n🤖 Step 5: Generating answer with AI...');

        const llmConfig = LlmConfig.openai({
            apiKey: this.apiKey,
            model: 'gpt-4o-mini'
        });

        const agent = await new AgentBuilder('RAG Assistant', llmConfig)
            .systemPrompt('You are a helpful assistant. Answer questions based ONLY on the provided context. If the context doesn\'t contain enough information, say so.')
            .temperature(0.3)
            .maxTokens(300)
            .build();

        const contextText = context
            .map((c, idx) => `[${idx + 1}] ${c.chunk.text}`)
            .join('\n\n');

        const prompt = `Context:\n${contextText}\n\nQuestion: ${query}\n\nAnswer:`;

        const answer = await agent.execute(prompt);

        console.log('\n💬 Answer:');
        console.log(`  ${answer}\n`);

        return answer;
    }

    async query(question, topK = 3) {
        const relevantChunks = await this.search(question, topK);
        const answer = await this.generateAnswer(question, relevantChunks);

        return {
            question,
            answer,
            sources: relevantChunks.map(r => ({
                source: r.chunk.source,
                text: r.chunk.text,
                score: r.score
            }))
        };
    }
}

async function main() {
    console.log('🚀 RAG Pipeline Live Test\n');
    console.log('='.repeat(70) + '\n');

    try {
        init();
        console.log('✅ GraphBit initialized');

        const rag = new SimpleRAG(OPENAI_API_KEY);

        // Create sample documents about GraphBit
        const documents = [
            {
                source: 'graphbit-overview.txt',
                text: `GraphBit is a powerful library for building AI agents and workflows. 
               It provides comprehensive tools for document processing, text splitting, 
               embeddings generation, and agent orchestration. The library supports 
               multiple LLM providers including OpenAI, Anthropic, and Ollama. GraphBit 
               is designed for production use with features like retry mechanisms, 
               error handling, and workflow validation.`
            },
            {
                source: 'graphbit-installation.txt',
                text: `To install GraphBit, you can use npm or yarn. Run 'npm install graphbit' 
               or 'yarn add graphbit' in your project directory. The library is available 
               for both Python and JavaScript. For JavaScript, you need Node.js 16 or higher. 
               After installation, import the required modules using require() or ES6 imports.`
            },
            {
                source: 'graphbit-features.txt',
                text: `GraphBit's main features include: Agent creation with AgentBuilder, 
               Workflow management with WorkflowBuilder, Document loading with DocumentLoader, 
               Text splitting with TextSplitter (character, recursive, sentence, token strategies), 
               Embedding generation with EmbeddingClient, Tool system with ToolRegistry, 
               and Workflow execution with Executor. All features are fully async in JavaScript.`
            },
            {
                source: 'graphbit-agents.txt',
                text: `Agents in GraphBit are created using the AgentBuilder class. You can configure 
               agents with custom system prompts, temperature settings, max tokens, and descriptions. 
               Agents support multiple LLM providers and can be integrated into workflows. 
               The Agent.execute() method processes prompts and returns AI-generated responses.
               Multi-agent systems can coordinate multiple specialized agents for complex tasks.`
            }
        ];

        // Build the index
        const docs = await rag.ingestDocuments(documents);
        await rag.splitDocuments(docs);
        await rag.generateEmbeddings();

        console.log('\n✅ RAG system ready!\n');
        console.log('='.repeat(70) + '\n');

        // Test queries
        const queries = [
            'How do I install GraphBit?',
            'What are the main features of GraphBit?',
            'How do I create an agent?'
        ];

        for (const query of queries) {
            console.log('\n' + '='.repeat(70));
            const result = await rag.query(query, 2);

            console.log('\n📝 QUERY RESULT:');
            console.log(`Question: ${result.question}`);
            console.log(`\nAnswer: ${result.answer}`);
            console.log('\nTop Sources:');
            result.sources.forEach((source, idx) => {
                console.log(`  ${idx + 1}. ${source.source} (similarity: ${source.score.toFixed(4)})`);
                console.log(`     "${source.text.substring(0, 60)}..."`);
            });
            console.log('='.repeat(70));

            // Small delay between queries
            await new Promise(resolve => setTimeout(resolve, 1000));
        }

        console.log('\n\n✨ RAG Pipeline test completed successfully!\n');

    } catch (error) {
        console.error('\n❌ Error:', error.message);
        console.error(error.stack);
        process.exit(1);
    }
}

main();
