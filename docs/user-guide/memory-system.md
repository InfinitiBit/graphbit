# Memory System for Stateful AI Agents

The GraphBit memory system enables AI agents to remember, learn, and evolve across interactions. Unlike stateless agents that forget everything after each session, agents with memory can maintain context, recall important information, and behave more intelligently over time.

## Overview

The memory system implements four types of memory, mimicking human information storage:

1. **Working Memory**: Short-term session-based context for current conversations
2. **Factual Memory**: Long-term structured knowledge (facts, preferences, settings)
3. **Episodic Memory**: Records of specific past conversations and interactions
4. **Semantic Memory**: General knowledge built from patterns and insights over time

## Key Features

- **LLM-based Extraction**: Intelligently decides what information to remember
- **Vector Search**: Semantic retrieval using embeddings for relevant memory recall
- **Memory Decay**: Time and importance-based filtering to prevent memory bloat
- **Graph Connections**: Link related memories across sessions
- **Sub-50ms Retrieval**: Fast in-memory lookups with optional persistence
- **Multimodal Support**: Store text and images in memory
- **Observability**: Statistics, monitoring, and memory graph visualization

## Quick Start

### Basic Usage

```rust
use graphbit_core::memory::{MemoryManager, MemoryConfig, MemoryQuery};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a memory manager with default configuration
    let mut manager = MemoryManager::with_defaults();

    // Start a session
    manager.start_session("session_123".to_string());

    // Store working memory (temporary, session-based)
    manager.store_working("User prefers dark mode".to_string()).await?;

    // Store a fact (persistent, long-term)
    manager.store_fact("theme".to_string(), "dark".to_string()).await?;

    // Retrieve memories
    let query = MemoryQuery::new("user preferences".to_string());
    let results = manager.retrieve(query).await?;

    for result in results {
        println!("Memory: {} (score: {})", result.memory.content, result.score);
    }

    // Get statistics
    let stats = manager.get_stats().await;
    println!("Total memories: {}", stats.total_memories);

    Ok(())
}
```

### With Embeddings for Semantic Search

```rust
use graphbit_core::memory::MemoryManager;
use graphbit_core::embeddings::{EmbeddingConfig, EmbeddingProvider, EmbeddingService};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure embeddings
    let embedding_config = EmbeddingConfig {
        provider: EmbeddingProvider::OpenAI,
        api_key: std::env::var("OPENAI_API_KEY")?,
        model: "text-embedding-3-small".to_string(),
        base_url: None,
        timeout_seconds: Some(30),
        max_batch_size: Some(100),
        extra_params: Default::default(),
    };

    let embedding_service = Arc::new(EmbeddingService::new(embedding_config)?);

    // Create memory manager with embeddings
    let mut manager = MemoryManager::with_embeddings(embedding_service);

    // Now semantic search will work automatically
    manager.start_session("session_456".to_string());
    manager.store_working("User loves Python programming".to_string()).await?;

    // Semantic search finds related memories even with different wording
    let query = MemoryQuery::new("coding preferences".to_string());
    let results = manager.retrieve(query).await?;

    Ok(())
}
```

## Memory Types in Detail

### Working Memory

Short-term memory for the current session. Automatically cleared when the session ends.

```rust
// Start a session
manager.start_session("session_789".to_string());

// Store working memory
manager.store_working("User is asking about GraphBit features".to_string()).await?;
manager.store_working("User mentioned they use Rust".to_string()).await?;

// Get formatted context for LLM injection
let context = manager.get_working_context().await;
println!("Session context:\n{}", context);

// End session (clears working memory)
let cleared = manager.end_session().await?;
println!("Cleared {} working memories", cleared);
```

### Factual Memory

Long-term storage for facts, preferences, and settings.

```rust
// Store facts
manager.store_fact("name".to_string(), "Alice".to_string()).await?;
manager.store_fact("language".to_string(), "Rust".to_string()).await?;
manager.store_fact("timezone".to_string(), "UTC-8".to_string()).await?;

// Retrieve a fact
if let Some(name) = manager.get_fact("name").await {
    println!("User's name: {}", name);
}

// Update a fact
manager.update_fact("timezone", "UTC-7".to_string()).await?;
```

### Episodic Memory

Records of specific conversations and interactions.

```rust
// Start recording an episode
manager.start_episode("Discussion about memory systems".to_string());

// Add content to the episode
manager.add_to_episode("User asked how memory works".to_string());
manager.add_to_episode("Explained the four memory types".to_string());
manager.add_to_episode("User understood the concept".to_string());

// End the episode
let episode_id = manager.end_episode().await?;

// Retrieve recent episodes
let recent = manager.get_recent_episodes(5).await;
for episode in recent {
    println!("Episode: {}", episode.content);
}
```

### Semantic Memory

General knowledge and patterns learned over time.

