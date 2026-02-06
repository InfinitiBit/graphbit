/**
 * Example usage of GraphBit Memory Layer in JavaScript/Node.js.
 *
 * This example demonstrates:
 * - Creating memory configurations
 * - Adding memories from conversations
 * - Searching memories semantically
 * - CRUD operations on memories
 * - Viewing memory history
 *
 * Requirements:
 *   - Set OPENAI_API_KEY environment variable
 *   - npm install @graphbit/memory (or build from source)
 */

const { MemoryConfig, MemoryClient } = require("@graphbit/memory");

async function main() {
  // Get API key from environment
  const apiKey = process.env.OPENAI_API_KEY;
  if (!apiKey) {
    console.error("Error: OPENAI_API_KEY environment variable not set");
    console.error("Please set it with: export OPENAI_API_KEY='your-key-here'");
    process.exit(1);
  }

  console.log("=== GraphBit Memory Layer Example ===\n");

  // Create memory config
  // Use ":memory:" for in-memory database or a file path for persistence
  const config = new MemoryConfig(
    "openai", // LLM provider
    apiKey, // LLM API key
    "gpt-4o-mini", // LLM model
    apiKey, // Embedding API key
    "text-embedding-3-small", // Embedding model
    ":memory:", // Database path
    0.7 // Similarity threshold
  );

  // Create memory client
  const client = await MemoryClient.create(config);
  console.log("Memory client initialized\n");

  // Add memories from a conversation
  console.log("--- Adding memories from conversation ---");
  const conversation = [
    { role: "user", content: "Hi! I'm Alex and I live in Munich, Germany." },
    {
      role: "assistant",
      content: "Nice to meet you, Alex! Munich is a beautiful city.",
    },
    { role: "user", content: "I work as a software engineer at a startup." },
    {
      role: "assistant",
      content: "That sounds exciting! What technologies do you work with?",
    },
    {
      role: "user",
      content: "Mostly Python and Rust. I also love hiking on weekends.",
    },
    {
      role: "assistant",
      content: "Great choices! The Bavarian Alps are perfect for hiking.",
    },
  ];

  const scope = { userId: "alex_123" };
  const memories = await client.add(conversation, scope);
  console.log(`Extracted and stored ${memories.length} memories:`);
  for (const mem of memories) {
    console.log(`  - ${mem.content}`);
  }
  console.log();

  // Search for memories semantically
  console.log("--- Searching memories ---");
  const queries = [
    "Where does the user live?",
    "What is their profession?",
    "What are their hobbies?",
  ];

  for (const query of queries) {
    const results = await client.search(query, scope, 2);
    console.log(`Query: '${query}'`);
    if (results.length > 0) {
      for (const r of results) {
        console.log(`  [${r.score.toFixed(2)}] ${r.memory.content}`);
      }
    } else {
      console.log("  No matching memories found");
    }
    console.log();
  }

  // Get all memories for a user
  console.log("--- All memories for user ---");
  const allMemories = await client.getAll(scope);
  console.log(`Total memories: ${allMemories.length}`);
  for (const mem of allMemories) {
    console.log(`  ID: ${mem.id.substring(0, 8)}... | ${mem.content}`);
  }
  console.log();

  // Update a memory
  if (memories.length > 0) {
    console.log("--- Updating a memory ---");
    const firstId = memories[0].id;
    const updated = await client.update(
      firstId,
      "User Alex lives in Munich, Germany (Bavaria)"
    );
    console.log(`Updated memory: ${updated.content}\n`);

    // View history for the updated memory
    console.log("--- Memory history ---");
    const history = await client.history(firstId);
    for (const entry of history) {
      console.log(
        `  [${entry.action}] '${entry.oldContent}' -> '${entry.newContent}'`
      );
    }
    console.log();
  }

  // Delete a specific memory
  if (memories.length > 1) {
    console.log("--- Deleting a memory ---");
    const toDelete = memories[1].id;
    await client.delete(toDelete);
    console.log(`Deleted memory ID: ${toDelete.substring(0, 8)}...`);

    const remaining = await client.getAll(scope);
    console.log(`Remaining memories: ${remaining.length}\n`);
  }

  // Demonstrate scope isolation
  console.log("--- Scope isolation ---");
  const otherConversation = [
    { role: "user", content: "My name is Jordan and I'm a data scientist." },
    {
      role: "assistant",
      content: "Hello Jordan! Data science is a fascinating field.",
    },
  ];
  await client.add(otherConversation, { userId: "jordan_456" });

  const alexCount = (await client.getAll({ userId: "alex_123" })).length;
  const jordanCount = (await client.getAll({ userId: "jordan_456" })).length;
  const totalCount = (await client.getAll()).length; // No scope filter

  console.log(`Alex's memories: ${alexCount}`);
  console.log(`Jordan's memories: ${jordanCount}`);
  console.log(`Total memories: ${totalCount}`);
  console.log();

  // Clean up - delete all memories for a user
  console.log("--- Cleanup ---");
  await client.deleteAll({ userId: "alex_123" });
  await client.deleteAll({ userId: "jordan_456" });
  console.log("All memories deleted");

  console.log("\n=== Example complete ===");
}

main().catch(console.error);
