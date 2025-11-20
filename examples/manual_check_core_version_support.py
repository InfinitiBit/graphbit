import os

def check_system_health():
    from graphbit import init, version, get_system_info, health_check
    init(debug=True, log_level="info")
    """Check GraphBit system health and capabilities."""
    print("\n\n\n🚀 System Health Check")
    print(f"GraphBit version: {version()}")
    print("=" * 50, end="\n")
    print(f"System info: {get_system_info()}")
    print("=" * 50, end="\n")
    print(f"Health check: {health_check()}")
    print("=" * 50, end="\n")

# check_system_health()

def check_configuration():
    import graphbit
    
    graphbit.shutdown()
    graphbit.configure_runtime(
        worker_threads=8,
        max_blocking_threads=16,
        thread_stack_size_mb=2,
    )
    graphbit.init()

    info = graphbit.get_system_info()
    print(f"Runtime initialized: {info['runtime_initialized']}")
    print(f"Worker threads: {info['runtime_worker_threads']}")

# check_configuration()

def check_llm_client():
    """Demonstrate LLM client functionality."""
    from graphbit import LlmConfig, LlmClient

    print("\n\n\n🤖 LLM Client Integration")
    llm_config = LlmConfig.openai(api_key=os.getenv("OPENAI_API_KEY"), model="gpt-4o-mini")
    llm_client = LlmClient(llm_config)
    response = llm_client.complete("Hello, world!", max_tokens=50)
    print(f"Response: {response}")
    print("=" * 50, end="\n")

# check_llm_client()

def check_embeddings():
    """Demonstrate embeddings functionality."""
    from graphbit import EmbeddingConfig, EmbeddingClient

    print("\n\n\n🔢 Embeddings Functionality")
    embedding_config = EmbeddingConfig.openai(api_key=os.getenv("OPENAI_API_KEY"), model="text-embedding-3-small")
    embedding_client = EmbeddingClient(embedding_config)
    embedding = embedding_client.embed("Hello, world!")
    print(f"Embedding: {len(embedding)} dimensions")
    print("=" * 50, end="\n")
    embeddings = embedding_client.embed_many(["Hello, world!", "Goodbye, world!"])
    print(f"Embeddings: {len(embeddings)} texts processed")
    print("=" * 50, end="\n")

    print(f"Similarity: {embedding_client.similarity(embeddings[0], embeddings[1])}")
    print("=" * 50, end="\n")

# check_embeddings()

def check_document_loading():
    """Demonstrate document loading functionality."""
    from graphbit import DocumentLoader

    print("\n\n\n📄 Document Loading")
    print("=" * 50)
    import tempfile
    import json
    from pathlib import Path

    # Create temporary documents for testing
    with tempfile.TemporaryDirectory() as temp_dir:
        temp_path = Path(temp_dir)

        # Create sample documents
        txt_file = temp_path / "sample.txt"
        txt_file.write_text("This is a sample text document for GraphBit demonstration.\n"
                           "It contains multiple lines of text that will be processed.\n"
                           "GraphBit can handle various document formats efficiently.")

        json_file = temp_path / "sample.json"
        json_file.write_text(json.dumps({
            "title": "GraphBit Demo",
            "description": "Comprehensive feature demonstration",
            "features": ["LLM Integration", "Embeddings", "Workflows", "Document Processing"]
        }, indent=2))

        csv_file = temp_path / "sample.csv"
        csv_file.write_text("name,role,experience\n"
                           "Alice,Developer,5\n"
                           "Bob,Designer,3\n"
                           "Charlie,Manager,8\n")

        # Demonstrate document loading
        try:
            loader = DocumentLoader()

            # Show supported types
            supported = DocumentLoader.supported_types()
            print(f"✅ Supported document types: {', '.join(supported)}")
            print("=" * 50, end="\n")

            # Load different document types
            for file_path in [txt_file, json_file, csv_file]:
                doc_type = DocumentLoader.detect_document_type(str(file_path))
                if doc_type:
                    content = loader.load_document(str(file_path), doc_type)
                    print(f"✅ Loaded {file_path.name} ({doc_type}): {content.content_length()} characters")
                    print(f"\n   Preview: {content.content[:100]}...")
                    print("=" * 50, end="\n")
                else:
                    print(f"❌ Could not detect type for {file_path.name}")

        except Exception as e:
            print(f"❌ Document loading error: {e}")

# check_document_loading()

