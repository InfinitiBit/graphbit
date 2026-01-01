/**
 * Text Splitter Example
 * 
 * Demonstrates various text splitting strategies for processing large documents.
 */

import { TextSplitter } from '../index';

function main() {
    console.log('Text Splitter Example\n');

    const sampleText = `GraphBit is a high-performance AI agent framework. It combines Rust's performance with ease of use in multiple languages.

The framework provides workflow orchestration, agent management, and document processing capabilities. It supports multiple LLM providers including OpenAI, Anthropic, and Ollama.

Text splitting is essential for processing large documents. It helps maintain context while staying within model limits. Different splitting strategies work better for different types of content.`;

    // Character-based splitter
    console.log('=== Character Splitter ===');
    const characterSplitter = TextSplitter.character(100);
    const charChunks = characterSplitter.split(sampleText);

    console.log(`Created ${charChunks.length} chunks`);
    console.log(`First chunk: ${charChunks[0]?.content.substring(0, 50)}...`);

    // Recursive splitter (best for general text)
    console.log('\n=== Recursive Splitter ===');
    const recursiveSplitter = TextSplitter.recursive(150);
    const recChunks = recursiveSplitter.split(sampleText);

    console.log(`Created ${recChunks.length} chunks`);

    // Sentence splitter
    console.log('\n=== Sentence Splitter ===');
    const sentenceSplitter = TextSplitter.sentence();
    const sentChunks = sentenceSplitter.split(sampleText);

    console.log(`Created ${sentChunks.length} chunks`);

    // Token splitter
    console.log('\n=== Token Splitter ===');
    const tokenSplitter = TextSplitter.token(30);
    const tokenChunks = tokenSplitter.split(sampleText);

    console.log(`Created ${tokenChunks.length} chunks`);

    console.log('\nExample completed successfully!');
}

main();
