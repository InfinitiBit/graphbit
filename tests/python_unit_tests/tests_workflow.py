"""Unit tests for workflow functionality."""

import os

import pytest

import graphbit


def get_api_key(provider: str) -> str:
    """Get API key from environment variables."""
    key = os.getenv(f"{provider.upper()}_API_KEY")
    if not key:
        pytest.skip(f"No {provider.upper()}_API_KEY found in environment")
    assert key is not None
    return key


class TestNode:
    """Test workflow node functionality."""

    def test_agent_node_creation(self):
        """Test creating agent node."""
        node = graphbit.Node.agent(name="test_agent", prompt="What is {input}?", agent_id="test_agent_1", output_name="result")
        assert node is not None
        assert node.name() == "test_agent"
        assert node.id() is not None

    def test_transform_node_creation(self):
        """Test creating transform node."""
        node = graphbit.Node.transform(name="test_transform", transformation="lambda x: x.upper()")
        assert node is not None
        assert node.name() == "test_transform"
        assert node.id() is not None

    def test_condition_node_creation(self):
        """Test creating condition node."""
        node = graphbit.Node.condition(name="test_condition", expression="value > 10")
        assert node is not None
        assert node.name() == "test_condition"
        assert node.id() is not None

    def test_node_validation(self):
        """Test node validation."""
        with pytest.raises(ValueError):
            graphbit.Node.agent(name="", prompt="test")
        with pytest.raises(ValueError):
            graphbit.Node.agent(name="test", prompt="")
        with pytest.raises(ValueError):
            graphbit.Node.transform(name="", transformation="test")
        with pytest.raises(ValueError):
            graphbit.Node.condition(name="test", expression="")


class TestWorkflow:
    """Test workflow functionality."""

    def test_workflow_creation(self):
        """Test creating workflow."""
        workflow = graphbit.Workflow("test_workflow")
        assert workflow is not None
        assert workflow.name() == "test_workflow"

    def test_workflow_add_node(self):
        """Test adding node to workflow."""
        workflow = graphbit.Workflow("test_workflow")
        node = graphbit.Node.agent(name="test_agent", prompt="What is {input}?")
        node_id = workflow.add_node(node)
        assert node_id is not None

    def test_workflow_connect_nodes(self):
        """Test connecting workflow nodes."""
        workflow = graphbit.Workflow("test_workflow")
        node1 = graphbit.Node.agent(name="agent1", prompt="First step")
        node2 = graphbit.Node.agent(name="agent2", prompt="Second step")
        id1 = workflow.add_node(node1)
        id2 = workflow.add_node(node2)
        workflow.connect(id1, id2)

    def test_workflow_connect_invalid(self):
        """Non-existent IDs raise; self-loop connects successfully."""
        workflow = graphbit.Workflow("wf_invalid")
        with pytest.raises(ValueError):
            workflow.connect("missing1", "missing2")
        node = graphbit.Node.agent(name="self", prompt="p")
        nid = workflow.add_node(node)
        workflow.connect(nid, nid)

    def test_workflow_validation(self):
        """Test workflow validation."""
        workflow = graphbit.Workflow("test_workflow")
        assert workflow.validate() is None


