import os
import sys
import json
import tempfile
from pathlib import Path

from dotenv import load_dotenv

load_dotenv()

import graphbit
from graphbit import (
    get_system_info,
    health_check,
    init,
    version,
    LlmClient,
    EmbeddingClient,
    EmbeddingConfig,
    DocumentLoader,
    CharacterSplitter,
    RecursiveSplitter,
    SentenceSplitter,
    TokenSplitter,
    Executor,
    LlmConfig,
    Node,
    Workflow,
    tool,
)


class GraphBitFunctionalityChecker:
    def __init__(self) -> None:
        self.api_key = os.getenv("OPENAI_API_KEY")

    # ---------- Helpers ----------

    @staticmethod
    def _print_header(title: str) -> None:
        print("\n\n\n" + title)
        print("=" * 50)

    def _require_api_key(self, feature_name: str) -> bool:
        if not self.api_key:
            print("OPENAI_API_KEY not found in environment variables")
            print(f"   Skipping {feature_name} test")
            return False
        return True

    # ---------- Checks ----------

    def check_system_health(self) -> bool:
        """Check GraphBit system health and capabilities."""
        self._print_header("System Health Check")

        # Initialize GraphBit
        try:
            init(debug=True, log_level="info")
            print("GraphBit initialization successful")
        except Exception as e:
            print(f"GraphBit initialization failed: {e}")
            return False

        # Version information
        try:
            version_info = version()
            print(f"GraphBit version: {version_info}")
        except Exception as e:
            print(f"Failed to get version information: {e}")
            return False

        print("=" * 50)

        # System information
        try:
            system_info = get_system_info()
            print(f"System info: {system_info}")
        except Exception as e:
            print(f"Failed to get system information: {e}")
            return False

        print("=" * 50)

        # Health check
        try:
            health_result = health_check()
            print(f"Health check: {health_result}")
        except Exception as e:
            print(f"Health check failed: {e}")
            return False

        print("=" * 50)
        return True

    def check_configuration(self) -> bool:
        """Test GraphBit runtime configuration."""
        self._print_header("Runtime Configuration Check")

        # Shutdown existing runtime (warning only on failure)
        try:
            graphbit.shutdown()
            print("Runtime shutdown successful")
        except Exception as e:
            print(f"Runtime shutdown warning: {e}")

        # Configure runtime with custom settings
        try:
            graphbit.configure_runtime(
                worker_threads=8,
                max_blocking_threads=16,
                thread_stack_size_mb=2,
            )
            print("Runtime configuration successful")
        except Exception as e:
            print(f"Runtime configuration failed: {e}")
            return False

        # Re-initialize with new configuration
        try:
            graphbit.init()
            print("Runtime re-initialization successful")
        except Exception as e:
            print(f"Runtime re-initialization failed: {e}")
            return False

        # Verify configuration
        try:
            info = graphbit.get_system_info()
            print(f"Runtime initialized: {info.get('runtime_initialized', 'Unknown')}")
            print(f"Worker threads: {info.get('runtime_worker_threads', 'Unknown')}")
        except Exception as e:
            print(f"Failed to verify configuration: {e}")
            return False

        print("=" * 50)
        return True

    def check_llm_client(self) -> bool:
        """Demonstrate LLM client functionality."""
        self._print_header("LLM Client Integration")

        if not self._require_api_key("LLM client"):
            return True  # Skipped, not a failure

        # Create LLM configuration
        try:
            llm_config = LlmConfig.openai(api_key=self.api_key, model="gpt-4o-mini")
            print("LLM configuration created successfully")
        except Exception as e:
            print(f"Failed to create LLM configuration: {e}")
            return False

        # Create LLM client
        try:
            llm_client = LlmClient(llm_config)
            print("LLM client created successfully")
        except Exception as e:
            print(f"Failed to create LLM client: {e}")
            return False

        # Test LLM completion
        try:
            response = llm_client.complete("Hello, world!", max_tokens=50)
            print(f"LLM response: {response}")
        except Exception as e:
            print(f"LLM completion failed: {e}")
            print("   This might be due to network issues, API key problems, or rate limits")
            return False

        print("=" * 50)
        return True

    def check_embeddings(self) -> bool:
        """Demonstrate embeddings functionality."""
        self._print_header("Embeddings Functionality")

        if not self._require_api_key("embeddings"):
            return True  # Skipped, not a failure

        # Create embedding configuration
        try:
            embedding_config = EmbeddingConfig.openai(
                api_key=self.api_key,
                model="text-embedding-3-small",
            )
            print("Embedding configuration created successfully")
        except Exception as e:
            print(f"Failed to create embedding configuration: {e}")
            return False

        # Create embedding client
        try:
            embedding_client = EmbeddingClient(embedding_config)
            print("Embedding client created successfully")
        except Exception as e:
            print(f"Failed to create embedding client: {e}")
            return False

        # Test single embedding
        try:
            embedding = embedding_client.embed("Hello, world!")
            print(f"Single embedding: {len(embedding)} dimensions")
        except Exception as e:
            print(f"Single embedding failed: {e}")
            return False

        print("=" * 50)

        # Test batch embeddings
        try:
            embeddings = embedding_client.embed_many(["Hello, world!", "Goodbye, world!"])
            print(f"Batch embeddings: {len(embeddings)} texts processed")
        except Exception as e:
            print(f"Batch embeddings failed: {e}")
            return False

        print("=" * 50)

        # Test similarity calculation
        try:
            similarity = embedding_client.similarity(embeddings[0], embeddings[1])
            print(f"Similarity calculation: {similarity}")
        except Exception as e:
            print(f"Similarity calculation failed: {e}")
            return False

        print("=" * 50)
        return True

    def check_document_loading(self) -> bool:
        """Demonstrate document loading functionality."""
        self._print_header("Document Loading")

        # Create document loader
        try:
            loader = DocumentLoader()
            print("Document loader created successfully")
        except Exception as e:
            print(f"Failed to create document loader: {e}")
            return False

        # Show supported types
        try:
            supported = DocumentLoader.supported_types()
            print(f"Supported document types: {', '.join(supported)}")
        except Exception as e:
            print(f"Failed to get supported types: {e}")
            return False

        print("=" * 50)

        # Create temporary documents for testing
        try:
            with tempfile.TemporaryDirectory() as temp_dir:
                temp_path = Path(temp_dir)

                # Create sample documents
                txt_file = temp_path / "sample.txt"
                txt_file.write_text(
                    "This is a sample text document for GraphBit demonstration.\n"
                    "It contains multiple lines of text that will be processed.\n"
                    "GraphBit can handle various document formats efficiently."
                )

                json_file = temp_path / "sample.json"
                json_file.write_text(
                    json.dumps(
                        {
                            "title": "GraphBit Demo",
                            "description": "Comprehensive feature demonstration",
                            "features": [
                                "LLM Integration",
                                "Embeddings",
                                "Workflows",
                                "Document Processing",
                            ],
                        },
                        indent=2,
                    )
                )

                csv_file = temp_path / "sample.csv"
                csv_file.write_text(
                    "name,role,experience\n"
                    "Alice,Developer,5\n"
                    "Bob,Designer,3\n"
                    "Charlie,Manager,8\n"
                )

                print("Sample documents created successfully")

                # Load different document types
                success_count = 0
                total_files = 3

                for file_path in [txt_file, json_file, csv_file]:
                    try:
                        doc_type = DocumentLoader.detect_document_type(str(file_path))
                        if doc_type:
                            content = loader.load_document(str(file_path), doc_type)
                            print(
                                f"Loaded {file_path.name} ({doc_type}): "
                                f"{content.content_length()} characters"
                            )
                            print(f"   Preview: {content.content[:100]}...")
                            success_count += 1
                        else:
                            print(f"Could not detect type for {file_path.name}")
                    except Exception as e:
                        print(f"Failed to load {file_path.name}: {e}")

                print("=" * 50)
                print(f"Successfully loaded {success_count}/{total_files} documents")

                return success_count == total_files

        except Exception as e:
            print(f"Failed to create temporary directory or sample docs: {e}")
            return False

    def check_text_splitter(self) -> bool:
        """Demonstrate text splitter functionality."""
        self._print_header("Text Splitting")

        sample_text = "This is a sample text document for GraphBit demonstration. " * 10
        print(f"Sample text: {sample_text[:100]}...")
        print("=" * 50)

        splitter_tests = [
            ("Character", CharacterSplitter, {"chunk_size": 200, "chunk_overlap": 50}),
            ("Recursive", RecursiveSplitter, {"chunk_size": 300, "chunk_overlap": 50}),
            ("Sentence", SentenceSplitter, {"chunk_size": 400, "chunk_overlap": 1}),
            ("Token", TokenSplitter, {"chunk_size": 50, "chunk_overlap": 10}),
        ]

        success_count = 0

        for splitter_name, splitter_class, kwargs in splitter_tests:
            try:
                splitter = splitter_class(**kwargs)
                print(f"{splitter_name} splitter created successfully")

                chunks = splitter.split_text(sample_text)
                print(f"{splitter_name} splitter: {len(chunks)} chunks")

                if chunks:
                    preview = (
                        chunks[0].content[:100]
                        if hasattr(chunks[0], "content")
                        else str(chunks[0])[:100]
                    )
                    print(f"   Preview: {preview}...")
                else:
                    print("   No chunks generated")

                success_count += 1
            except Exception as e:
                print(f"{splitter_name} splitter failed: {e}")

            print("=" * 50)

        print(f"Successfully tested {success_count}/{len(splitter_tests)} text splitters")
        return success_count == len(splitter_tests)

    def check_simple_workflow(self) -> bool:
        """Demonstrate simple workflow functionality."""
        self._print_header("Simple Workflow")

        if not self._require_api_key("simple workflow"):
            return True  # Skipped, not a failure

        # Create workflow
        try:
            workflow = Workflow("Simple Workflow")
            print("Workflow created successfully")
        except Exception as e:
            print(f"Failed to create workflow: {e}")
            return False

        # Create and add node
        try:
            node = Node.agent(name="Greet User", prompt="Say hello to the user")
            workflow.add_node(node)
            print("Node added to workflow successfully")
        except Exception as e:
            print(f"Failed to add node to workflow: {e}")
            return False

        # Validate workflow
        try:
            workflow.validate()
            print("Workflow validation successful")
        except Exception as e:
            print(f"Workflow validation failed: {e}")
            return False

        # Create executor and execute workflow
        try:
            llm_config = LlmConfig.openai(api_key=self.api_key, model="gpt-4o-mini")
            executor = Executor(llm_config)
            print("Executor created successfully")

            result = executor.execute(workflow)
            output = result.get_node_output("Greet User")
            print("Workflow executed successfully")
            print(f"   Result: {output}")
        except Exception as e:
            print(f"Workflow execution failed: {e}")
            print("   This might be due to network issues, API key problems, or rate limits")
            return False

        print("=" * 50)
        return True

    def check_complex_workflow(self) -> bool:
        """Demonstrate complex workflow functionality."""
        self._print_header("Complex Workflow")

        if not self._require_api_key("complex workflow"):
            return True  # Skipped, not a failure

        # Create workflow
        try:
            workflow = Workflow("Complex Workflow")
            print("Complex workflow created successfully")
        except Exception as e:
            print(f"Failed to create complex workflow: {e}")
            return False

        # Create nodes
        try:
            node1 = Node.agent(
                name="Start",
                prompt="Generate a Beautiful topic from nature to write a poem on",
                agent_id="start_001",
            )
            node2 = Node.agent(
                name="Middle",
                prompt="Gather information about the topic",
                agent_id="middle_001",
            )
            node3 = Node.agent(
                name="End",
                prompt="write the poem",
                agent_id="end_001",
            )
            print("Workflow nodes created successfully")
        except Exception as e:
            print(f"Failed to create workflow nodes: {e}")
            return False

        # Add nodes to workflow
        try:
            node_id1 = workflow.add_node(node1)
            node_id2 = workflow.add_node(node2)
            node_id3 = workflow.add_node(node3)
            print("Nodes added to workflow successfully")
        except Exception as e:
            print(f"Failed to add nodes to workflow: {e}")
            return False

        # Connect nodes
        try:
            workflow.connect(node_id1, node_id2)
            workflow.connect(node_id2, node_id3)
            print("Workflow connections created successfully")
        except Exception as e:
            print(f"Failed to connect workflow nodes: {e}")
            return False

        # Validate workflow
        try:
            workflow.validate()
            print("Complex workflow validation successful")
        except Exception as e:
            print(f"Complex workflow validation failed: {e}")
            return False

        # Execute workflow
        try:
            llm_config = LlmConfig.openai(api_key=self.api_key, model="gpt-4o-mini")
            executor = Executor(llm_config)
            print("Executor created for complex workflow")

            result = executor.execute(workflow)
            end_output = result.get_node_output("End")
            all_outputs = result.get_all_node_outputs()

            print("Complex workflow executed successfully")
            print(f"   End result: {end_output}")
            print(f"   All outputs: {len(all_outputs)} nodes completed")
        except Exception as e:
            print(f"Complex workflow execution failed: {e}")
            print("   This might be due to network issues, API key problems, or rate limits")
            return False

        print("=" * 50)
        return True

    def check_tool_calling_workflow(self) -> bool:
        """Demonstrate tool calling workflow functionality."""
        self._print_header("Tool Calling Workflow")

        if not self._require_api_key("tool calling workflow"):
            return True  # Skipped, not a failure

        # Create tools
        try:

            @tool(_description="Get current weather information for any city")
            def get_weather(location: str) -> dict:
                return {"location": location, "temperature": 22, "condition": "sunny"}

            @tool(_description="Perform mathematical calculations and return results")
            def calculate(expression: str) -> str:
                # Safe-ish evaluation for demo purposes
                try:
                    allowed_chars = set("0123456789+-*/(). ")
                    if all(c in allowed_chars for c in expression):
                        result = eval(expression)
                        return f"Result: {result}"
                    else:
                        return "Error: Invalid expression"
                except Exception:
                    return "Error: Could not evaluate expression"

            print("Tools created successfully")
        except Exception as e:
            print(f"Failed to create tools: {e}")
            return False

        # Create workflow
        try:
            workflow = Workflow("Tool Calling Workflow")
            print("Tool calling workflow created successfully")
        except Exception as e:
            print(f"Failed to create tool calling workflow: {e}")
            return False

        # Create node with tools
        try:
            node = Node.agent(
                name="Greet User",
                prompt=(
                    "Say hello to the user and provide the output of the tool calling. "
                    "What is the weather in San Francisco? and calculate 15 + 27?"
                ),
                tools=[get_weather, calculate],
            )
            workflow.add_node(node)
            print("Node with tools added to workflow successfully")
        except Exception as e:
            print(f"Failed to add node with tools to workflow: {e}")
            return False

        # Validate workflow
        try:
            workflow.validate()
            print("Tool calling workflow validation successful")
        except Exception as e:
            print(f"Tool calling workflow validation failed: {e}")
            return False

        # Execute workflow
        try:
            llm_config = LlmConfig.openai(api_key=self.api_key, model="gpt-4o-mini")
            executor = Executor(llm_config)
            print("Executor created for tool calling workflow")

            result = executor.execute(workflow)
            output = result.get_node_output("Greet User")
            print("Tool calling workflow executed successfully")
            print(f"   Result: {output}")
        except Exception as e:
            print(f"Tool calling workflow execution failed: {e}")
            print("   This might be due to network issues, API key problems, or rate limits")
            return False

        print("=" * 50)
        return True


def main() -> None:
    diagnostics = GraphBitFunctionalityChecker()

    if not diagnostics.check_system_health():
        print("\nSystem health check failed")
        sys.exit(1)

    if not diagnostics.check_configuration():
        print("\nConfiguration check failed")
        sys.exit(1)

    if not diagnostics.check_llm_client():
        print("\nLLM client check failed")
        sys.exit(1)

    if not diagnostics.check_embeddings():
        print("\nEmbeddings check failed")
        sys.exit(1)

    if not diagnostics.check_document_loading():
        print("\nDocument loading check failed")
        sys.exit(1)

    if not diagnostics.check_text_splitter():
        print("\nText splitter check failed")
        sys.exit(1)

    if not diagnostics.check_simple_workflow():
        print("\nSimple workflow check failed")
        sys.exit(1)

    if not diagnostics.check_complex_workflow():
        print("\nComplex workflow check failed")
        sys.exit(1)

    if not diagnostics.check_tool_calling_workflow():
        print("\nTool calling workflow check failed")
        sys.exit(1)

    # Final success message
    print("\n\nAll GraphBit core version support checks completed successfully!")
    print("=" * 60)
    print("System is ready for GraphBit operations")
    print("=" * 60)


if __name__ == "__main__":
    main()
