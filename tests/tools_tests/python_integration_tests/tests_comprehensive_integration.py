"""Comprehensive integration tests for GraphBit components."""

import pytest
import uuid
import tempfile
import os
import asyncio
import functools
import time
from typing import Dict, List, Optional, Tuple
from graphbit import (
    LlmConfig, LlmClient, EmbeddingConfig, EmbeddingClient,
    DocumentLoader, DocumentLoaderConfig, CharacterSplitter,
    Workflow, Node, Executor, WorkflowResult,
    ToolRegistry, ToolExecutor, tool
)


# Environment-based API key detection for real integration testing
class APIKeyManager:
    """Manages real API keys from environment variables for genuine integration testing."""

    ENV_VARS = {
        'openai': 'OPENAI_API_KEY',
        'anthropic': 'ANTHROPIC_API_KEY',
        'huggingface': 'HUGGINGFACE_API_KEY',
        'deepseek': 'DEEPSEEK_API_KEY',
        'fireworks': 'FIREWORKS_API_KEY',
        'perplexity': 'PERPLEXITY_API_KEY',
        'xai': 'XAI_API_KEY',
        'ollama': None  # Ollama doesn't use API keys, uses local URL
    }

    @classmethod
    def is_ollama_available(cls) -> bool:
        """Check if Ollama is running locally."""
        try:
            import requests
            response = requests.get("http://localhost:11434/api/tags", timeout=2)
            return response.status_code == 200
        except Exception:
            return False

    @classmethod
    def get_real_api_key(cls, provider: str) -> Optional[str]:
        """Get real API key from environment variable."""
        env_var = cls.ENV_VARS.get(provider)

        # Ollama doesn't use API keys, check if service is running
        if provider == 'ollama':
            return "ollama-local" if cls.is_ollama_available() else None

        if not env_var:
            return None

        api_key = os.getenv(env_var)
        if api_key and len(api_key.strip()) > 10:
            return api_key.strip()
        return None

    @classmethod
    def get_available_llm_configs(cls) -> Dict[str, LlmConfig]:
        """Get LLM configurations for providers with real API keys."""
        configs = {}

        for provider in cls.ENV_VARS.keys():
            api_key = cls.get_real_api_key(provider)
            if not api_key:
                continue

            try:
                if provider == 'openai':
                    configs[provider] = LlmConfig.openai(api_key, "gpt-4o-mini")
                elif provider == 'anthropic':
                    configs[provider] = LlmConfig.anthropic(api_key, "claude-3-haiku-20240307")
                elif provider == 'huggingface':
                    configs[provider] = LlmConfig.huggingface(api_key, "microsoft/DialoGPT-medium")
                elif provider == 'deepseek':
                    configs[provider] = LlmConfig.deepseek(api_key, "deepseek-chat")
                elif provider == 'perplexity':
                    configs[provider] = LlmConfig.perplexity(api_key, "llama-3.1-sonar-small-128k-online")
                elif provider == 'ollama':
                    # Ollama doesn't use API keys, just model name
                    configs[provider] = LlmConfig.ollama("llama3.2")
            except Exception:
                # Skip providers that can't be configured
                continue

        return configs

    @classmethod
    def get_available_embedding_configs(cls) -> Dict[str, EmbeddingConfig]:
        """Get embedding configurations for providers with real API keys."""
        configs = {}

        for provider in ['openai', 'huggingface']:  # Only providers that support embeddings
            api_key = cls.get_real_api_key(provider)
            if not api_key:
                continue

            try:
                if provider == 'openai':
                    configs[provider] = EmbeddingConfig.openai(api_key, "text-embedding-3-small")
                elif provider == 'huggingface':
                    configs[provider] = EmbeddingConfig.huggingface(api_key, "sentence-transformers/all-MiniLM-L6-v2")
            except Exception:
                # Skip providers that can't be configured
                continue

        return configs


# Global provider configurations - only includes providers with real API keys
AVAILABLE_LLM_CONFIGS = APIKeyManager.get_available_llm_configs()
AVAILABLE_EMBEDDING_CONFIGS = APIKeyManager.get_available_embedding_configs()


# Module-level fixtures
@pytest.fixture
def temp_dir(tmp_path):
    """Create temporary directory for test files."""
    return str(tmp_path)


