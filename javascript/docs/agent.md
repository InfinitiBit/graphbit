# Agent

This document covers AI agent creation and execution in GraphBit JavaScript bindings.

## Overview

Agents are autonomous AI entities configured with specific behaviors, capabilities, and LLM backing. They can process messages, execute tasks, and maintain their own configuration. GraphBit provides a builder pattern for flexible agent construction.

## Class: `AgentBuilder`

Builder for creating agents with custom configuration.

### Constructor

#### `new AgentBuilder(name, llmConfig)`

Create a new agent builder.

**Signature:**

```typescript
constructor(name: string, llmConfig: LlmConfig)
```

**Parameters:**

- `name` (string, required): Agent name
- `llmConfig` (LlmConfig, required): LLM configuration

### 🟢 Verified Example

```javascript
const { AgentBuilder, LlmConfig } = require('graphbit');

const llmConfig = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY,
  model: 'gpt-4o-mini'
});

const builder = new AgentBuilder('Research Assistant', llmConfig);
```

---

### Methods

All builder methods return `this` for method chaining.

#### `description(description)`

Set the agent description.

**Signature:**

```typescript
description(description: string): this
```

**Parameters:**

- `description` (string): Agent description

### 🟢 Verified Example

```javascript
builder.description('An AI assistant specialized in research tasks');
```

---

#### `systemPrompt(prompt)`

Set the system prompt for the agent.

**Signature:**

```typescript
systemPrompt(prompt: string): this
```

**Parameters:**

- `prompt` (string): System prompt that defines agent behavior

### 🟢 Verified Example

```javascript
builder.systemPrompt(
  'You are a helpful research assistant. Provide accurate, ' +
  'well-sourced information and cite your sources.'
);
```

---

#### `temperature(temp)`

Set the LLM temperature for response generation.

**Signature:**

```typescript
temperature(temp: number): this
```

**Parameters:**

- `temp` (number): Temperature value (0.0 to 2.0)
  - Lower values (0.0-0.3): More focused, deterministic
  - Medium values (0.4-0.9): Balanced
  - Higher values (1.0-2.0): More creative, random

### 🟢 Verified Example

```javascript
// For factual tasks
builder.temperature(0.2);

// For creative tasks
builder.temperature(0.9);
```

---

#### `maxTokens(tokens)`

Set the maximum response length in tokens.

**Signature:**

```typescript
maxTokens(tokens: number): this
```

**Parameters:**

- `tokens` (number): Maximum tokens for responses

### 🟢 Verified Example

```javascript
// Short responses
builder.maxTokens(500);

// Longer responses
builder.maxTokens(2000);
```

---

#### `build()`

Build and return the configured agent.

**Signature:**

```typescript
async build(): Promise<Agent>
```

**Returns:** Promise resolving to `Agent` instance

**Throws:** Configuration or initialization errors

### 🟢 Verified Example

```javascript
const agent = await new AgentBuilder('Research Agent', llmConfig)
  .description('Specialized research assistant')
  .systemPrompt('You are a helpful research assistant')
  .temperature(0.5)
  .maxTokens(1000)
  .build();

console.log('Agent created:', await agent.name());
```

---

## Class: `Agent`

Represents a configured AI agent instance.

### Methods

#### `name()`

Get the agent's name.

**Signature:**

```typescript
async name(): Promise<string>
```

**Returns:** Promise resolving to agent name

### 🟢 Verified Example

```javascript
const name = await agent.name();
console.log('Agent name:', name); // "Research Agent"
```

---

#### `description()`

Get the agent's description.

**Signature:**

```typescript
async description(): Promise<string>
```

**Returns:** Promise resolving to description

### 🟢 Verified Example

```javascript
const desc = await agent.description();
console.log('Description:', desc);
```

---

#### `id()`

Get the agent's unique identifier.

**Signature:**

```typescript
async id(): Promise<AgentId>
```

**Returns:** Promise resolving to agent ID object

**AgentId Structure:**

```typescript
interface AgentId {
  uuid: string;  // Unique identifier
}
```

### 🟢 Verified Example

```javascript
const id = await agent.id();
console.log('Agent ID:', id.uuid);
```

---

#### `execute(message)`

Execute the agent with a message and return its response.

**Signature:**

```typescript
async execute(message: string): Promise<string>
```

**Parameters:**

- `message` (string): Input message/prompt for the agent

**Returns:** Promise resolving to agent's response

**Throws:** Execution errors (LLM errors, timeouts, etc.)

### 🟢 Verified Example

```javascript
const response = await agent.execute('What is the capital of France?');
console.log('Response:', response); // "Paris"
```

---

#### `config()`

Get the agent's current configuration.

**Signature:**

```typescript
async config(): Promise<AgentConfig>
```

**Returns:** Promise resolving to agent configuration

**AgentConfig Structure:**

