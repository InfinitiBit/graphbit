# Multi-Agent System Example

**Level:** Advanced  
**Estimated Time:** 45 minutes  
**Prerequisites:** OpenAI or Anthropic API key

## Overview

This example demonstrates building a **collaborative multi-agent system** where specialized agents work together to accomplish complex tasks. You'll learn:

1. Creating specialized agents with different roles
2. Orchestrating agent workflows
3. Agent communication patterns
4. Aggregating multi-agent results
5. Error handling in multi-agent systems

---

## Use Case: Research Report Generation

We'll build a system with three specialized agents:

- **Researcher**: Gathers information
- **Analyst**: Analyzes and synthesizes data
- **Writer**: Creates the final report

---

## Complete Code

```javascript
const { 
  init,
  AgentBuilder,
  LlmConfig,
  WorkflowBuilder
} = require('@infinitibit_gmbh/graphbit');
require('dotenv').config();

/**
 * Multi-Agent Orchestrator
 * Coordinates multiple specialized agents
 */
class MultiAgentSystem {
  constructor(apiKey, provider = 'openai') {
    this.apiKey = apiKey;
    this.provider = provider;
    this.agents = {};
  }

  /**
   * Initialize all specialized agents
   */
  async initializeAgents() {
    console.log('🤖 Initializing agents...\n');

    const llmConfig = this.provider === 'openai'
      ? LlmConfig.openai({
          apiKey: this.apiKey,
          model: 'gpt-4o-mini'
        })
      : LlmConfig.anthropic({
          apiKey: this.apiKey,
          model: 'claude-3-5-sonnet-20241022'
        });

    // Researcher Agent: Gathers information
    this.agents.researcher = await new AgentBuilder('Researcher',llmConfig)
      .description('Gathers and organizes information on topics')
      .systemPrompt(`You are a thorough researcher. 
        - Gather comprehensive information on the given topic
        - Organize findings into clear sections
        - Note key facts, statistics, and important details
        - Be objective and factual`)
      .temperature(0.4)  // Moderate for balanced research
      .maxTokens(1500)
      .build();

    console.log('  ✅ Researcher agent ready');

    // Analyst Agent: Synthesizes and analyzes
    this.agents.analyst = await new AgentBuilder('Analyst', llmConfig)
      .description('Analyzes data and identifies patterns')
      .systemPrompt(`You are an analytical expert.
        - Identify patterns and trends in the data
        - Draw meaningful conclusions
        - Highlight key insights
        - Provide data-driven recommendations
        - Be critical and thorough`)
      .temperature(0.5)
      .maxTokens(1500)
      .build();

    console.log('  ✅ Analyst agent ready');

    // Writer Agent: Creates polished content
    this.agents.writer = await new AgentBuilder('Writer', llmConfig)
      .description('Creates clear, engaging written content')
      .systemPrompt(`You are a professional writer.
        - Create well-structured, engaging content
        - Use clear, concise language
        - Ensure logical flow
        - Format properly with sections and headers
        - Maintain professional tone`)
      .temperature(0.7)  // Higher for creative writing
      .maxTokens(2000)
      .build();

    console.log('  ✅ Writer agent ready\n');

    return this.agents;
  }

  /**
   * Research workflow
   */
  async research(topic) {
    console.log(`📚 Step 1: Research Phase`);
    console.log(`Topic: "${topic}"\n`);

    const researchPrompt = `Research the following topic and provide a structured overview:

Topic: ${topic}

Please include:
- Overview/Introduction
- Key concepts and definitions
- Main features or characteristics
- Current trends or developments
- Important considerations`;

    const research = await this.agents.researcher.execute(researchPrompt);
    
    console.log('Research Results:');
    console.log('-'.repeat(60));
    console.log(research.substring(0, 300) + '...\n');
    
    return research;
  }

  /**
   * Analysis workflow
   */
  async analyze(researchData) {
    console.log(`🔬 Step 2: Analysis Phase\n`);

    const analysisPrompt = `Analyze the following research and provide insights:

${researchData}

Please provide:
- Key insights and patterns
- Strengths and opportunities
- Challenges and limitations
- Strategic recommendations
- Future outlook`;

    const analysis = await this.agents.analyst.execute(analysisPrompt);
    
    console.log('Analysis Results:');
    console.log('-'.repeat(60));
    console.log(analysis.substring(0, 300) + '...\n');
    
    return analysis;
  }

  /**
   * Writing workflow
   */
  async writeReport(research, analysis, topic) {
    console.log(`✍️  Step 3: Writing Phase\n`);

    const writingPrompt = `Create a comprehensive report on ${topic} using the following materials:

RESEARCH:
${research}

ANALYSIS:
${analysis}

Create a well-structured report with:
- Executive Summary
- Detailed Findings
- Analysis and Insights
- Recommendations
- Conclusion

