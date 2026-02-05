/**
 * Text Processing Example
 *
 * This example demonstrates how to use document loaders and text splitters
 * to process documents.
 */

import { init, DocumentLoader, TextSplitter } from 'graphbit';

async function main() {
  // Initialize the GraphBit library
  init();
  console.log('GraphBit initialized');

  // Example 1: Load and split text
  console.log('\n=== Example 1: Text Splitting ===');

  const sampleText = `
GraphBit is a powerful workflow automation framework that enables you to build
complex agentic workflows with ease. It provides a flexible and extensible
architecture for creating AI-powered applications.

The framework supports multiple LLM providers including OpenAI, Anthropic,
and Ollama. You can easily switch between providers or use multiple providers
in the same workflow.

GraphBit also includes powerful document processing capabilities, allowing you
to load and process various document formats including PDF, DOCX, and plain text.
  `.trim();

  // Create a character-based text splitter
  const characterSplitter = TextSplitter.character(100, 20);
  const characterChunks = await characterSplitter.split(sampleText);

  console.log(`Character splitter created ${characterChunks.length} chunks`);
  characterChunks.forEach((chunk: any, index) => {
    console.log(`\nChunk ${index + 1}:`);
    console.log(`  Content: ${chunk.content.substring(0, 50)}...`);
    console.log(`  Range: [${chunk.startIndex}, ${chunk.endIndex}]`);
  });

  // Example 2: Recursive text splitting
  console.log('\n=== Example 2: Recursive Splitting ===');

  const recursiveSplitter = TextSplitter.recursive(150, 30);
  const recursiveChunks = await recursiveSplitter.split(sampleText);

  console.log(`Recursive splitter created ${recursiveChunks.length} chunks`);

  // Example 3: Sentence-based splitting
  console.log('\n=== Example 3: Sentence Splitting ===');

  const sentenceSplitter = TextSplitter.sentence(2);
  const sentenceChunks = await sentenceSplitter.split(sampleText);

  console.log(`Sentence splitter created ${sentenceChunks.length} chunks`);
  sentenceChunks.forEach((chunk: any, index) => {
    console.log(`\nSentence chunk ${index + 1}:`);
    console.log(`  ${chunk.content}`);
  });

  // Example 4: Token-based splitting
  console.log('\n=== Example 4: Token Splitting ===');

  const tokenSplitter = TextSplitter.token(50, 10);
  const tokenChunks = await tokenSplitter.split(sampleText);

  console.log(`Token splitter created ${tokenChunks.length} chunks`);

  // Example 5: Document loading
  console.log('\n=== Example 5: Document Loading ===');

  const loader = new DocumentLoader({
    extractImages: false,
    extractTables: true,
  });

  // Load text directly
  const doc = await loader.loadText(sampleText, 'example.txt');
  console.log('Loaded document:');
  console.log(`  Source: ${doc.source}`);
  console.log(`  Content length: ${doc.content.length} characters`);

  // Split the loaded document
  const docChunks = await characterSplitter.split(doc?.content);
  console.log(`  Split into ${docChunks.length} chunks`);

  // Example 6: Processing a file (if it exists)
  console.log('\n=== Example 6: File Loading ===');

  try {
    const fileDoc = await loader.loadFile('./README.md');
    console.log('Loaded file:');
    console.log(`  Source: ${fileDoc.source}`);
    console.log(`  Content length: ${fileDoc.content.length} characters`);

    // Split the file content
    const fileChunks = await recursiveSplitter.split(fileDoc?.content);
    console.log(`  Split into ${fileChunks.length} chunks`);
  } catch (error) {
    console.log('  README.md not found (this is expected in examples)');
  }

  console.log('\n✓ Text processing examples completed');
}

// Run the example
main().catch(console.error);