```typescript
interface AgentConfig {
  id: AgentId;
  name: string;
  description: string;
  capabilities: AgentCapability[];
  systemPrompt: string;
  llmConfig: any;
  maxTokens?: number;
  temperature?: number;
}
```

### 🟢 Verified Example

```javascript
const config = await agent.config();
console.log('Temperature:', config.temperature);
console.log('Max tokens:', config.maxTokens);
```

---

## Complete Examples

### Example 1: Simple Q&A Agent

### 🟢 Verified End-to-End Example

```javascript
const { AgentBuilder, LlmConfig } = require('graphbit');

async function createQAAgent() {
  // 1. Configure LLM
  const llmConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o-mini'
  });

  // 2. Build agent
  const agent = await new AgentBuilder('QA Bot', llmConfig)
    .description('Answers questions accurately and concisely')
    .systemPrompt('You are a helpful assistant. Answer questions briefly and accurately.')
    .temperature(0.3)  // Low for factual responses
    .maxTokens(500)
    .build();

  // 3. Execute queries
  const questions = [
    'What is JavaScript?',
    'What is the capital of Japan?',
    'Explain async/await in one sentence'
  ];

  for (const question of questions) {
    console.log(`\nQ: ${question}`);
    const answer = await agent.execute(question);
    console.log(`A: ${answer}`);
  }

  return agent;
}

createQAAgent().catch(console.error);
```

---

### Example 2: Code Review Agent

```javascript
const { AgentBuilder, LlmConfig } = require('graphbit');

async function createCodeReviewAgent() {
  const llmConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o'  // More capable model for code
  });

  const agent = await new AgentBuilder('Code Reviewer', llmConfig)
    .description('Reviews code for best practices and potential issues')
    .systemPrompt(`You are an expert code reviewer. 
      Analyze code for:
      1. Bugs and potential errors
      2. Performance issues
      3. Security vulnerabilities
      4. Code style and best practices
      
      Provide specific, actionable feedback.`)
    .temperature(0.4)
    .maxTokens(2000)
    .build();

  // Review code
  const codeToReview = `
function processUsers(users) {
  for (var i = 0; i < users.length; i++) {
    setTimeout(function() {
      console.log(users[i].name);
    }, 100);
  }
}
  `;

  const review = await agent.execute(
    `Review this code:\n${codeToReview}`
  );

  console.log('Code Review:\n', review);
  
  return agent;
}
```

---

### Example 3: Multi-Agent Conversation

```javascript
const { AgentBuilder, LlmConfig } = require('graphbit');

async function multiAgentConversation() {
  const llmConfig = LlmConfig.anthropic({
    apiKey: process.env.ANTHROPIC_API_KEY,
    model: 'claude-3-5-sonnet-20241022'
  });

  // Create multiple specialized agents
  const researcher = await new AgentBuilder('Researcher', llmConfig)
    .systemPrompt('You gather and present factual information')
    .temperature(0.3)
    .build();

  const analyst = await new AgentBuilder('Analyst', llmConfig)
    .systemPrompt('You analyze data and draw conclusions')
    .temperature(0.5)
    .build();

  const writer = await new AgentBuilder('Writer', llmConfig)
    .systemPrompt('You create clear, engaging summaries')
    .temperature(0.7)
    .build();

  // Topic to explore
  const topic = 'GraphRAG and its applications';

  // 1. Research phase
  console.log('\n--- Research Phase ---');
  const research = await researcher.execute(
    `Research ${topic} and provide key facts`
  );
  console.log('Research:', research);

  // 2. Analysis phase
  console.log('\n--- Analysis Phase ---');
  const analysis = await analyst.execute(
    `Analyze this research: ${research}`
  );
  console.log('Analysis:', analysis);

  // 3. Write summary
  console.log('\n--- Writing Phase ---');
  const summary = await writer.execute(
    `Write a brief summary based on:\nResearch: ${research}\nAnalysis: ${analysis}`
  );
  console.log('Summary:', summary);

  return { researcher, analyst, writer };
}
```

---

### Example 4: Agent with Tool Integration

```javascript
const { AgentBuilder, LlmConfig } = require('graphbit');

async function createToolAwareAgent() {
  const llmConfig = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY,
    model: 'gpt-4o'
  });

  const agent = await new AgentBuilder('Personal Assistant', llmConfig)
    .description('Helps with daily tasks and information retrieval')
    .systemPrompt(`You are a personal assistant with access to tools.
      When asked about weather, time, or calculations, use available tools.
      Otherwise, provide direct answers.`)
    .temperature(0.6)
    .maxTokens(1500)
    .build();

  // Helper function to detect tool use
  async function executeWithTools(message) {
    const response = await agent.execute(message);
    
    // In a real implementation, parse tool calls from response
    // and execute appropriate tools
    
    return response;
  }

  const tasks = [
    'What is 234 * 567?',
    'Summarize the benefits of meditation',
    'What should I prepare for a job interview?'
  ];

  for (const task of tasks) {
    console.log(`\nTask: ${task}`);
    const result = await executeWithTools(task);
    console.log(`Result: ${result}`);
  }

  return agent;
}
```