Use clear headings and professional formatting.`;

    const report = await this.agents.writer.execute(writingPrompt);
    
    console.log('Final Report:');
    console.log('='.repeat(60));
    console.log(report);
    console.log('='.repeat(60) + '\n');
    
    return report;
  }

  /**
   * Complete multi-agent pipeline
   */
  async generateReport(topic) {
    console.log('\n🚀 Starting Multi-Agent Research Pipeline\n');
    console.log('='.repeat(70) + '\n');

    const startTime = Date.now();

    try {
      // Phase 1: Research
      const research = await this.research(topic);
      
      // Small delay to respect rate limits
      await new Promise(resolve => setTimeout(resolve, 1000));

      // Phase 2: Analysis
      const analysis = await this.analyze(research);
      
      await new Promise(resolve => setTimeout(resolve, 1000));

      // Phase 3: Write Report
      const report = await this.writeReport(research, analysis, topic);

      const duration = ((Date.now() - startTime) / 1000).toFixed(2);

      console.log(`\n✅ Report generated successfully in ${duration}s\n`);

      return {
        topic,
        research,
        analysis,
        report,
        duration
      };

    } catch (error) {
      console.error('❌ Pipeline failed:', error.message);
      throw error;
    }
  }

  /**
   * Parallel agent execution (for independent tasks)
   */
  async executeParallel(tasks) {
    console.log(`\n⚡ Executing ${tasks.length} tasks in parallel...\n`);

    const results = await Promise.all(
      tasks.map(async (task, idx) => {
        console.log(`  Starting task ${idx + 1}: ${task.title}`);
        
        const agent = this.agents[task.agent];
        const result = await agent.execute(task.prompt);
        
        console.log(`  ✅ Completed task ${idx + 1}`);
        
        return {
          task: task.title,
          result
        };
      })
    );

    console.log(`\n✅ All parallel tasks completed\n`);
    return results;
  }

  /**
   * Agent collaboration pattern - iterative refinement
   */
  async collaborativeRefinement(topic, iterations = 2) {
    console.log(`\n🔄 Collaborative Refinement (${iterations} iterations)\n`);

    let content = `Topic: ${topic}`;

    for (let i = 1; i <= iterations; i++) {
      console.log(`--- Iteration ${i} ---\n`);

      // Research refines the content
      const refined = await this.agents.researcher.execute(
        `Expand and improve this content with more details:\n\n${content}`
      );

      // Analyst reviews and suggests improvements
      const feedback = await this.agents.analyst.execute(
        `Review this content and suggest specific improvements:\n\n${refined}`
      );

      // Writer incorporates feedback
      content = await this.agents.writer.execute(
        `Improve this content based on the feedback:\n\nContent:\n${refined}\n\nFeedback:\n${feedback}`
      );

      console.log(`Iteration ${i} complete. Content length: ${content.length} chars\n`);

      if (i < iterations) {
        await new Promise(resolve => setTimeout(resolve, 1000));
      }
    }

    return content;
  }
}

/**
 * Example usage patterns
 */
async function main() {
  console.log('🎭 Multi-Agent System Example\n');
  console.log('================================\n');

  init();

  const system = new MultiAgentSystem(
    process.env.OPENAI_API_KEY,
    'openai'
  );

  await system.initializeAgents();

  // Example 1: Sequential pipeline
  console.log('\n📖 EXAMPLE 1: Sequential Research Pipeline\n');
  const report = await system.generateReport('GraphRAG technology and applications');

  // Example 2: Parallel execution
  console.log('\n📖 EXAMPLE 2: Parallel Task Execution\n');
  const parallelTasks = [
    {
      title: 'Research AI trends',
      agent: 'researcher',
      prompt: 'Summarize current AI trends in 2024'
    },
    {
      title: 'Analyze market',
      agent: 'analyst',
      prompt: 'Analyze the AI market landscape'
    },
    {
      title: 'Write summary',
      agent: 'writer',
      prompt: 'Write a brief on AI adoption'
    }
  ];

  const parallelResults = await system.executeParallel(parallelTasks);

  parallelResults.forEach(result => {
    console.log(`\nTask: ${result.task}`);
    console.log(`Result: ${result.result.substring(0, 150)}...\n`);
  });

  // Example 3: Iterative refinement
  console.log('\n📖 EXAMPLE 3: Collaborative Refinement\n');
  const refined = await system.collaborativeRefinement(
    'Best practices for AI deployment',
    2
  );

  console.log('Final Refined Content:');
  console.log('-'.repeat(60));
  console.log(refined.substring(0, 500) + '...\n');
}

main().catch(console.error);
```

---

## Walkthrough

### Pattern 1: Sequential Pipeline

Each agent builds on the previous agent's work:

```
Research → Analysis → Writing
```

This is ideal when each step depends on the previous one.