class TestExecutor:
    """Test workflow executor functionality."""

    def test_executor_creation(self):
        """Test creating executor."""
        api_key = get_api_key("openai")
        config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        executor = graphbit.Executor(config)
        assert executor is not None

    def test_executor_creation_anthropic(self):
        """Test creating executor with Anthropic."""
        api_key = get_api_key("anthropic")
        config = graphbit.LlmConfig.anthropic(api_key=api_key, model="claude-3-5-sonnet-20241022")
        executor = graphbit.Executor(config)
        assert executor is not None

    def test_executor_creation_deepseek(self):
        """Test creating executor with DeepSeek."""
        api_key = get_api_key("deepseek")
        config = graphbit.LlmConfig.deepseek(api_key=api_key, model="deepseek-chat")
        executor = graphbit.Executor(config)
        assert executor is not None

    def test_executor_creation_perplexity(self):
        """Test creating executor with Perplexity."""
        api_key = get_api_key("perplexity")
        config = graphbit.LlmConfig.perplexity(api_key=api_key, model="sonar")
        executor = graphbit.Executor(config)
        assert executor is not None

    def test_executor_high_throughput(self):
        """Test creating high-throughput executor."""
        api_key = get_api_key("openai")
        config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        executor = graphbit.Executor.new_high_throughput(config, timeout_seconds=300)
        assert executor is not None
        assert executor.get_execution_mode() == "HighThroughput"

    def test_executor_low_latency(self):
        """Test creating low-latency executor."""
        api_key = get_api_key("openai")
        config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        executor = graphbit.Executor.new_low_latency(config, timeout_seconds=30)
        assert executor is not None
        assert executor.get_execution_mode() == "LowLatency"

    def test_executor_memory_optimized(self):
        """Test creating memory-optimized executor."""
        api_key = get_api_key("openai")
        config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        executor = graphbit.Executor.new_memory_optimized(config, timeout_seconds=300)
        assert executor is not None
        assert executor.get_execution_mode() == "MemoryOptimized"

    @pytest.mark.asyncio
    async def test_executor_execute_workflow(self):
        """Test executing workflow."""
        api_key = get_api_key("openai")
        config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        executor = graphbit.Executor(config)
        workflow = graphbit.Workflow("test_workflow")
        node = graphbit.Node.agent(name="test_agent", prompt="Say hello!")
        workflow.add_node(node)
        result = await executor.run_async(workflow)
        assert result is not None

    @pytest.mark.asyncio
    async def test_executor_execute_workflow_anthropic(self):
        """Test executing workflow with Anthropic (skips if API key missing)."""
        api_key = get_api_key("anthropic")
        config = graphbit.LlmConfig.anthropic(api_key=api_key, model="claude-3-5-sonnet-20241022")
        executor = graphbit.Executor(config)
        workflow = graphbit.Workflow("test_workflow_anthropic")
        node = graphbit.Node.agent(name="test_agent", prompt="Say hello!")
        workflow.add_node(node)
        result = await executor.run_async(workflow)
        assert result is not None

    @pytest.mark.asyncio
    async def test_executor_execute_workflow_deepseek(self):
        """Test executing workflow with DeepSeek (skips if API key missing)."""
        api_key = get_api_key("deepseek")
        config = graphbit.LlmConfig.deepseek(api_key=api_key, model="deepseek-chat")
        executor = graphbit.Executor(config)
        workflow = graphbit.Workflow("test_workflow_deepseek")
        node = graphbit.Node.agent(name="test_agent", prompt="Say hello!")
        workflow.add_node(node)
        result = await executor.run_async(workflow)
        assert result is not None

    @pytest.mark.asyncio
    async def test_executor_execute_workflow_perplexity(self):
        """Test executing workflow with Perplexity (skips if API key missing)."""
        api_key = get_api_key("perplexity")
        config = graphbit.LlmConfig.perplexity(api_key=api_key, model="sonar")
        executor = graphbit.Executor(config)
        workflow = graphbit.Workflow("test_workflow_perplexity")
        node = graphbit.Node.agent(name="test_agent", prompt="Say hello!")
        workflow.add_node(node)
        result = await executor.run_async(workflow)
        assert result is not None

    def test_executor_configuration(self):
        """Test executor configuration."""
        api_key = get_api_key("openai")
        config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        executor = graphbit.Executor(config)
        executor.configure(timeout_seconds=120, max_retries=3, enable_metrics=True, debug=True)
        stats = executor.get_stats()
        assert isinstance(stats, dict)
        assert "total_executions" in stats
        assert "successful_executions" in stats
        assert "failed_executions" in stats
        executor.configure(timeout_seconds=60, max_retries=1, enable_metrics=False, debug=False)
        stats2 = executor.get_stats()
        assert set(stats2.keys()) == set(stats.keys())

    def test_executor_stats_reset(self):
        """Test resetting executor statistics."""
        api_key = get_api_key("openai")
        config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        executor = graphbit.Executor(config)
        executor.reset_stats()
        stats = executor.get_stats()
        assert stats["total_executions"] == 0
        assert stats["successful_executions"] == 0
        assert stats["failed_executions"] == 0


class TestWorkflowErrorHandling:
    """Test workflow error handling."""

    def test_empty_workflow_execution(self):
        """Test executing empty workflow."""
        api_key = get_api_key("openai")
        config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        executor = graphbit.Executor(config)
        workflow = graphbit.Workflow("empty_workflow")
        with pytest.raises(ValueError):
            executor.execute(workflow)

    def test_invalid_node_connection(self):
        """Test invalid node connection."""
        workflow = graphbit.Workflow("test_workflow")
        with pytest.raises(ValueError):
            workflow.connect("nonexistent1", "nonexistent2")

    def test_executor_timeout(self):
        """Test executor timeout handling."""
        api_key = get_api_key("openai")
        config = graphbit.LlmConfig.openai(api_key=api_key, model="gpt-4-turbo")
        executor = graphbit.Executor(config, timeout_seconds=1)
        workflow = graphbit.Workflow("test_workflow")
        node = graphbit.Node.agent(name="slow_agent", prompt="Think very carefully about this for a long time...")
        workflow.add_node(node)
        with pytest.raises(Exception) as exc_info:
            executor.execute(workflow)
        assert "timeout" in str(exc_info.value).lower()