```rust
use graphbit_core::memory::SemanticConcept;

// Store a semantic concept
let concept = SemanticConcept::new(
    "Rust is preferred for performance".to_string(),
    0.8, // confidence
);

manager.store_concept(concept).await?;

// Reinforce a concept (increases confidence)
manager.reinforce_concept("Rust is preferred for performance").await?;
```

## LLM-Based Memory Extraction

The memory system can use an LLM to intelligently decide what to remember:

```rust
use graphbit_core::memory::extraction::{MemoryExtractor, ExtractionConfig};
use graphbit_core::llm::{LlmProvider, LlmConfig};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure LLM
    let llm_config = LlmConfig::openai("gpt-4".to_string());
    let llm_provider = Arc::new(LlmProvider::new(llm_config)?);

    // Create extractor
    let extraction_config = ExtractionConfig::new();
    let mut extractor = MemoryExtractor::new(llm_provider, extraction_config);

    // Extract memories from a message
    let message = "I prefer dark mode and I'm working on a Rust project";
    let result = extractor.extract(message, None).await?;

    if result.should_remember {
        println!("Should remember: {}", result.content.unwrap());
        println!("Type: {:?}", result.memory_type.unwrap());
        println!("Importance: {}", result.importance);
        println!("Tags: {:?}", result.tags);
    }

    Ok(())
}
```

## Configuration

### Memory Configuration

```rust
use graphbit_core::memory::{MemoryConfig, MemoryType};
use graphbit_core::memory::decay::DecayConfig;

let mut config = MemoryConfig::new();

// Enable/disable memory types
config.enable_working = true;
config.enable_factual = true;
config.enable_episodic = true;
config.enable_semantic = true;

// Set capacities
config.set_capacity(MemoryType::Working, 100);
config.set_capacity(MemoryType::Factual, 1000);
config.set_capacity(MemoryType::Episodic, 500);
config.set_capacity(MemoryType::Semantic, 200);

// Configure decay
config.decay_config = DecayConfig::conservative();

// Enable automatic embeddings
config.auto_embed = true;

let manager = MemoryManager::new(config, None);
```

### Decay Configuration

```rust
use graphbit_core::memory::decay::DecayConfig;

// Conservative decay (keeps more memories)
let conservative = DecayConfig::conservative();

// Aggressive decay (removes more memories)
let aggressive = DecayConfig::aggressive();

// Custom decay
let mut custom = DecayConfig::new();
custom.importance_threshold = 0.3;
custom.age_threshold_days = 60;
custom.access_threshold_days = 14;
custom.decay_interval_hours = 24;
```

## Memory Retrieval

### Basic Query

```rust
use graphbit_core::memory::MemoryQuery;

let query = MemoryQuery::new("user preferences".to_string())
    .with_limit(10);

let results = manager.retrieve(query).await?;
```

### Filtered Query

```rust
use graphbit_core::memory::{MemoryQuery, MemoryType};

let query = MemoryQuery::new("programming".to_string())
    .with_memory_type(MemoryType::Factual)
    .with_session_id("session_123".to_string())
    .with_tags(vec!["language".to_string()])
    .with_limit(5);

let results = manager.retrieve(query).await?;
```

## Best Practices

1. **Use Working Memory for Session Context**: Store temporary conversation context in working memory
2. **Store Important Facts**: Use factual memory for user preferences, settings, and long-term information
3. **Record Significant Interactions**: Create episodes for important conversations
4. **Build Semantic Knowledge**: Extract patterns and insights into semantic memory
5. **Run Decay Regularly**: Prevent memory bloat by running decay periodically
6. **Use Embeddings for Better Retrieval**: Enable semantic search with embedding support
7. **Monitor Memory Usage**: Check statistics regularly to understand memory patterns

## Integration with Workflows

```rust
use graphbit_core::workflow::{Workflow, WorkflowBuilder};
use graphbit_core::memory::MemoryManager;

// Create workflow with memory
let mut manager = MemoryManager::with_defaults();
manager.start_session("workflow_session".to_string());

// Store context before workflow execution
manager.store_working("Starting workflow execution".to_string()).await?;

// Execute workflow
let workflow = WorkflowBuilder::new("my_workflow")
    .build()?;

// Store results after execution
manager.store_fact("last_workflow".to_string(), "my_workflow".to_string()).await?;
```

## Performance Considerations

- **In-Memory Storage**: Default storage is in-memory for sub-50ms retrieval
- **Capacity Limits**: Set appropriate capacity limits to prevent unbounded growth
- **Decay Intervals**: Run decay periodically (e.g., every 24 hours)
- **Embedding Batching**: Use batch embedding generation for better performance
- **Query Limits**: Set reasonable limits on query results

## Next Steps

- Explore the [API Reference](../api/memory.md) for detailed documentation
- See [Examples](../../examples/memory/) for more usage patterns
- Learn about [Python Bindings](./python-bindings.md) for using memory from Python