### Pattern 2: Parallel Execution

Multiple agents work on independent tasks simultaneously:

```
┌─ Agent 1: Research  ─┐
├─ Agent 2: Analysis  ─┤ → Combine Results
└─ Agent 3: Writing   ─┘
```

This is faster when tasks are independent.

### Pattern 3: Iterative Refinement

Agents review and improve each other's work:

```
Draft → Review → Improve → Review → Improve → Final
```

This produces higher quality results through collaboration.

---

## Running the Example

```bash
# Set environment variable
export OPENAI_API_KEY=your_key_here

# Run
node multi-agent-example.js
```

### Expected Output

```
🎭 Multi-Agent System Example

================================

🤖 Initializing agents...

  ✅ Researcher agent ready
  ✅ Analyst agent ready
  ✅ Writer agent ready


📖 EXAMPLE 1: Sequential Research Pipeline

🚀 Starting Multi-Agent Research Pipeline

======================================================================

📚 Step 1: Research Phase
Topic: "GraphRAG technology and applications"

Research Results:
------------------------------------------------------------
GraphRAG represents a significant advancement in retrieval-augmented
generation systems, combining graph-based knowledge representation
with large language models...

🔬 Step 2: Analysis Phase

Analysis Results:
------------------------------------------------------------
The integration of graph structures with RAG systems presents several
key advantages. First, it enables multi-hop reasoning...

✍️  Step 3: Writing Phase

Final Report:
============================================================

# GraphRAG Technology: Comprehensive Analysis and Applications

## Executive Summary

GraphRAG (Graph-based Retrieval-Augmented Generation) represents...

[Full report continues...]

============================================================

✅ Report generated successfully in 45.23s
```

---

## Advanced Patterns

### 1. Agent Voting/Consensus

```javascript
async voteOnDecision(question, options) {
  const votes = await Promise.all([
    this.agents.researcher.execute(`Choose: ${question}\nOptions: ${options}`),
    this.agents.analyst.execute(`Choose: ${question}\nOptions: ${options}`),
    this.agents.writer.execute(`Choose: ${question}\nOptions: ${options}`)
  ]);

  // Tally votes and return consensus
  return this.findConsensus(votes);
}
```

### 2. Specialized Task Assignment

```javascript
async routeTask(task) {
  // Determine which agent should handle the task
  if (task.type === 'research') return this.agents.researcher;
  if (task.type === 'analysis') return this.agents.analyst;
  if (task.type === 'writing') return this.agents.writer;
}
```

### 3. Error Recovery with Fallback Agents

```javascript
async executeWithFallback(prompt, primaryAgent, fallbackAgent) {
  try {
    return await this.agents[primaryAgent].execute(prompt);
  } catch (error) {
    console.log(`Primary agent failed, using fallback...`);
    return await this.agents[fallbackAgent].execute(prompt);
  }
}
```

---

## Customization

### Use Different LLMs Per Agent

```javascript
// Fast, cheap model for research
const researchConfig = LlmConfig.openai({
  apiKey,
  model: 'gpt-4o-mini'
});

// Powerful model for analysis
const analysisConfig = LlmConfig.openai({
  apiKey,
  model: 'gpt-4o'
});
```

### Adjust Agent Personalities

```javascript
.systemPrompt(`You are a creative, out-of-the-box thinker.
  Challenge assumptions and propose innovative solutions.`)
.temperature(0.9)  // High creativity
```

---

## Performance Tips

1. **Use Parallel Execution:** When tasks are independent
2. **Rate Limiting:** Add delays between sequential API calls
3. **Result Caching:** Cache intermediate results
4. **Timeout Handling:** Set timeouts for agent executions

---

## Troubleshooting

### Issue: Agents Give Inconsistent Results

**Solution:** Lower temperature for more deterministic outputs

```javascript
.temperature(0.2)  // More consistent
```

### Issue: Pipeline Takes Too Long

**Solution:** Use parallel execution where possible

```javascript
const [research1, research2] = await Promise.all([
  agent1.execute(prompt1),
  agent2.execute(prompt2)
]);
```

### Issue: Rate Limits Hit

**Solution:** Add delays between calls

```javascript
await new Promise(resolve => setTimeout(resolve, 2000));
```

---

## Production Considerations

1. **Logging:** Log all agent interactions
2. **Monitoring:** Track agent performance metrics
3. **Error Handling:** Implement retry logic
4. **Cost Management:** Monitor API usage
5. **Quality Assurance:** Validate agent outputs

---

## Related Examples

- [RAG Pipeline](./rag-pipeline.md) - Knowledge retrieval
- [Error Handling](./error-handling.md) - Production error handling
- [Production Deployment](./production-deployment.md) - Deploy to production

---

**Example Created:** 2025-12-05  
**GraphBit Version:** 0.5.5  
**Difficulty:** Advanced