---

## Best Practices

### 1. System Prompts

```javascript
// ❌ Vague system prompt
builder.systemPrompt('You are helpful');

// ✅ Specific, detailed system prompt
builder.systemPrompt(`You are a technical documentation assistant.
  
  Guidelines:
  - Provide accurate, well-structured information
  - Use examples when explaining concepts
  - Cite sources when referencing external information
  - Admit when you don't know something
  - Keep responses concise but complete`);
```

### 2. Temperature Selection

```javascript
// For facts and accuracy
const factualAgent = await new AgentBuilder('Facts', config)
  .temperature(0.1)  // Very deterministic
  .build();

// For creative tasks
const creativeAgent = await new AgentBuilder('Creative', config)
  .temperature(0.9)  // More varied
  .build();
```

### 3. Token Limits

```javascript
// ❌ Too restrictive
builder.maxTokens(50);  // May cut off important info

// ✅ Appropriate for task
builder.maxTokens(1000);  // Balanced

// ✅ For long-form content
builder.maxTokens(4000);
```

### 4. Error Handling

```javascript
async function safeAgentExecution(agent, message, maxRetries = 3) {
  for (let attempt = 1; attempt <= maxRetries; attempt++) {
    try {
      return await agent.execute(message);
    } catch (error) {
      if (attempt === maxRetries) throw error;
      
      console.log(`Attempt ${attempt} failed, retrying...`);
      await new Promise(r => setTimeout(r, 1000 * attempt));
    }
  }
}
```

---

## Differences from Python

| Aspect | Python | JavaScript |
|--------|--------|------------|
| **Builder creation** | `AgentBuilder(name, llm_config)` | `new AgentBuilder(name, llmConfig)` |
| **Configuration** | `add_capability()`, `add_tool()` | Not available |
| **Build method** | `build()` - sync | `build()` - async (Promise) |
| **Execute method** | `process(message)` or `execute(message)` | `execute(message)` - async |
| **Capabilities** | Can be set via builder | Read-only from config |

**Key Difference:** JavaScript uses async/await throughout, while Python has both sync and async variants.

---

## Common Use Cases

### Use Case 1: Customer Support Agent

```javascript
async function createSupportAgent() {
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY
  });

  return await new AgentBuilder('Support Agent', config)
    .description('Handles customer inquiries professionally')
    .systemPrompt(`You are a customer support agent.
      - Be polite and empathetic
      - Solve problems step-by-step
      - Escalate when necessary
      - Never make promises you can't keep`)
    .temperature(0.5)
    .maxTokens(1000)
    .build();
}
```

### Use Case 2: Data Analysis Agent

```javascript
async function createAnalystAgent() {
  const config = LlmConfig.anthropic({
    apiKey: process.env.ANTHROPIC_API_KEY,
    model: 'claude-3-5-sonnet-20241022'
  });

  return await new AgentBuilder('Data Analyst', config)
    .description('Analyzes data and generates insights')
    .systemPrompt(`You are a data analyst.
      - Identify patterns and trends
      - Provide statistical insights
      - Suggest actionable recommendations
      - Explain findings clearly`)
    .temperature(0.3)
    .maxTokens(2000)
    .build();
}
```

---

## Troubleshooting

### Issue: Agent Build Fails

```javascript
// Problem: LLM not available or misconfigured

try {
  const agent = await builder.build();
} catch (error) {
  if (error.message.includes('model')) {
    console.error('LLM model unavailable - check config');
  } else if (error.message.includes('api key')) {
    console.error('Invalid API key');
  } else {
    console.error('Build failed:', error);
  }
}
```

### Issue: Responses Too Short/Long

```javascript
// Adjust maxTokens based on needs
const agent = await builder
  .maxTokens(2000)  // Increase if too short
  .build();
```

### Issue: Inconsistent Responses

```javascript
// Lower temperature for more consistency
const agent = await builder
  .temperature(0.2)  // More deterministic
  .build();
```

---

## Performance Tips

### Tip 1: Reuse Agents

```javascript
// ❌ Bad: Creating agent for each request
for (const message of messages) {
  const agent = await builder.build();
  await agent.execute(message);
}

// ✅ Good: Reuse single agent
const agent = await builder.build();
for (const message of messages) {
  await agent.execute(message);
}
```

### Tip 2: Parallel Execution

```javascript
// For independent tasks
const agent = await builder.build();
const tasks = ['task1', 'task2', 'task3'];

const results = await Promise.all(
  tasks.map(task => agent.execute(task))
);
```

---

## Related Documentation

- [LLM Configuration](./llm-config.md) - Configure LLMs for agents
- [Executor](./executor.md) - Use agents in workflows
- [Workflow](./workflow.md) - Build multi-agent workflows
