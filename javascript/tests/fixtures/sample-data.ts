/**
 * Sample data and fixtures for testing
 */

export const sampleTexts = {
  short: 'This is a short text.',
  medium: `This is a medium-length text.
It has multiple sentences and paragraphs.
This helps test various text processing scenarios.

The second paragraph provides additional content.
It ensures we have enough data for testing.`,
  long: `Lorem ipsum dolor sit amet, consectetur adipiscing elit.
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris.

Duis aute irure dolor in reprehenderit in voluptate velit esse.
Cillum dolore eu fugiat nulla pariatur.
Excepteur sint occaecat cupidatat non proident.

Sunt in culpa qui officia deserunt mollit anim id est laborum.
Sed ut perspiciatis unde omnis iste natus error sit voluptatem.
Accusantium doloremque laudantium, totam rem aperiam.`.repeat(10),
};

export const sampleDocuments = {
  simple: {
    content: 'This is a simple document.',
    source: 'test.txt',
  },
  withMetadata: {
    content: 'This is a document with metadata.',
    source: 'test-with-meta.txt',
    metadata: {
      author: 'Test Author',
      date: '2024-01-01',
      tags: ['test', 'sample'],
    },
  },
};

export const sampleWorkflows = {
  simple: {
    name: 'Simple Workflow',
    description: 'A simple test workflow',
  },
  complex: {
    name: 'Complex Workflow',
    description: 'A complex multi-step workflow',
    metadata: {
      version: '1.0',
      author: 'Test',
      tags: ['test', 'complex'],
    },
  },
};

export const sampleLlmConfigs = {
  openai: {
    apiKey: 'test-openai-key',
    model: 'gpt-4o-mini',
    temperature: 0.7,
    maxTokens: 1000,
  },
  anthropic: {
    apiKey: 'test-anthropic-key',
    model: 'claude-3-5-sonnet-20241022',
    temperature: 0.7,
    maxTokens: 1000,
  },
  ollama: {
    model: 'llama2',
    baseUrl: 'http://localhost:11434',
    temperature: 0.8,
  },
};

export const sampleAgents = {
  simple: {
    name: 'Simple Agent',
    description: 'A simple test agent',
    systemPrompt: 'You are a helpful assistant.',
  },
  specialized: {
    name: 'Code Generator',
    description: 'An agent specialized in code generation',
    systemPrompt: 'You are an expert programmer. Generate clean, efficient code.',
    capabilities: ['CodeGeneration', 'TextGeneration'],
    temperature: 0.3,
    maxTokens: 2000,
  },
};

export const sampleEmbeddings = {
  texts: [
    'The quick brown fox jumps over the lazy dog.',
    'Machine learning is a subset of artificial intelligence.',
    'GraphBit is a workflow automation framework.',
  ],
  expectedDimensions: {
    'text-embedding-3-small': 1536,
    'text-embedding-3-large': 3072,
  },
};

export const sampleJsonSchemas = {
  simple: {
    type: 'object',
    properties: {
      name: { type: 'string' },
      age: { type: 'number' },
    },
    required: ['name'],
  },
  complex: {
    type: 'object',
    properties: {
      user: {
        type: 'object',
        properties: {
          id: { type: 'string' },
          email: { type: 'string', format: 'email' },
          profile: {
            type: 'object',
            properties: {
              firstName: { type: 'string' },
              lastName: { type: 'string' },
              age: { type: 'number', minimum: 0 },
            },
          },
        },
        required: ['id', 'email'],
      },
      tags: {
        type: 'array',
        items: { type: 'string' },
      },
    },
    required: ['user'],
  },
};