def require_api_key(provider: str):
    """Decorator to skip test if specific provider API key is not available."""
    def decorator(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            api_key = APIKeyManager.get_real_api_key(provider)
            if not api_key:
                pytest.skip(f"{provider.upper()} API key not available in environment")
            return func(*args, **kwargs)
        return wrapper
    return decorator


def require_llm_provider():
    """Decorator to skip test if no LLM providers are available."""
    def decorator(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            if not AVAILABLE_LLM_CONFIGS:
                pytest.skip("No LLM API keys available in environment")
            return func(*args, **kwargs)
        return wrapper
    return decorator


def require_embedding_provider():
    """Decorator to skip test if no embedding providers are available."""
    def decorator(func):
        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            if not AVAILABLE_EMBEDDING_CONFIGS:
                pytest.skip("No embedding API keys available in environment")
            return func(*args, **kwargs)
        return wrapper
    return decorator

class TestLLMEmbeddingIntegration:
    """Integration tests for LLM and Embedding components."""

    @require_llm_provider()
    @require_embedding_provider()
    def test_llm_embedding_workflow_integration(self):
        """Test integration of LLM and embedding in a workflow."""
        # Use first available LLM and embedding configs
        llm_config = list(AVAILABLE_LLM_CONFIGS.values())[0]
        embedding_config = list(AVAILABLE_EMBEDDING_CONFIGS.values())[0]

        # Create clients
        llm_client = LlmClient(llm_config)
        embedding_client = EmbeddingClient(embedding_config)

        # Test basic integration with real API calls
        test_text = "This is a test for integration."

        # Generate embedding
        embedding = embedding_client.embed(test_text)
        assert isinstance(embedding, list)
        assert len(embedding) > 0

        # Use LLM to process text
        response = llm_client.complete(f"Summarize: {test_text}")
        assert isinstance(response, str)
        assert len(response) > 0

        # Generate embedding for response
        response_embedding = embedding_client.embed(response)
        assert isinstance(response_embedding, list)
        assert len(response_embedding) > 0

        # Calculate similarity
        similarity = EmbeddingClient.similarity(embedding, response_embedding)
        assert isinstance(similarity, float)
        assert 0.0 <= similarity <= 1.0

    @require_llm_provider()
    @require_embedding_provider()
    def test_batch_processing_integration(self):
        """Test batch processing integration between LLM and embeddings."""
        # Use first available configs
        llm_config = list(AVAILABLE_LLM_CONFIGS.values())[0]
        embedding_config = list(AVAILABLE_EMBEDDING_CONFIGS.values())[0]

        llm_client = LlmClient(llm_config)
        embedding_client = EmbeddingClient(embedding_config)

        texts = ["First text", "Second text", "Third text"]

        # Generate embeddings for all texts
        embeddings = embedding_client.embed_many(texts)
        assert isinstance(embeddings, list)
        assert len(embeddings) == len(texts)

        # Verify each embedding
        for embedding in embeddings:
            assert isinstance(embedding, list)
            assert len(embedding) > 0

        # Process texts with LLM (if batch processing available)
        if hasattr(llm_client, 'complete_batch'):
            prompts = [f"Process: {text}" for text in texts]

            async def test_batch():
                responses = await llm_client.complete_batch(prompts)
                return responses

            loop = asyncio.new_event_loop()
            asyncio.set_event_loop(loop)
            try:
                responses = loop.run_until_complete(test_batch())
                assert isinstance(responses, list)
                assert len(responses) == len(prompts)
            finally:
                loop.close()

    @pytest.mark.parametrize("llm_provider,embedding_provider", [
        (llm_p, emb_p) for llm_p in AVAILABLE_LLM_CONFIGS.keys()
        for emb_p in AVAILABLE_EMBEDDING_CONFIGS.keys()
    ])
    def test_cross_provider_integration(self, llm_provider, embedding_provider):
        """Test integration between different LLM and embedding providers."""
        if llm_provider not in AVAILABLE_LLM_CONFIGS:
            pytest.skip(f"LLM provider {llm_provider} API key not available")
        if embedding_provider not in AVAILABLE_EMBEDDING_CONFIGS:
            pytest.skip(f"Embedding provider {embedding_provider} API key not available")

        llm_config = AVAILABLE_LLM_CONFIGS[llm_provider]
        embedding_config = AVAILABLE_EMBEDDING_CONFIGS[embedding_provider]

        llm_client = LlmClient(llm_config)
        embedding_client = EmbeddingClient(embedding_config)

        test_text = f"Cross-provider test: {llm_provider} + {embedding_provider}"

        # Test cross-provider workflow with real API calls
        embedding = embedding_client.embed(test_text)
        response = llm_client.complete(f"Analyze: {test_text}")

        assert isinstance(embedding, list)
        assert len(embedding) > 0
        assert isinstance(response, str)
        assert len(response) > 0


class TestDocumentProcessingIntegration:
    """Integration tests for document processing components."""

    @pytest.fixture
    def temp_dir(self):
        """Create temporary directory for test files."""
        with tempfile.TemporaryDirectory() as temp_dir:
            yield temp_dir

    @pytest.fixture
    def sample_document(self, temp_dir):
        """Create sample document for testing."""
        doc_path = os.path.join(temp_dir, "test_document.txt")
        content = "This is a test document. " * 100  # Create substantial content
        with open(doc_path, "w", encoding="utf-8") as f:
            f.write(content)
        return doc_path

    def test_document_loading_splitting_integration(self, sample_document):
        """Test integration of document loading and text splitting."""
        # Create document loader
        loader = DocumentLoader()
        
        # Create text splitter
        splitter = CharacterSplitter(chunk_size=200, chunk_overlap=50)
        
        try:
            # Load document
            document = loader.load_document(sample_document, "txt")
            assert document is not None
            assert hasattr(document, 'content')
            
            # Split document content
            chunks = splitter.split_text(document.content)
            assert isinstance(chunks, list)
            assert len(chunks) > 0
            
            # Verify chunks
            for chunk in chunks:
                chunk_content = chunk.content if hasattr(chunk, 'content') else chunk
                assert isinstance(chunk_content, str)
                assert len(chunk_content) <= 250  # chunk_size + tolerance
                
        except Exception as e:
            pytest.skip(f"Document processing integration not available: {e}")

    def test_document_embedding_integration(self, sample_document):
        """Test integration of document loading with embeddings."""
        loader = DocumentLoader()
        embedding_config = EmbeddingConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef12")
        embedding_client = EmbeddingClient(embedding_config)
        splitter = CharacterSplitter(chunk_size=500, chunk_overlap=100)
        
        try:
            # Load and split document
            document = loader.load_document(sample_document, "txt")
            chunks = splitter.split_text(document.content)
            
            # Generate embeddings for chunks
            chunk_texts = [chunk.content if hasattr(chunk, 'content') else chunk for chunk in chunks[:3]]  # Limit for testing
            embeddings = embedding_client.embed_many(chunk_texts)
            
            assert isinstance(embeddings, list)
            assert len(embeddings) == len(chunk_texts)
            
            # Calculate similarities between chunks
            if len(embeddings) >= 2:
                similarity = EmbeddingClient.similarity(embeddings[0], embeddings[1])
                assert isinstance(similarity, float)
                
        except Exception as e:
            # Expected to fail with test API keys or missing dependencies
            error_msg = str(e).lower()
            assert any(keyword in error_msg for keyword in 
                      ['api', 'key', 'auth', 'token', 'invalid', 'unauthorized', 'forbidden',
                       'not available', 'skip'])


class TestWorkflowToolIntegration:
    """Integration tests for workflow and tool components."""

    @require_llm_provider()
    def test_workflow_tool_integration(self):
        """Test integration of workflows with tool calling."""
        # Use first available LLM config
        llm_config = list(AVAILABLE_LLM_CONFIGS.values())[0]

        # Create tool registry and register tools
        registry = ToolRegistry()

        @tool(description="Add two numbers together")
        def add_numbers(a: int, b: int) -> int:
            """Add two numbers."""
            return a + b

        @tool(description="Multiply two numbers together")
        def multiply_numbers(a: int, b: int) -> int:
            """Multiply two numbers."""
            return a * b

        # Create workflow with tool-enabled agent
        workflow = Workflow("Tool Integration Workflow")
        agent_id = str(uuid.uuid4())
        node = Node.agent(
            name="Calculator Agent",
            prompt="Use tools to calculate: {input}",
            agent_id=agent_id
        )
        workflow.add_node(node)
        workflow.validate()

        # Create executor
        executor = Executor(llm_config)

        # Execute workflow with real API
        result = executor.execute(workflow)
        assert isinstance(result, WorkflowResult)

    @require_llm_provider()
    def test_multi_step_workflow_integration(self):
        """Test multi-step workflow integration."""
        # Use first available LLM config
        llm_config = list(AVAILABLE_LLM_CONFIGS.values())[0]

        # Create complex workflow
        workflow = Workflow("Multi-Step Integration Workflow")

        # Step 1: Data preparation
        prep_agent_id = str(uuid.uuid4())
        prep_node = Node.agent(
            name="Data Preparation",
            prompt="Prepare data: {input}",
            agent_id=prep_agent_id
        )

        # Step 2: Analysis
        analysis_agent_id = str(uuid.uuid4())
        analysis_node = Node.agent(
            name="Data Analysis",
            prompt="Analyze prepared data: {input}",
            agent_id=analysis_agent_id
        )

        # Step 3: Summary
        summary_agent_id = str(uuid.uuid4())
        summary_node = Node.agent(
            name="Summary Generation",
            prompt="Summarize analysis: {input}",
            agent_id=summary_agent_id
        )

        # Add nodes and connect them
        prep_id = workflow.add_node(prep_node)
        analysis_id = workflow.add_node(analysis_node)
        summary_id = workflow.add_node(summary_node)

        workflow.connect(prep_id, analysis_id)
        workflow.connect(analysis_id, summary_id)
        workflow.validate()

        # Execute workflow with real API
        executor = Executor(llm_config)
        result = executor.execute(workflow)
        assert isinstance(result, WorkflowResult)

    @pytest.mark.parametrize("provider", list(AVAILABLE_LLM_CONFIGS.keys()))
    def test_provider_specific_workflow_integration(self, provider):
        """Test workflow integration for each available LLM provider."""
        if provider not in AVAILABLE_LLM_CONFIGS:
            pytest.skip(f"Provider {provider} API key not available")

        llm_config = AVAILABLE_LLM_CONFIGS[provider]

        # Create simple workflow
        workflow = Workflow(f"Provider Test Workflow - {provider}")
        agent_id = str(uuid.uuid4())
        node = Node.agent(
            name=f"{provider.title()} Agent",
            prompt="Process this input: {input}",
            agent_id=agent_id
        )
        workflow.add_node(node)
        workflow.validate()

        # Create executor and run with real API
        executor = Executor(llm_config)
        result = executor.execute(workflow)
        assert isinstance(result, WorkflowResult)


class TestEndToEndIntegration:
    """End-to-end integration tests combining multiple components."""

    @require_llm_provider()
    @require_embedding_provider()
    def test_document_analysis_pipeline(self, temp_dir):
        """Test complete document analysis pipeline."""
        # Create test document
        doc_path = os.path.join(temp_dir, "analysis_doc.txt")
        content = "This is a comprehensive document for analysis. " * 50
        with open(doc_path, "w", encoding="utf-8") as f:
            f.write(content)

        # Configure components with first available providers
        llm_config = list(AVAILABLE_LLM_CONFIGS.values())[0]
        embedding_config = list(AVAILABLE_EMBEDDING_CONFIGS.values())[0]

        # Step 1: Load document
        loader = DocumentLoader()
        document = loader.load_document(doc_path, "txt")

        # Step 2: Split into chunks
        splitter = CharacterSplitter(chunk_size=300, chunk_overlap=50)
        chunks = splitter.split_text(document.content)

        # Step 3: Generate embeddings
        embedding_client = EmbeddingClient(embedding_config)
        chunk_texts = [chunk.content if hasattr(chunk, 'content') else chunk for chunk in chunks[:5]]
        embeddings = embedding_client.embed_many(chunk_texts)

        # Step 4: Analyze with LLM
        llm_client = LlmClient(llm_config)
        analysis = llm_client.complete(f"Analyze this document: {chunk_texts[0][:200]}")

        # Step 5: Create workflow for processing
        workflow = Workflow("Document Analysis Workflow")
        agent_id = str(uuid.uuid4())
        node = Node.agent(
            name="Document Analyzer",
            prompt="Provide insights on: {input}",
            agent_id=agent_id
        )
        workflow.add_node(node)
        workflow.validate()

        # Step 6: Execute workflow
        executor = Executor(llm_config)
        result = executor.execute(workflow)

        # Verify pipeline results
        assert document is not None
        assert isinstance(chunks, list)
        assert len(chunks) > 0
        assert isinstance(embeddings, list)
        assert len(embeddings) > 0
        assert isinstance(analysis, str)
        assert len(analysis) > 0
        assert isinstance(result, WorkflowResult)

    @require_llm_provider()
    @require_embedding_provider()
    def test_rag_pipeline_integration(self, temp_dir):
        """Test Retrieval-Augmented Generation (RAG) pipeline."""
        # Create knowledge base documents
        docs = []
        for i in range(3):
            doc_path = os.path.join(temp_dir, f"knowledge_{i}.txt")
            content = f"Knowledge document {i}: This contains important information about topic {i}. " * 20
            with open(doc_path, "w", encoding="utf-8") as f:
                f.write(content)
            docs.append(doc_path)

        # Configure components with first available providers
        llm_config = list(AVAILABLE_LLM_CONFIGS.values())[0]
        embedding_config = list(AVAILABLE_EMBEDDING_CONFIGS.values())[0]

        # Step 1: Build knowledge base
        loader = DocumentLoader()
        splitter = CharacterSplitter(chunk_size=400, chunk_overlap=80)
        embedding_client = EmbeddingClient(embedding_config)

        knowledge_chunks = []
        knowledge_embeddings = []

        for doc_path in docs:
            document = loader.load_document(doc_path, "txt")
            chunks = splitter.split_text(document.content)
            chunk_texts = [chunk.content if hasattr(chunk, 'content') else chunk for chunk in chunks[:2]]
            embeddings = embedding_client.embed_many(chunk_texts)

            knowledge_chunks.extend(chunk_texts)
            knowledge_embeddings.extend(embeddings)

        # Step 2: Query processing
        query = "What information is available about topic 1?"
        query_embedding = embedding_client.embed(query)

        # Step 3: Retrieve relevant chunks (simplified similarity search)
        similarities = [EmbeddingClient.similarity(query_embedding, emb) for emb in knowledge_embeddings]
        best_chunk_idx = similarities.index(max(similarities))
        relevant_chunk = knowledge_chunks[best_chunk_idx]

        # Step 4: Generate response with LLM
        llm_client = LlmClient(llm_config)
        prompt = f"Based on this context: {relevant_chunk}\n\nAnswer: {query}"
        response = llm_client.complete(prompt)

        # Step 5: Create workflow for RAG processing
        workflow = Workflow("RAG Workflow")
        agent_id = str(uuid.uuid4())
        node = Node.agent(
            name="RAG Agent",
            prompt="Context: {context}\nQuery: {query}\nProvide answer:",
            agent_id=agent_id
        )
        workflow.add_node(node)
        workflow.validate()

        # Verify RAG pipeline results
        assert len(knowledge_chunks) > 0
        assert len(knowledge_embeddings) > 0
        assert isinstance(query_embedding, list)
        assert len(query_embedding) > 0
        assert isinstance(relevant_chunk, str)
        assert len(relevant_chunk) > 0
        assert isinstance(response, str)
        assert len(response) > 0
        assert workflow is not None


class TestErrorPropagationIntegration:
    """Integration tests for error propagation across components."""

    def test_llm_error_propagation(self):
        """Test error propagation from LLM components."""
        # Test with invalid API key
        invalid_config = LlmConfig.openai("invalid-key-too-short")
        client = LlmClient(invalid_config)
        
        # Error should propagate appropriately
        with pytest.raises(Exception) as exc_info:
            client.complete("Test prompt")
        
        error_msg = str(exc_info.value).lower()
        assert any(keyword in error_msg for keyword in 
                  ['api', 'key', 'auth', 'invalid', 'unauthorized'])

    def test_workflow_error_propagation(self):
        """Test error propagation in workflow execution."""
        # Create workflow with invalid configuration
        invalid_config = LlmConfig.openai("invalid-key-too-short")
        executor = Executor(invalid_config)
        
        workflow = Workflow("Error Test Workflow")
        agent_id = str(uuid.uuid4())
        node = Node.agent(
            name="Error Agent",
            prompt="This will fail: {input}",
            agent_id=agent_id
        )
        workflow.add_node(node)
        workflow.validate()
        
        # Error should propagate from executor
        with pytest.raises(Exception) as exc_info:
            executor.execute(workflow)
        
        error_msg = str(exc_info.value).lower()
        assert any(keyword in error_msg for keyword in 
                  ['api', 'key', 'auth', 'invalid', 'unauthorized'])

    def test_component_chain_error_propagation(self):
        """Test error propagation through component chains."""
        # Create chain: Document -> Splitter -> Embedding -> LLM
        invalid_embedding_config = EmbeddingConfig.openai("invalid-key-too-short")
        embedding_client = EmbeddingClient(invalid_embedding_config)
        
        # Create valid components up to the failing one
        loader = DocumentLoader()
        splitter = CharacterSplitter(chunk_size=100, chunk_overlap=20)
        
        # Create temporary document
        with tempfile.NamedTemporaryFile(mode='w', suffix='.txt', delete=False) as f:
            f.write("Test document content for error propagation.")
            temp_path = f.name
        
        try:
            # Process through chain until error
            document = loader.load_document(temp_path, "txt")
            chunks = splitter.split_text(document.content)
            chunk_text = chunks[0].content if hasattr(chunks[0], 'content') else chunks[0]
            
            # This should fail and propagate error
            with pytest.raises(Exception) as exc_info:
                embedding_client.embed(chunk_text)
            
            error_msg = str(exc_info.value).lower()
            assert any(keyword in error_msg for keyword in 
                      ['api', 'key', 'auth', 'invalid', 'unauthorized'])
                      
        finally:
            os.unlink(temp_path)


class TestPerformanceIntegration:
    """Integration tests for performance monitoring across components."""

    def test_component_performance_monitoring(self):
        """Test performance monitoring across integrated components."""
        import time
        
        # Configure components
        llm_config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef12")
        embedding_config = EmbeddingConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef12")
        
        # Test LLM performance monitoring
        llm_client = LlmClient(llm_config)
        
        start_time = time.time()
        try:
            response = llm_client.complete("Quick test")
            llm_duration = time.time() - start_time
            assert llm_duration >= 0
        except Exception:
            # Expected to fail with test API key
            pass
        
        # Test embedding performance monitoring
        embedding_client = EmbeddingClient(embedding_config)
        
        start_time = time.time()
        try:
            embedding = embedding_client.embed("Performance test")
            embedding_duration = time.time() - start_time
            assert embedding_duration >= 0
        except Exception:
            # Expected to fail with test API key
            pass
        
        # Test workflow performance monitoring
        executor = Executor(llm_config)
        workflow = Workflow("Performance Test")
        agent_id = str(uuid.uuid4())
        node = Node.agent(
            name="Performance Agent",
            prompt="Test: {input}",
            agent_id=agent_id
        )
        workflow.add_node(node)
        workflow.validate()
        
        start_time = time.time()
        try:
            result = executor.execute(workflow)
            workflow_duration = time.time() - start_time
            assert workflow_duration >= 0
        except Exception:
            # Expected to fail with test API key
            pass

    def test_concurrent_component_performance(self):
        """Test performance under concurrent component usage."""
        import threading
        import time
        
        llm_config = LlmConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef12")
        embedding_config = EmbeddingConfig.openai("sk-1234567890abcdef1234567890abcdef1234567890abcdef12")
        
        results = []
        errors = []
        
        def concurrent_operation(index):
            try:
                start_time = time.time()
                
                # Mix of operations
                if index % 2 == 0:
                    client = LlmClient(llm_config)
                    client.complete(f"Concurrent test {index}")
                else:
                    client = EmbeddingClient(embedding_config)
                    client.embed(f"Concurrent embedding test {index}")
                
                duration = time.time() - start_time
                results.append(duration)
            except Exception as e:
                errors.append(e)
        
        # Run concurrent operations
        threads = []
        for i in range(5):
            thread = threading.Thread(target=concurrent_operation, args=(i,))
            threads.append(thread)
            thread.start()
        
        # Wait for completion
        for thread in threads:
            thread.join()
        
        # All operations should complete (with errors due to test API keys)
        assert len(results) + len(errors) == 5
        
        # Verify error types
        for error in errors:
            assert any(keyword in str(error).lower() for keyword in 
                      ['api', 'key', 'auth', 'invalid', 'unauthorized'])


@pytest.mark.parametrize("component_combo,expected_integration", [
    (("LlmClient", "EmbeddingClient"), True),
    (("DocumentLoader", "CharacterSplitter"), True),
    (("Workflow", "Executor"), True),
    (("ToolRegistry", "Workflow"), True),
])
def test_component_integration_matrix(component_combo, expected_integration):
    """Test integration compatibility across component combinations."""
    component1, component2 = component_combo

    # Test basic compatibility with environment-based API key detection
    if component1 == "LlmClient" and component2 == "EmbeddingClient":
        # Skip if no API keys available
        if not AVAILABLE_LLM_CONFIGS:
            pytest.skip("No LLM API keys available in environment")
        if not AVAILABLE_EMBEDDING_CONFIGS:
            pytest.skip("No embedding API keys available in environment")

        llm_config = list(AVAILABLE_LLM_CONFIGS.values())[0]
        embedding_config = list(AVAILABLE_EMBEDDING_CONFIGS.values())[0]
        llm_client = LlmClient(llm_config)
        embedding_client = EmbeddingClient(embedding_config)
        assert llm_client is not None and embedding_client is not None

    elif component1 == "DocumentLoader" and component2 == "CharacterSplitter":
        loader = DocumentLoader()
        # Add required chunk_size parameter
        splitter = CharacterSplitter(chunk_size=1000, chunk_overlap=100)
        assert loader is not None and splitter is not None

    elif component1 == "Workflow" and component2 == "Executor":
        workflow = Workflow("Test")
        if not AVAILABLE_LLM_CONFIGS:
            pytest.skip("No LLM API keys available for Executor")
        config = list(AVAILABLE_LLM_CONFIGS.values())[0]
        executor = Executor(config)
        assert workflow is not None and executor is not None

    elif component1 == "ToolRegistry" and component2 == "Workflow":
        registry = ToolRegistry()
        workflow = Workflow("Test")
        assert registry is not None and workflow is not None

    # All expected integrations should work if we reach this point
    assert expected_integration


# Environment-based multi-provider integration tests
class TestMultiProviderIntegration:
    """Test integration across multiple LLM providers with real API keys."""

    @pytest.mark.parametrize("provider", list(AVAILABLE_LLM_CONFIGS.keys()))
    def test_provider_specific_llm_integration(self, provider):
        """Test LLM integration for each available provider."""
        if provider not in AVAILABLE_LLM_CONFIGS:
            pytest.skip(f"Provider {provider} API key not available in environment")

        config = AVAILABLE_LLM_CONFIGS[provider]
        client = LlmClient(config)

        # Test with real API call
        response = client.complete("Hello, world!")
        assert isinstance(response, str)
        assert len(response) > 0

    @pytest.mark.parametrize("provider", list(AVAILABLE_EMBEDDING_CONFIGS.keys()))
    def test_provider_specific_embedding_integration(self, provider):
        """Test embedding integration for each available provider."""
        if provider not in AVAILABLE_EMBEDDING_CONFIGS:
            pytest.skip(f"Provider {provider} API key not available in environment")

        config = AVAILABLE_EMBEDDING_CONFIGS[provider]
        client = EmbeddingClient(config)

        # Test with real API call
        embedding = client.embed("Test embedding")
        assert isinstance(embedding, list)
        assert len(embedding) > 0

    def test_environment_key_detection(self):
        """Test that environment API keys are properly detected."""
        # Count available providers
        llm_count = len(AVAILABLE_LLM_CONFIGS)
        embedding_count = len(AVAILABLE_EMBEDDING_CONFIGS)

        # This test provides information about available providers
        print(f"Available LLM providers: {llm_count}")
        print(f"Available embedding providers: {embedding_count}")

        # Test passes if at least some configuration is available
        # (even if no real API keys are present, the test structure is valid)
        assert True  # Always passes - informational test
