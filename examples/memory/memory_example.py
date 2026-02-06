"""Example usage of GraphBit Memory Layer in Python.

This example demonstrates:
- Creating memory configurations
- Adding memories from conversations
- Searching memories semantically
- CRUD operations on memories
- Viewing memory history

Requirements:
    - Set OPENAI_API_KEY environment variable or create .env file
    - pip install graphbit python-dotenv (or maturin develop from source)
"""

import os
from pathlib import Path

# Load .env file from project root if it exists
try:
    from dotenv import load_dotenv

    # Look for .env in current dir, parent dirs, or project root
    env_path = Path(__file__).resolve().parent.parent.parent / ".env"
    if env_path.exists():
        load_dotenv(env_path)
    else:
        load_dotenv()  # Try default locations
except ImportError:
    pass  # python-dotenv not installed, rely on environment

import graphbit


def main():
    # Get API key from environment
    api_key = os.getenv("OPENAI_API_KEY")
    if not api_key:
        print("Error: OPENAI_API_KEY environment variable not set")
        print("Please set it with: export OPENAI_API_KEY='your-key-here'")
        return

    print("=== GraphBit Memory Layer Example ===\n")

    # Create LLM config for fact extraction
    llm_config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4o-mini")

    # Create embedding config for semantic search
    embedding_config = graphbit.EmbeddingConfig.openai(
        api_key=api_key, model="text-embedding-3-small"
    )

    # Create memory config with in-memory database for this example
    # Use a file path like "memories.db" for persistence
    memory_config = graphbit.MemoryConfig(
        llm_config,
        embedding_config,
        db_path=":memory:",  # Use ":memory:" for in-memory or a file path
        similarity_threshold=0.1,  # Reasonable threshold for semantic search
    )

    # Create memory client
    client = graphbit.MemoryClient(memory_config)
    print("Memory client initialized\n")

    # Add memories from a conversation
    print("--- Adding memories from conversation ---")
    conversation = [
        ("user", "Hi! I'm Alex and I live in Munich, Germany."),
        ("assistant", "Nice to meet you, Alex! Munich is a beautiful city."),
        ("user", "I work as a software engineer at a startup."),
        ("assistant", "That sounds exciting! What technologies do you work with?"),
        ("user", "Mostly Python and Rust. I also love hiking on weekends."),
        ("assistant", "Great choices! The Bavarian Alps are perfect for hiking."),
    ]

    memories = client.add(conversation, user_id="alex_123")
    print(f"Extracted and stored {len(memories)} memories:")
    for mem in memories:
        print(f"  - {mem.content}")
    print()

    # Search for memories semantically
    print("--- Searching memories ---")
    queries = [
        "Where does the user live?",
        "What is their profession?",
        "What are their hobbies?",
    ]

    for query in queries:
        results = client.search(query, user_id="alex_123", top_k=2)
        print(f"Query: '{query}'")
        if results:
            for r in results:
                print(f"  [{r.score:.2f}] {r.memory.content}")
        else:
            print("  No matching memories found")
        print()

    # Get all memories for a user
    print("--- All memories for user ---")
    all_memories = client.get_all(user_id="alex_123")
    print(f"Total memories: {len(all_memories)}")
    for mem in all_memories:
        print(f"  ID: {mem.id[:8]}... | {mem.content}")
    print()

    # Update a memory
    if memories:
        print("--- Updating a memory ---")
        first_id = memories[0].id
        updated = client.update(
            first_id, "User Alex lives in Munich, Germany (Bavaria)"
        )
        print(f"Updated memory: {updated.content}\n")

        # View history for the updated memory
        print("--- Memory history ---")
        history = client.history(first_id)
        for entry in history:
            print(f"  [{entry.action}] '{entry.old_content}' -> '{entry.new_content}'")
        print()

    # Delete a specific memory
    if len(memories) > 1:
        print("--- Deleting a memory ---")
        to_delete = memories[1].id
        client.delete(to_delete)
        print(f"Deleted memory ID: {to_delete[:8]}...")

        remaining = client.get_all(user_id="alex_123")
        print(f"Remaining memories: {len(remaining)}\n")

    # Demonstrate scope isolation
    print("--- Scope isolation ---")
    other_conversation = [
        ("user", "My name is Jordan and I'm a data scientist."),
        ("assistant", "Hello Jordan! Data science is a fascinating field."),
    ]
    client.add(other_conversation, user_id="jordan_456")

    alex_count = len(client.get_all(user_id="alex_123"))
    jordan_count = len(client.get_all(user_id="jordan_456"))
    total_count = len(client.get_all())  # No scope filter

    print(f"Alex's memories: {alex_count}")
    print(f"Jordan's memories: {jordan_count}")
    print(f"Total memories: {total_count}")
    print()

    # Clean up - delete all memories for a user
    print("--- Cleanup ---")
    client.delete_all(user_id="alex_123")
    client.delete_all(user_id="jordan_456")
    print("All memories deleted")

    print("\n=== Example complete ===")


if __name__ == "__main__":
    main()
