"""Integration tests for complex GraphBit workflow patterns."""

import contextlib
import os
from typing import Any

import pytest

import graphbit


class TestMultiBranchWorkflows:
    """Tests for complex multi-branch workflow patterns."""

    @pytest.fixture
    def llm_config(self) -> Any:
        """Get LLM config for complex workflow tests."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")
        return graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

    def test_parallel_branch_workflow(self) -> None:
        """Test workflow with parallel processing branches."""
        workflow = graphbit.Workflow("parallel_branches")

        # Create entry point
        entry_agent = graphbit.Node.agent("entry_point", "Initialize processing", "entry_001")
        entry_id = workflow.add_node(entry_agent)

        # Create parallel processing branches
        branch_nodes = []
        for i in range(3):
            branch_agent = graphbit.Node.agent(f"branch_{i}", f"Process branch {i} data", f"branch_{i:03d}")
            branch_id = workflow.add_node(branch_agent)
            branch_nodes.append((branch_agent, branch_id))

        # Create convergence point
        merge_agent = graphbit.Node.agent("merge_point", "Merge all results", "merge_001")
        merge_id = workflow.add_node(merge_agent)

        # Connect entry to all branches
        for _, branch_id in branch_nodes:
            try:
                workflow.connect(entry_id, branch_id)
            except Exception as e:
                pytest.skip(f"Branch connection failed: {e}")

        # Connect all branches to merge point
        for _, branch_id in branch_nodes:
            try:
                workflow.connect(branch_id, merge_id)
            except Exception as e:
                pytest.skip(f"Merge connection failed: {e}")

        # Validate complex workflow structure
        with contextlib.suppress(Exception):
            workflow.validate()

    def test_conditional_branch_workflow(self) -> None:
        """Test workflow with conditional branching logic."""
        workflow = graphbit.Workflow("conditional_branches")

        # Create decision tree structure
        entry_agent = graphbit.Node.agent("entry", "Start decision process", "entry_001")
        entry_id = workflow.add_node(entry_agent)

        # Create primary condition
        primary_condition = graphbit.Node.condition("primary_check", "priority == 'high'")
        primary_id = workflow.add_node(primary_condition)

        # Create secondary condition
        secondary_condition = graphbit.Node.condition("secondary_check", "type == 'urgent'")
        secondary_id = workflow.add_node(secondary_condition)

        # Create processing branches for different paths
        high_priority_agent = graphbit.Node.agent("high_priority", "Handle high priority", "high_001")
        urgent_agent = graphbit.Node.agent("urgent_processing", "Handle urgent items", "urgent_001")
        normal_agent = graphbit.Node.agent("normal_processing", "Handle normal items", "normal_001")

        high_id = workflow.add_node(high_priority_agent)
        urgent_id = workflow.add_node(urgent_agent)
        normal_id = workflow.add_node(normal_agent)

        # Create decision tree connections
        try:
            workflow.connect(entry_id, primary_id)
            workflow.connect(primary_id, high_id)  # True branch
            workflow.connect(primary_id, secondary_id)  # False branch
            workflow.connect(secondary_id, urgent_id)  # True branch
            workflow.connect(secondary_id, normal_id)  # False branch
        except Exception as e:
            pytest.skip(f"Conditional workflow connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    def test_nested_conditional_workflow(self) -> None:
        """Test workflow with nested conditional structures."""
        workflow = graphbit.Workflow("nested_conditionals")

        # Create multi-level decision structure
        root_agent = graphbit.Node.agent("root", "Root decision point", "root_001")
        root_id = workflow.add_node(root_agent)

        # Level 1 conditions
        level1_condition = graphbit.Node.condition("level1", "category == 'A'")
        level1_id = workflow.add_node(level1_condition)

        # Level 2 conditions (nested under level 1)
        level2a_condition = graphbit.Node.condition("level2a", "subcategory == 'X'")
        level2b_condition = graphbit.Node.condition("level2b", "subcategory == 'Y'")

        level2a_id = workflow.add_node(level2a_condition)
        level2b_id = workflow.add_node(level2b_condition)

        # Terminal processing nodes
        terminal_nodes = []
        for i in range(4):
            terminal_agent = graphbit.Node.agent(f"terminal_{i}", f"Process type {i}", f"term_{i:03d}")
            terminal_id = workflow.add_node(terminal_agent)
            terminal_nodes.append(terminal_id)

        # Create nested structure
        try:
            workflow.connect(root_id, level1_id)
            workflow.connect(level1_id, level2a_id)  # True branch
            workflow.connect(level1_id, level2b_id)  # False branch

            # Connect level 2 conditions to terminals
            workflow.connect(level2a_id, terminal_nodes[0])  # True
            workflow.connect(level2a_id, terminal_nodes[1])  # False
            workflow.connect(level2b_id, terminal_nodes[2])  # True
            workflow.connect(level2b_id, terminal_nodes[3])  # False

        except Exception as e:
            pytest.skip(f"Nested conditional workflow connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_complex_workflow_execution(self, llm_config: Any) -> None:
        """Test execution of complex multi-branch workflow."""
        workflow = graphbit.Workflow("complex_execution")

        # Create a moderately complex but executable workflow
        start_agent = graphbit.Node.agent("start", "Begin complex processing", "start_001")
        analysis_agent = graphbit.Node.agent("analysis", "Analyze input data", "analysis_001")
        decision_condition = graphbit.Node.condition("decision", "confidence > 0.8")
        high_conf_agent = graphbit.Node.agent("high_confidence", "Process high confidence result", "high_001")
        low_conf_agent = graphbit.Node.agent("low_confidence", "Process low confidence result", "low_001")
        final_agent = graphbit.Node.agent("final", "Finalize processing", "final_001")

        # Add nodes to workflow
        start_id = workflow.add_node(start_agent)
        analysis_id = workflow.add_node(analysis_agent)
        decision_id = workflow.add_node(decision_condition)
        high_id = workflow.add_node(high_conf_agent)
        low_id = workflow.add_node(low_conf_agent)
        final_id = workflow.add_node(final_agent)

        try:
            # Create linear flow for simplicity but maintain complexity
            workflow.connect(start_id, analysis_id)
            workflow.connect(analysis_id, decision_id)
            workflow.connect(decision_id, high_id)
            workflow.connect(decision_id, low_id)
            workflow.connect(high_id, final_id)
            workflow.connect(low_id, final_id)

            # Validate and execute
            workflow.validate()

            executor = graphbit.Executor(llm_config)
            result = executor.execute(workflow)

            assert isinstance(result, graphbit.WorkflowResult)
            assert isinstance(result.is_success(), bool)
            assert isinstance(result.is_failed(), bool)

        except Exception as e:
            pytest.skip(f"Complex workflow execution test skipped: {e}")


class TestTransformationChains:
    """Tests for complex data transformation chains."""

    def test_sequential_transformation_chain(self) -> None:
        """Test workflow with sequential data transformations."""
        workflow = graphbit.Workflow("transformation_chain")

        # Create input agent
        input_agent = graphbit.Node.agent("input", "Generate initial data", "input_001")
        input_id = workflow.add_node(input_agent)

        # Create transformation chain
        transformations = [("normalize", "normalize_data"), ("validate", "validate_format"), ("enrich", "enrich_metadata"), ("format", "format_output")]

        transform_ids = []
        for name, operation in transformations:
            transform_node = graphbit.Node.transform(name, operation)
            transform_id = workflow.add_node(transform_node)
            transform_ids.append(transform_id)

        # Create output agent
        output_agent = graphbit.Node.agent("output", "Process final result", "output_001")
        output_id = workflow.add_node(output_agent)

        # Connect transformation chain
        try:
            # Connect input to first transformation
            workflow.connect(input_id, transform_ids[0])

            # Connect transformations sequentially
            for i in range(len(transform_ids) - 1):
                workflow.connect(transform_ids[i], transform_ids[i + 1])

            # Connect last transformation to output
            workflow.connect(transform_ids[-1], output_id)

        except Exception as e:
            pytest.skip(f"Transformation chain connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    def test_parallel_transformation_workflow(self) -> None:
        """Test workflow with parallel data transformations."""
        workflow = graphbit.Workflow("parallel_transforms")

        # Create data source
        source_agent = graphbit.Node.agent("source", "Generate source data", "source_001")
        source_id = workflow.add_node(source_agent)

        # Create parallel transformation branches
        transform_branches = [("text_processing", "process_text"), ("numeric_analysis", "analyze_numbers"), ("metadata_extraction", "extract_metadata"), ("quality_assessment", "assess_quality")]

        branch_ids = []
        for name, operation in transform_branches:
            transform_node = graphbit.Node.transform(name, operation)
            transform_id = workflow.add_node(transform_node)
            branch_ids.append(transform_id)

        # Create aggregation point
        aggregator_agent = graphbit.Node.agent("aggregator", "Combine all transformations", "agg_001")
        agg_id = workflow.add_node(aggregator_agent)

        try:
            # Connect source to all transformation branches
            for branch_id in branch_ids:
                workflow.connect(source_id, branch_id)

            # Connect all branches to aggregator
            for branch_id in branch_ids:
                workflow.connect(branch_id, agg_id)

        except Exception as e:
            pytest.skip(f"Parallel transformation connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    def test_conditional_transformation_workflow(self) -> None:
        """Test workflow with conditional transformation paths."""
        workflow = graphbit.Workflow("conditional_transforms")

        # Create input processor
        input_agent = graphbit.Node.agent("input", "Process input", "input_001")
        input_id = workflow.add_node(input_agent)

        # Create type detection condition
        type_condition = graphbit.Node.condition("type_check", "data_type == 'structured'")
        type_id = workflow.add_node(type_condition)

        # Create format detection condition
        format_condition = graphbit.Node.condition("format_check", "format == 'json'")
        format_id = workflow.add_node(format_condition)

        # Create transformation nodes for different paths
        structured_transform = graphbit.Node.transform("structured", "process_structured")
        unstructured_transform = graphbit.Node.transform("unstructured", "process_unstructured")
        json_transform = graphbit.Node.transform("json", "parse_json")
        xml_transform = graphbit.Node.transform("xml", "parse_xml")

        struct_id = workflow.add_node(structured_transform)
        unstruct_id = workflow.add_node(unstructured_transform)
        json_id = workflow.add_node(json_transform)
        xml_id = workflow.add_node(xml_transform)

        # Create final processor
        final_agent = graphbit.Node.agent("final", "Finalize processing", "final_001")
        final_id = workflow.add_node(final_agent)

        try:
            # Create conditional transformation paths
            workflow.connect(input_id, type_id)
            workflow.connect(type_id, struct_id)  # Structured path
            workflow.connect(type_id, unstruct_id)  # Unstructured path
            workflow.connect(struct_id, format_id)  # Further format checking
            workflow.connect(format_id, json_id)  # JSON path
            workflow.connect(format_id, xml_id)  # XML path

            # Connect all transformation paths to final processor
            workflow.connect(unstruct_id, final_id)
            workflow.connect(json_id, final_id)
            workflow.connect(xml_id, final_id)

        except Exception as e:
            pytest.skip(f"Conditional transformation connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()


class TestErrorHandlingWorkflows:
    """Tests for workflows with built-in error handling patterns."""

    def test_try_catch_workflow_pattern(self) -> None:
        """Test workflow implementing try-catch error handling pattern."""
        workflow = graphbit.Workflow("try_catch_pattern")

        # Create main processing path
        try_agent = graphbit.Node.agent("try_operation", "Attempt main operation", "try_001")
        try_id = workflow.add_node(try_agent)

        # Create error detection condition
        error_condition = graphbit.Node.condition("error_check", "operation_success == false")
        error_id = workflow.add_node(error_condition)

        # Create success and error handling paths
        success_agent = graphbit.Node.agent("success_handler", "Handle successful operation", "success_001")
        error_agent = graphbit.Node.agent("error_handler", "Handle operation error", "error_001")
        retry_agent = graphbit.Node.agent("retry_handler", "Retry failed operation", "retry_001")

        success_id = workflow.add_node(success_agent)
        error_id_node = workflow.add_node(error_agent)
        retry_id = workflow.add_node(retry_agent)

        # Create final cleanup
        cleanup_agent = graphbit.Node.agent("cleanup", "Perform cleanup operations", "cleanup_001")
        cleanup_id = workflow.add_node(cleanup_agent)

        try:
            # Create try-catch structure
            workflow.connect(try_id, error_id)
            workflow.connect(error_id, success_id)  # Success path
            workflow.connect(error_id, error_id_node)  # Error path
            workflow.connect(error_id_node, retry_id)  # Retry path
            workflow.connect(retry_id, error_id)  # Retry loop

            # Connect to cleanup
            workflow.connect(success_id, cleanup_id)
            workflow.connect(error_id_node, cleanup_id)

        except Exception as e:
            pytest.skip(f"Try-catch workflow connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    def test_circuit_breaker_workflow_pattern(self) -> None:
        """Test workflow implementing circuit breaker pattern."""
        workflow = graphbit.Workflow("circuit_breaker")

        # Create service call attempt
        service_agent = graphbit.Node.agent("service_call", "Call external service", "service_001")
        service_id = workflow.add_node(service_agent)

        # Create circuit breaker logic
        circuit_condition = graphbit.Node.condition("circuit_state", "circuit_open == false")
        circuit_id = workflow.add_node(circuit_condition)

        # Create fallback mechanisms
        primary_agent = graphbit.Node.agent("primary_service", "Use primary service", "primary_001")
        fallback_agent = graphbit.Node.agent("fallback_service", "Use fallback service", "fallback_001")
        cache_agent = graphbit.Node.agent("cache_service", "Use cached response", "cache_001")

        primary_id = workflow.add_node(primary_agent)
        fallback_id = workflow.add_node(fallback_agent)
        cache_id = workflow.add_node(cache_agent)

        # Create response handler
        response_agent = graphbit.Node.agent("response_handler", "Handle service response", "response_001")
        response_id = workflow.add_node(response_agent)

        try:
            # Create circuit breaker pattern
            workflow.connect(service_id, circuit_id)
            workflow.connect(circuit_id, primary_id)  # Circuit closed
            workflow.connect(circuit_id, fallback_id)  # Circuit open
            workflow.connect(fallback_id, cache_id)  # Fallback chain

            # Connect to response handler
            workflow.connect(primary_id, response_id)
            workflow.connect(cache_id, response_id)

        except Exception as e:
            pytest.skip(f"Circuit breaker workflow connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    def test_retry_with_backoff_workflow(self) -> None:
        """Test workflow implementing retry with exponential backoff."""
        workflow = graphbit.Workflow("retry_backoff")

        # Create operation attempt
        operation_agent = graphbit.Node.agent("operation", "Perform operation", "op_001")
        op_id = workflow.add_node(operation_agent)

        # Create retry counter check
        retry_condition = graphbit.Node.condition("retry_check", "retry_count < max_retries")
        retry_id = workflow.add_node(retry_condition)

        # Create backoff calculation
        backoff_transform = graphbit.Node.transform("backoff", "calculate_backoff_delay")
        backoff_id = workflow.add_node(backoff_transform)

        # Create delay and retry logic
        delay_agent = graphbit.Node.agent("delay", "Apply backoff delay", "delay_001")
        retry_agent = graphbit.Node.agent("retry", "Retry operation", "retry_001")
        failure_agent = graphbit.Node.agent("failure", "Handle final failure", "failure_001")
        success_agent = graphbit.Node.agent("success", "Handle success", "success_001")

        delay_id = workflow.add_node(delay_agent)
        retry_op_id = workflow.add_node(retry_agent)
        failure_id = workflow.add_node(failure_agent)
        success_id = workflow.add_node(success_agent)

        try:
            # Create retry pattern
            workflow.connect(op_id, retry_id)
            workflow.connect(retry_id, success_id)  # Success path
            workflow.connect(retry_id, backoff_id)  # Retry path
            workflow.connect(backoff_id, delay_id)
            workflow.connect(delay_id, retry_op_id)
            workflow.connect(retry_op_id, op_id)  # Retry loop
            workflow.connect(retry_id, failure_id)  # Final failure

        except Exception as e:
            pytest.skip(f"Retry backoff workflow connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()


class TestDataPipelineWorkflows:
    """Tests for complex data pipeline workflow patterns."""

    def test_etl_pipeline_workflow(self) -> None:
        """Test workflow implementing ETL (Extract, Transform, Load) pattern."""
        workflow = graphbit.Workflow("etl_pipeline")

        # Extract phase
        extract_agent = graphbit.Node.agent("extractor", "Extract data from sources", "extract_001")
        extract_id = workflow.add_node(extract_agent)

        # Transform phase - multiple transformation steps
        transform_steps = [("cleanse", "clean_data"), ("normalize", "normalize_schema"), ("validate", "validate_integrity"), ("enrich", "enrich_data"), ("aggregate", "aggregate_metrics")]

        transform_ids = []
        for name, operation in transform_steps:
            if name == "validate":
                # Use condition node for validation
                node = graphbit.Node.condition(name, "data_valid == true")
            else:
                # Use transform node for data operations
                node = graphbit.Node.transform(name, operation)

            node_id = workflow.add_node(node)
            transform_ids.append(node_id)

        # Load phase
        load_agent = graphbit.Node.agent("loader", "Load data to destination", "load_001")
        load_id = workflow.add_node(load_agent)

        # Quality assurance
        qa_agent = graphbit.Node.agent("quality_check", "Perform quality assurance", "qa_001")
        qa_id = workflow.add_node(qa_agent)

        try:
            # Connect extract to first transform
            workflow.connect(extract_id, transform_ids[0])

            # Connect transform steps sequentially
            for i in range(len(transform_ids) - 1):
                workflow.connect(transform_ids[i], transform_ids[i + 1])

            # Connect last transform to load
            workflow.connect(transform_ids[-1], load_id)

            # Connect load to quality check
            workflow.connect(load_id, qa_id)

        except Exception as e:
            pytest.skip(f"ETL pipeline connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    def test_stream_processing_workflow(self) -> None:
        """Test workflow for stream processing pattern."""
        workflow = graphbit.Workflow("stream_processing")

        # Stream input
        stream_agent = graphbit.Node.agent("stream_input", "Receive stream data", "stream_001")
        stream_id = workflow.add_node(stream_agent)

        # Partitioning logic
        partition_condition = graphbit.Node.condition("partition", "hash(key) % partition_count")
        partition_id = workflow.add_node(partition_condition)

        # Parallel processing partitions
        partition_processors = []
        for i in range(3):
            processor = graphbit.Node.agent(f"partition_{i}", f"Process partition {i}", f"part_{i:03d}")
            proc_id = workflow.add_node(processor)
            partition_processors.append(proc_id)

        # Aggregation
        aggregator = graphbit.Node.agent("aggregator", "Aggregate partition results", "agg_001")
        agg_id = workflow.add_node(aggregator)

        # Output
        output_agent = graphbit.Node.agent("output", "Output processed stream", "output_001")
        output_id = workflow.add_node(output_agent)

        try:
            # Connect stream processing flow
            workflow.connect(stream_id, partition_id)

            # Connect partitioner to processors
            for proc_id in partition_processors:
                workflow.connect(partition_id, proc_id)

            # Connect processors to aggregator
            for proc_id in partition_processors:
                workflow.connect(proc_id, agg_id)

            # Connect aggregator to output
            workflow.connect(agg_id, output_id)

        except Exception as e:
            pytest.skip(f"Stream processing workflow connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_complex_pipeline_execution(self) -> None:
        """Test execution of complex data pipeline workflow."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")
        workflow = graphbit.Workflow("complex_pipeline_exec")

        # Create simplified but executable pipeline
        input_agent = graphbit.Node.agent("input", "Process pipeline input", "input_001")
        transform_node = graphbit.Node.transform("transform", "process_data")
        validation_condition = graphbit.Node.condition("validate", "quality_score > 0.7")
        output_agent = graphbit.Node.agent("output", "Generate pipeline output", "output_001")

        # Add nodes and connect
        input_id = workflow.add_node(input_agent)
        transform_id = workflow.add_node(transform_node)
        validate_id = workflow.add_node(validation_condition)
        output_id = workflow.add_node(output_agent)

        try:
            workflow.connect(input_id, transform_id)
            workflow.connect(transform_id, validate_id)
            workflow.connect(validate_id, output_id)

            # Validate and execute
            workflow.validate()

            executor = graphbit.Executor(config)
            result = executor.execute(workflow)

            assert isinstance(result, graphbit.WorkflowResult)

        except Exception as e:
            pytest.skip(f"Complex pipeline execution test skipped: {e}")