def check_text_splitter():
    """Demonstrate text splitter functionality."""
    from graphbit import CharacterSplitter, RecursiveSplitter, SentenceSplitter, TokenSplitter

    print("\n\n\n✂️ Text Splitting")
    print("=" * 50)
    sample_text = "This is a sample text document for GraphBit demonstration. " * 10
    print(f"Sample text: {sample_text[:100]}...")
    print("=" * 50, end="\n")
    try:
        # Character splitter
        char_splitter = CharacterSplitter(chunk_size=200, chunk_overlap=50)
        char_chunks = char_splitter.split_text(sample_text)
        print(f"✅ Character splitter: {len(char_chunks)} chunks")
        print(f"   Preview: {char_chunks[0].content[:200]}...")
        print("=" * 50, end="\n")

        # Recursive splitter
        recursive_splitter = RecursiveSplitter(chunk_size=300, chunk_overlap=50)
        recursive_chunks = recursive_splitter.split_text(sample_text)
        print(f"✅ Recursive splitter: {len(recursive_chunks)} chunks")
        print(f"   Preview: {recursive_chunks[0].content[:200]}...")
        print("=" * 50, end="\n")

        # Sentence splitter
        sentence_splitter = SentenceSplitter(chunk_size=400, chunk_overlap=1)
        sentence_chunks = sentence_splitter.split_text(sample_text)
        print(f"✅ Sentence splitter: {len(sentence_chunks)} chunks")
        print(f"   Preview: {sentence_chunks[0].content[:200]}...")
        print("=" * 50, end="\n")

        # Token splitter
        token_splitter = TokenSplitter(chunk_size=50, chunk_overlap=10)
        token_chunks = token_splitter.split_text(sample_text)
        print(f"✅ Token splitter: {len(token_chunks)} chunks")
        print(f"   Preview: {token_chunks[0].content[:200]}...")
        print("=" * 50, end="\n")

    except Exception as e:
        print(f"❌ Text splitting error: {e}")
        print("=" * 50, end="\n")

# check_text_splitter()

def check_simple_workflow():
    """Demonstrate simple workflow functionality."""
    from graphbit import Workflow, Node, Executor, LlmConfig

    print("\n\n\n🔗 Simple Workflow")
    print("=" * 50)
    try:
        # Create workflow
        workflow = Workflow("Simple Workflow")
        node = Node.agent(name="Greet User", prompt="Say hello to the user")
        workflow.add_node(node)
        workflow.validate()

        # Execute workflow
        executor = Executor(LlmConfig.openai(api_key=os.getenv("OPENAI_API_KEY"), model="gpt-4o-mini"))
        result = executor.execute(workflow)
        print(f"Workflow result: {result.get_node_output('Greet User')}")
        print("=" * 50, end="\n")

    except Exception as e:
        print(f"❌ Workflow error: {e}")
        print("=" * 50, end="\n")

# check_simple_workflow()

def check_complex_workflow():
    """Demonstrate complex workflow functionality."""
    from graphbit import Workflow, Node, Executor, LlmConfig
    
    print("\n\n\n🔗 Complex Workflow")
    print("=" * 50)
    try:
        # Create workflow
        workflow = Workflow("Complex Workflow")
        node1 = Node.agent(name="Start", prompt="Generate a Beautiful topic from nature to write a poem on", agent_id="start_001")
        node2 = Node.agent(name="Middle", prompt="Gather information about the topic", agent_id="middle_001")
        node3 = Node.agent(name="End", prompt="write the poem", agent_id="end_001")
        node_id1 = workflow.add_node(node1)
        node_id2 = workflow.add_node(node2)
        node_id3 = workflow.add_node(node3)
        workflow.connect(node_id1, node_id2)
        workflow.connect(node_id2, node_id3)
        workflow.validate()

        # Execute workflow
        executor = Executor(LlmConfig.openai(api_key=os.getenv("OPENAI_API_KEY"), model="gpt-4o-mini"))
        result = executor.execute(workflow)
        print(f"Workflow result: {result.get_node_output('End')}")
        print("=" * 50, end="\n")
        print(f"Workflow results: {result.get_all_node_outputs()}")
        print("=" * 50, end="\n")

    except Exception as e:
        print(f"❌ Workflow error: {e}")
        print("=" * 50, end="\n")

# check_complex_workflow()

def check_tool_calling_workflow():
    """Demonstrate tool calling workflow functionality."""
    from graphbit import Workflow, Node, Executor, LlmConfig, tool

    print("\n\n\n🔗 Tool Calling Workflow")
    print("=" * 50)
    try:
        # Create tools
        @tool(_description="Get current weather information for any city")
        def get_weather(location: str) -> dict:
            return {"location": location, "temperature": 22, "condition": "sunny"}

        @tool(_description="Perform mathematical calculations and return results")
        def calculate(expression: str) -> str:
            return f"Result: {eval(expression)}"
        
        # Create workflow
        workflow = Workflow("Tool Calling Workflow")
        node = Node.agent(name="Greet User", prompt="Say hello to the user and provide the output of the tool calling. What is the weather in San Francisco? and calculate 15 + 27?", tools=[get_weather, calculate])
        workflow.add_node(node)
        workflow.validate()

        # Execute workflow
        executor = Executor(LlmConfig.openai(api_key=os.getenv("OPENAI_API_KEY"), model="gpt-4o-mini"))
        result = executor.execute(workflow)
        print(f"Workflow result: {result.get_node_output('Greet User')}")
        print("=" * 50, end="\n")

    except Exception as e:
        print(f"❌ Workflow error: {e}")
        print("=" * 50, end="\n")

# check_tool_calling_workflow()