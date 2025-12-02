# Agent Configuration (JavaScript/TypeScript)

Learn how to configure and customize AI agents in GraphBit.

## Overview

GraphBit agents are AI-powered components that can:
- Process natural language input
- Generate intelligent responses
- Execute with custom configurations
- Integrate with multiple LLM providers

## Creating Agents

### Basic Agent

```typescript
import { AgentBuilder, LlmConfig } from '@graphbit/core';

const config = LlmConfig.openai({
  apiKey: process.env.OPENAI_API_KEY || '',
  model: 'gpt-4o-mini'
});

const agent = await new AgentBuilder('My Agent', config)
  .build();

const response = await agent.execute('What is quantum computing?');
console.log(response);
```

### Agent with Description

```typescript
const agent = await new AgentBuilder('Research Assistant', config)
  .description('Helps with academic research and analysis')
  .build();
```

### Agent with System Prompt

```typescript
const agent = await new AgentBuilder('Code Reviewer', config)
  .description('Reviews code for best practices')
  .systemPrompt('You are an expert code reviewer. Focus on security, performance, and maintainability.')
  .build();
```

### Agent with Configuration

```typescript
const agent = await new AgentBuilder('Creative Writer', config)
  .description('Generates creative content')
  .systemPrompt('You are a creative writer specializing in short stories.')
  .temperature(0.9)        // Higher for creativity
  .maxTokens(2000)         // Longer responses
  .build();
```

## Configuration Options

### Temperature

Controls randomness in responses (0.0 to 2.0):
- **0.0-0.3**: Deterministic, factual
- **0.4-0.7**: Balanced
- **0.8-2.0**: Creative, varied

```typescript
.temperature(0.7)
```

### Max Tokens

Maximum length of response:

```typescript
.maxTokens(1000)  // Limit response length
```

## Agent Methods

### Execute

Send a message and get a response:

```typescript
const response = await agent.execute('Explain machine learning');
console.log(response);
```

### Get Agent Info

```typescript
const name = await agent.name();
const description = await agent.description();

console.log(`Agent: ${name}`);
console.log(`Description: ${description}`);
```

## Complete Examples

### Research Assistant

```typescript
import { AgentBuilder, LlmConfig } from '@graphbit/core';

async function createResearchAssistant() {
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || '',
    model: 'gpt-4o'  // More capable model
  });

  const agent = await new AgentBuilder('Research Assistant', config)
    .description('Analyzes and summarizes research papers')
    .systemPrompt(`You are a research assistant specializing in academic literature analysis. 
      Provide clear, well-structured summaries with key findings and insights.`)
    .temperature(0.3)    // Lower for factual accuracy
    .maxTokens(1500)
    .build();

  return agent;
}

async function main() {
  const agent = await createResearchAssistant();
  
  const query = 'Summarize the latest developments in quantum computing';
  const response = await agent.execute(query);
  
  console.log('Research Assistant Response:');
  console.log(response);
}

main().catch(console.error);
```

### Code Analyzer

```typescript
async function createCodeAnalyzer() {
  const config = LlmConfig.openai({
    apiKey: process.env.OPENAI_API_KEY || ''
  });

  const agent = await new AgentBuilder('Code Analyzer', config)
    .description('Analyzes code for improvements')
    .systemPrompt(`You are an expert code reviewer. Analyze code for:
      1. Security vulnerabilities
      2. Performance issues
      3. Best practices
      4. Code smell
      Provide specific, actionable recommendations.`)
    .temperature(0.2)
    .maxTokens(2000)
    .build();

  return agent;
}
```

### Creative Writer

```typescript
async function createCreativeWriter() {
  const config = LlmConfig.anthropic({
    apiKey: process.env.ANTHROPIC_API_KEY || '',
    model: 'claude-3-5-sonnet-20241022'
  });

  const agent = await new AgentBuilder('Creative Writer', config)
    .description('Generates creative stories and content')
    .systemPrompt(`You are a creative writer with expertise in:
      - Short stories
      - Character development
      - Vivid descriptions
      - Engaging narratives`)
    .temperature(0.9)     // High for creativity
    .maxTokens(3000)      // Allow longer stories
    .build();

  return agent;
}
```

## Best Practices

1. **Choose Appropriate Models**
   - GPT-4o / Claude Opus: Complex reasoning
   - GPT-4o-mini / Claude Sonnet: Balanced tasks
   - GPT-3.5-turbo / Claude Haiku: Simple tasks

2. **Set Clear System Prompts**
   - Define the agent's role and expertise
   - Specify output format
   - Include constraints and guidelines

3. **Optimize Temperature**
   - Lower (0.0-0.3) for factual, deterministic tasks
   - Medium (0.4-0.7) for balanced outputs
   - Higher (0.8-1.0+) for creative tasks

4. **Manage Token Limits**
   - Set appropriate maxTokens for your use case
   - Consider model context windows
   - Balance completeness vs. cost

## Error Handling

```typescript
import { AgentBuilder, LlmConfig } from '@graphbit/core';

async function safeAgentExecution() {
  try {
    const config = LlmConfig.openai({
      apiKey: process.env.OPENAI_API_KEY || ''
    });

    const agent = await new AgentBuilder('Assistant', config)
      .build();

    const response = await agent.execute('Hello!');
    console.log(response);

  } catch (error) {
    console.error('Agent execution failed:', error);
  }
}
```