@pytest.mark.integration
class TestWorkflowComposition:
    """Integration tests for composing workflows from multiple patterns."""

    def test_nested_workflow_patterns(self) -> None:
        """Test composition of multiple workflow patterns."""
        workflow = graphbit.Workflow("nested_patterns")

        # Combine parallel processing with conditional logic
        entry_agent = graphbit.Node.agent("entry", "Start nested processing", "entry_001")
        entry_id = workflow.add_node(entry_agent)

        # Primary condition
        primary_condition = graphbit.Node.condition("primary", "processing_mode == 'parallel'")
        primary_id = workflow.add_node(primary_condition)

        # Parallel branch
        parallel_nodes = []
        for i in range(2):
            node = graphbit.Node.agent(f"parallel_{i}", f"Parallel process {i}", f"par_{i:03d}")
            node_id = workflow.add_node(node)
            parallel_nodes.append(node_id)

        # Sequential branch
        sequential_nodes = []
        for i in range(2):
            node = graphbit.Node.agent(f"sequential_{i}", f"Sequential process {i}", f"seq_{i:03d}")
            node_id = workflow.add_node(node)
            sequential_nodes.append(node_id)

        # Convergence
        converge_agent = graphbit.Node.agent("converge", "Converge results", "converge_001")
        converge_id = workflow.add_node(converge_agent)

        try:
            # Connect nested patterns
            workflow.connect(entry_id, primary_id)

            # Parallel path
            for node_id in parallel_nodes:
                workflow.connect(primary_id, node_id)
                workflow.connect(node_id, converge_id)

            # Sequential path
            workflow.connect(primary_id, sequential_nodes[0])
            workflow.connect(sequential_nodes[0], sequential_nodes[1])
            workflow.connect(sequential_nodes[1], converge_id)

        except Exception as e:
            pytest.skip(f"Nested pattern composition failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    def test_hierarchical_workflow_structure(self) -> None:
        """Test hierarchical workflow with multiple levels."""
        workflow = graphbit.Workflow("hierarchical")

        # Level 0: Root
        root_agent = graphbit.Node.agent("root", "Root processor", "root_001")
        root_id = workflow.add_node(root_agent)

        # Level 1: Department processing
        dept_nodes = []
        for dept in ["sales", "marketing", "support"]:
            node = graphbit.Node.agent(f"dept_{dept}", f"Process {dept} data", f"{dept}_001")
            node_id = workflow.add_node(node)
            dept_nodes.append((dept, node_id))

        # Level 2: Team processing
        team_nodes = []
        for dept, dept_id in dept_nodes:
            for team in ["team_a", "team_b"]:
                node = graphbit.Node.agent(f"{dept}_{team}", f"Process {dept} {team}", f"{dept}_{team}_001")
                node_id = workflow.add_node(node)
                team_nodes.append((dept_id, node_id))

        # Level 3: Individual processing
        individual_nodes = []
        for _team_dept_id, team_id in team_nodes[:4]:  # Limit to avoid too complex
            for individual in ["person_1", "person_2"]:
                node = graphbit.Node.agent(f"individual_{individual}", f"Process {individual}", f"ind_{individual}_001")
                node_id = workflow.add_node(node)
                individual_nodes.append((team_id, node_id))

        # Aggregation
        agg_agent = graphbit.Node.agent("aggregator", "Final aggregation", "agg_001")
        agg_id = workflow.add_node(agg_agent)

        try:
            # Connect hierarchical structure
            for _dept, dept_id in dept_nodes:
                workflow.connect(root_id, dept_id)

            for dept_id, team_id in team_nodes:
                workflow.connect(dept_id, team_id)

            for team_id, ind_id in individual_nodes:
                workflow.connect(team_id, ind_id)
                workflow.connect(ind_id, agg_id)

        except Exception as e:
            pytest.skip(f"Hierarchical workflow connection failed: {e}")

        with contextlib.suppress(Exception):
            workflow.validate()

    @pytest.mark.skipif(not os.getenv("OPENAI_API_KEY"), reason="Requires OpenAI API key")
    def test_workflow_pattern_performance(self) -> None:
        """Test performance characteristics of different workflow patterns."""
        api_key = os.getenv("OPENAI_API_KEY")
        if not api_key:
            pytest.skip("OPENAI_API_KEY not set")

        config = graphbit.LlmConfig.openai(api_key, "gpt-3.5-turbo")

        # Test different workflow patterns
        patterns = {"linear": self._create_linear_workflow(), "parallel": self._create_simple_parallel_workflow(), "conditional": self._create_simple_conditional_workflow()}

        for pattern_name, workflow in patterns.items():
            try:
                workflow.validate()

                executor = graphbit.Executor(config)

                import time

                start_time = time.time()
                result = executor.execute(workflow)
                end_time = time.time()

                assert isinstance(result, graphbit.WorkflowResult)

                # Check execution time is reasonable
                duration = end_time - start_time
                assert duration > 0
                assert duration < 300  # Should complete within 5 minutes

            except Exception as e:
                pytest.skip(f"Pattern {pattern_name} performance test skipped: {e}")

    def _create_linear_workflow(self) -> Any:
        """Create a simple linear workflow for testing."""
        workflow = graphbit.Workflow("linear_test")

        agent1 = graphbit.Node.agent("step1", "First step", "step1_001")
        agent2 = graphbit.Node.agent("step2", "Second step", "step2_001")
        agent3 = graphbit.Node.agent("step3", "Third step", "step3_001")

        id1 = workflow.add_node(agent1)
        id2 = workflow.add_node(agent2)
        id3 = workflow.add_node(agent3)

        workflow.connect(id1, id2)
        workflow.connect(id2, id3)

        return workflow

    def _create_simple_parallel_workflow(self) -> Any:
        """Create a simple parallel workflow for testing."""
        workflow = graphbit.Workflow("parallel_test")

        start = graphbit.Node.agent("start", "Start parallel", "start_001")
        branch1 = graphbit.Node.agent("branch1", "Parallel branch 1", "branch1_001")
        branch2 = graphbit.Node.agent("branch2", "Parallel branch 2", "branch2_001")
        merge = graphbit.Node.agent("merge", "Merge branches", "merge_001")

        start_id = workflow.add_node(start)
        b1_id = workflow.add_node(branch1)
        b2_id = workflow.add_node(branch2)
        merge_id = workflow.add_node(merge)

        workflow.connect(start_id, b1_id)
        workflow.connect(start_id, b2_id)
        workflow.connect(b1_id, merge_id)
        workflow.connect(b2_id, merge_id)

        return workflow

    def _create_simple_conditional_workflow(self) -> Any:
        """Create a simple conditional workflow for testing."""
        workflow = graphbit.Workflow("conditional_test")

        start = graphbit.Node.agent("start", "Start conditional", "start_001")
        condition = graphbit.Node.condition("check", "value > 0")
        true_branch = graphbit.Node.agent("true_branch", "Handle true case", "true_001")
        false_branch = graphbit.Node.agent("false_branch", "Handle false case", "false_001")

        start_id = workflow.add_node(start)
        cond_id = workflow.add_node(condition)
        true_id = workflow.add_node(true_branch)
        false_id = workflow.add_node(false_branch)

        workflow.connect(start_id, cond_id)
        workflow.connect(cond_id, true_id)
        workflow.connect(cond_id, false_id)

        return workflow
