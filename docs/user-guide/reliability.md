# Reliability and Fault Tolerance

GraphBit provides robust reliability features including retries, circuit breakers, timeouts, and graceful error handling. This guide covers building resilient workflows that can handle failures, network issues, and service degradation gracefully.

## Overview

GraphBit reliability features include:
- **Retry Mechanisms**: Automatic retry of failed operations with configurable policies
- **Circuit Breakers**: Protection against cascading failures and service overload
- **Timeout Management**: Preventing hung operations and resource exhaustion
- **Graceful Degradation**: Fallback strategies when services are unavailable
- **Error Recovery**: Automatic and manual recovery from various failure modes

## Retry Policies

### Basic Retry Configuration

```python
import graphbit
import time
import os
from typing import Optional, Dict, Any

def create_reliable_workflow_with_retries():
    """Create workflow with comprehensive retry configuration."""
    
    builder = graphbit.PyWorkflowBuilder("Reliable Workflow with Retries")
    
    # Resilient processor with retry logic
    processor = graphbit.PyWorkflowNode.agent_node(
        name="Resilient Processor",
        description="Processes data with built-in retry mechanisms",
        agent_id="resilient_processor",
        prompt="""
        Process this data with reliability in mind:
        
        Input: {input}
        Retry Context: {retry_context}
        
        Handle processing with:
        1. Graceful error handling
        2. Partial result preservation
        3. State recovery capabilities
        4. Quality validation
        5. Fallback strategies
        
        If this is a retry attempt, consider previous failures and adapt approach.
        Return robust processing results.
        """
    )
    
    # Validation node
    validator = graphbit.PyWorkflowNode.agent_node(
        name="Result Validator",
        description="Validates processing results for quality",
        agent_id="validator",
        prompt="""
        Validate processing results for reliability:
        
        Results: {results}
        Quality Standards: {quality_standards}
        
        Check for:
        1. Completeness of results
        2. Data integrity and consistency
        3. Expected format compliance
        4. Business rule adherence
        5. Performance criteria
        
        Return validation status and quality metrics.
        """
    )
    
    # Build reliable workflow
    proc_id = builder.add_node(processor)
    val_id = builder.add_node(validator)
    
    builder.connect(proc_id, val_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()

def configure_retry_policies():
    """Configure various retry policies for different scenarios."""
    
    # Basic exponential backoff retry
    basic_retry = {
        "max_attempts": 3,
        "initial_delay": 1000,  # 1 second
        "multiplier": 2.0,      # Double delay each time
        "max_delay": 30000      # Max 30 seconds
    }
    
    # Aggressive retry for critical operations
    aggressive_retry = {
        "max_attempts": 5,
        "initial_delay": 500,   # 0.5 seconds
        "multiplier": 1.5,      # Moderate backoff
        "max_delay": 10000      # Max 10 seconds
    }
    
    # Conservative retry for resource-intensive operations
    conservative_retry = {
        "max_attempts": 2,
        "fixed_delay": 5000     # Fixed 5-second delay
    }
    
    return {
        "basic": basic_retry,
        "aggressive": aggressive_retry,
        "conservative": conservative_retry
    }

class RetryableExecutor:
    """Executor wrapper with advanced retry capabilities."""
    
    def __init__(self, base_executor, retry_config=None):
        self.base_executor = base_executor
        self.retry_config = retry_config or {"max_attempts": 3, "initial_delay": 1000, "multiplier": 2.0}
        self.retry_stats = {
            "total_attempts": 0,
            "successful_retries": 0,
            "failed_after_retries": 0,
            "retry_patterns": {}
        }
    
    def execute_with_retries(self, workflow, input_data, context=None):
        """Execute workflow with retry logic."""
        
        max_attempts = self.retry_config.get("max_attempts", 3)
        attempt = 0
        last_error = None
        
        while attempt < max_attempts:
            try:
                self.retry_stats["total_attempts"] += 1
                
                # Add retry context to input
                execution_context = {
                    "attempt": attempt + 1,
                    "max_attempts": max_attempts,
                    "previous_error": str(last_error) if last_error else None,
                    "retry_context": context or {}
                }
                
                # Execute workflow
                result = self.base_executor.execute_with_input(workflow, {
                    **input_data,
                    "retry_context": execution_context
                })
                
                # Success - track retry stats
                if attempt > 0:
                    self.retry_stats["successful_retries"] += 1
                
                return result
                
            except Exception as e:
                last_error = e
                attempt += 1
                
                # Track retry patterns
                error_type = type(e).__name__
                self.retry_stats["retry_patterns"][error_type] = \
                    self.retry_stats["retry_patterns"].get(error_type, 0) + 1
                
                if attempt >= max_attempts:
                    self.retry_stats["failed_after_retries"] += 1
                    raise e
                
                # Calculate delay with backoff
                delay = self._calculate_delay(attempt)
                print(f"Attempt {attempt} failed with {error_type}, retrying in {delay/1000:.1f}s...")
                time.sleep(delay / 1000.0)  # Convert to seconds
        
        # Should not reach here, but just in case
        raise last_error
    
    def _calculate_delay(self, attempt):
        """Calculate retry delay with exponential backoff."""
        
        if "fixed_delay" in self.retry_config:
            return self.retry_config["fixed_delay"]
        
        # Exponential backoff
        initial_delay = self.retry_config.get("initial_delay", 1000)
        multiplier = self.retry_config.get("multiplier", 2.0)
        max_delay = self.retry_config.get("max_delay", 30000)
        
        delay = min(initial_delay * (multiplier ** (attempt - 1)), max_delay)
        
        return int(delay)
    
    def get_retry_stats(self):
        """Get retry statistics."""
        
        total = self.retry_stats["total_attempts"]
        if total == 0:
            return {"message": "No executions attempted"}
        
        return {
            "total_attempts": total,
            "successful_retries": self.retry_stats["successful_retries"],
            "failed_after_retries": self.retry_stats["failed_after_retries"],
            "success_rate": ((total - self.retry_stats["failed_after_retries"]) / total) * 100,
            "retry_patterns": self.retry_stats["retry_patterns"]
        }
```

## Circuit Breakers

### Circuit Breaker Implementation

```python
from enum import Enum
from datetime import datetime, timedelta

class CircuitState(Enum):
    CLOSED = "closed"      # Normal operation
    OPEN = "open"          # Failing, blocking requests
    HALF_OPEN = "half_open"  # Testing if service recovered

class CircuitBreaker:
    """Circuit breaker implementation for GraphBit workflows."""
    
    def __init__(self, failure_threshold=5, timeout_duration=60000, success_threshold=3):
        self.failure_threshold = failure_threshold
        self.timeout_duration = timeout_duration  # milliseconds
        self.success_threshold = success_threshold
        
        # State tracking
        self.state = CircuitState.CLOSED
        self.failure_count = 0
        self.success_count = 0
        self.last_failure_time = None
        self.state_change_time = datetime.now()
        
        # Statistics
        self.stats = {
            "total_requests": 0,
            "successful_requests": 0,
            "failed_requests": 0,
            "circuit_opens": 0,
            "circuit_half_opens": 0,
            "circuit_closes": 0
        }
    
    def can_execute(self):
        """Check if execution is allowed by circuit breaker."""
        
        current_time = datetime.now()
        
        if self.state == CircuitState.OPEN:
            # Check if timeout period has elapsed
            time_diff = (current_time - self.state_change_time).total_seconds() * 1000
            if time_diff >= self.timeout_duration:
                self._transition_to_half_open()
                return True
            return False
        
        return True  # CLOSED or HALF_OPEN states allow execution
    
    def record_success(self):
        """Record successful execution."""
        
        self.stats["total_requests"] += 1
        self.stats["successful_requests"] += 1
        
        if self.state == CircuitState.HALF_OPEN:
            self.success_count += 1
            if self.success_count >= self.success_threshold:
                self._transition_to_closed()
        elif self.state == CircuitState.CLOSED:
            # Reset failure count on success
            self.failure_count = 0
    
    def record_failure(self):
        """Record failed execution."""
        
        self.stats["total_requests"] += 1
        self.stats["failed_requests"] += 1
        self.last_failure_time = datetime.now()
        
        if self.state == CircuitState.CLOSED:
            self.failure_count += 1
            if self.failure_count >= self.failure_threshold:
                self._transition_to_open()
        elif self.state == CircuitState.HALF_OPEN:
            # Single failure in half-open state trips back to open
            self._transition_to_open()
    
    def _transition_to_open(self):
        """Transition circuit breaker to OPEN state."""
        self.state = CircuitState.OPEN
        self.state_change_time = datetime.now()
        self.failure_count = 0
        self.success_count = 0
        self.stats["circuit_opens"] += 1
        print("🔴 Circuit breaker opened - blocking requests")
    
    def _transition_to_half_open(self):
        """Transition circuit breaker to HALF_OPEN state."""
        self.state = CircuitState.HALF_OPEN
        self.state_change_time = datetime.now()
        self.success_count = 0
        self.stats["circuit_half_opens"] += 1
        print("🟡 Circuit breaker half-open - testing service")
    
    def _transition_to_closed(self):
        """Transition circuit breaker to CLOSED state."""
        self.state = CircuitState.CLOSED
        self.state_change_time = datetime.now()
        self.failure_count = 0
        self.success_count = 0
        self.stats["circuit_closes"] += 1
        print("🟢 Circuit breaker closed - normal operation resumed")
    
    def get_state(self):
        """Get current circuit breaker state and statistics."""
        
        return {
            "state": self.state.value,
            "failure_count": self.failure_count,
            "success_count": self.success_count,
            "last_failure_time": self.last_failure_time.isoformat() if self.last_failure_time else None,
            "stats": self.stats.copy()
        }

class CircuitBreakerExecutor:
    """Executor with circuit breaker protection."""
    
    def __init__(self, base_executor, circuit_breaker=None):
        self.base_executor = base_executor
        self.circuit_breaker = circuit_breaker or CircuitBreaker()
    
    def execute_with_circuit_breaker(self, workflow, input_data):
        """Execute workflow with circuit breaker protection."""
        
        # Check if circuit breaker allows execution
        if not self.circuit_breaker.can_execute():
            raise Exception("Circuit breaker is OPEN - execution blocked")
        
        try:
            # Execute workflow
            result = self.base_executor.execute_with_input(workflow, input_data)
            
            # Record success
            self.circuit_breaker.record_success()
            
            return result
            
        except Exception as e:
            # Record failure
            self.circuit_breaker.record_failure()
            raise e
    
    def get_circuit_breaker_state(self):
        """Get circuit breaker state."""
        return self.circuit_breaker.get_state()

def create_circuit_breaker_workflow():
    """Create workflow with circuit breaker patterns."""
    
    builder = graphbit.PyWorkflowBuilder("Circuit Breaker Workflow")
    
    # Service health checker
    health_checker = graphbit.PyWorkflowNode.agent_node(
        name="Service Health Checker",
        description="Checks service health before processing",
        agent_id="health_checker",
        prompt="""
        Check service health and readiness:
        
        Service Status: {service_status}
        Historical Performance: {performance_history}
        
        Evaluate:
        1. Service availability and responsiveness
        2. Current load and capacity
        3. Error rates and patterns
        4. Recovery indicators
        5. Risk assessment
        
        Return health status and recommendations.
        """
    )
    
    # Protected processor
    protected_processor = graphbit.PyWorkflowNode.agent_node(
        name="Protected Processor",
        description="Main processor with circuit breaker protection",
        agent_id="protected_processor",
        prompt="""
        Process data with circuit breaker protection:
        
        Input: {input}
        Health Status: {health_status}
        Circuit State: {circuit_state}
        
        Process considering:
        1. Current system health
        2. Risk mitigation strategies
        3. Graceful degradation options
        4. Partial processing capabilities
        5. Fallback mechanisms
        
        Return processing results with status information.
        """
    )
    
    # Build circuit breaker workflow
    health_id = builder.add_node(health_checker)
    processor_id = builder.add_node(protected_processor)
    
    builder.connect(health_id, processor_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## Timeout Management

### Timeout Configuration and Handling

```python
import threading
from contextlib import contextmanager

class TimeoutManager:
    """Manages timeouts for workflow execution."""
    
    def __init__(self):
        self.active_timeouts = {}
        self.timeout_stats = {
            "total_executions": 0,
            "timeouts_triggered": 0,
            "average_execution_time": 0
        }
    
    @contextmanager
    def timeout_context(self, timeout_seconds, operation_name="operation"):
        """Context manager for timeout handling."""
        
        execution_id = id(threading.current_thread())
        start_time = time.time()
        
        def timeout_handler():
            """Handle timeout expiration."""
            self.timeout_stats["timeouts_triggered"] += 1
            raise TimeoutError(f"Operation '{operation_name}' timed out after {timeout_seconds} seconds")
        
        # Set up timeout
        timer = threading.Timer(timeout_seconds, timeout_handler)
        self.active_timeouts[execution_id] = timer
        timer.start()
        
        try:
            self.timeout_stats["total_executions"] += 1
            yield
            
            # Update average execution time
            execution_time = time.time() - start_time
            total_time = (self.timeout_stats["average_execution_time"] * 
                         (self.timeout_stats["total_executions"] - 1) + execution_time)
            self.timeout_stats["average_execution_time"] = total_time / self.timeout_stats["total_executions"]
            
        finally:
            # Clean up timeout
            if execution_id in self.active_timeouts:
                self.active_timeouts[execution_id].cancel()
                del self.active_timeouts[execution_id]
    
    def get_timeout_stats(self):
        """Get timeout statistics."""
        return self.timeout_stats.copy()

class TimeoutAwareExecutor:
    """Executor with comprehensive timeout management."""
    
    def __init__(self, base_executor, default_timeout=300):  # 5 minutes default
        self.base_executor = base_executor
        self.default_timeout = default_timeout
        self.timeout_manager = TimeoutManager()
    
    def execute_with_timeout(self, workflow, input_data, timeout=None, operation_name=None):
        """Execute workflow with timeout protection."""
        
        effective_timeout = timeout or self.default_timeout
        op_name = operation_name or f"workflow_{getattr(workflow, 'name', 'unknown')}"
        
        with self.timeout_manager.timeout_context(effective_timeout, op_name):
            return self.base_executor.execute_with_input(workflow, input_data)
    
    def get_timeout_stats(self):
        """Get timeout statistics."""
        return self.timeout_manager.get_timeout_stats()

def create_timeout_aware_workflow():
    """Create workflow with timeout awareness."""
    
    builder = graphbit.PyWorkflowBuilder("Timeout Aware Workflow")
    
    # Timeout monitor
    timeout_monitor = graphbit.PyWorkflowNode.agent_node(
        name="Timeout Monitor",
        description="Monitors execution time and manages timeouts",
        agent_id="timeout_monitor",
        prompt="""
        Monitor execution time and manage timeout concerns:
        
        Execution Context: {execution_context}
        Time Constraints: {time_constraints}
        
        Monitor:
        1. Current execution time vs. budget
        2. Remaining processing steps
        3. Resource availability
        4. Performance optimization opportunities
        5. Timeout risk assessment
        
        Return timing analysis and recommendations.
        """
    )
    
    # Adaptive processor
    adaptive_processor = graphbit.PyWorkflowNode.agent_node(
        name="Adaptive Processor",
        description="Adapts processing based on time constraints",
        agent_id="adaptive_processor",
        prompt="""
        Process data adaptively based on time constraints:
        
        Input: {input}
        Time Budget: {time_budget}
        Timing Analysis: {timing_analysis}
        
        Adapt processing:
        1. Prioritize critical operations
        2. Skip non-essential processing if time is short
        3. Use faster algorithms when appropriate
        4. Implement progressive disclosure
        5. Prepare for early termination if needed
        
        Return optimized results within time constraints.
        """
    )
    
    # Build timeout-aware workflow
    monitor_id = builder.add_node(timeout_monitor)
    processor_id = builder.add_node(adaptive_processor)
    
    builder.connect(monitor_id, processor_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## Graceful Degradation

### Fallback Strategies

```python
from typing import List, Callable, Any

class FallbackStrategy:
    """Base class for fallback strategies."""
    
    def __init__(self, name, priority=1):
        self.name = name
        self.priority = priority  # Lower numbers = higher priority
    
    def can_handle(self, error, context):
        """Check if this strategy can handle the given error."""
        raise NotImplementedError
    
    def execute(self, original_input, error, context):
        """Execute the fallback strategy."""
        raise NotImplementedError

class CachedResponseFallback(FallbackStrategy):
    """Fallback strategy using cached responses."""
    
    def __init__(self, cache_store, cache_ttl=3600):
        super().__init__("cached_response", priority=1)
        self.cache_store = cache_store
        self.cache_ttl = cache_ttl
    
    def can_handle(self, error, context):
        """Check if cached response is available."""
        cache_key = self._generate_cache_key(context.get("input", {}))
        return cache_key in self.cache_store
    
    def execute(self, original_input, error, context):
        """Return cached response."""
        cache_key = self._generate_cache_key(original_input)
        cached_response = self.cache_store.get(cache_key)
        
        return {
            "result": cached_response,
            "fallback_used": "cached_response",
            "cache_age": time.time() - cached_response.get("timestamp", 0),
            "warning": "This is a cached response due to service unavailability"
        }
    
    def _generate_cache_key(self, input_data):
        """Generate cache key from input data."""
        import hashlib
        import json
        
        input_str = json.dumps(input_data, sort_keys=True)
        return hashlib.md5(input_str.encode()).hexdigest()

class DefaultResponseFallback(FallbackStrategy):
    """Fallback strategy providing default responses."""
    
    def __init__(self, default_responses):
        super().__init__("default_response", priority=3)
        self.default_responses = default_responses
    
    def can_handle(self, error, context):
        """Can always provide a default response."""
        return True
    
    def execute(self, original_input, error, context):
        """Return appropriate default response."""
        
        input_type = self._classify_input(original_input)
        default_response = self.default_responses.get(input_type, 
                                                     self.default_responses.get("generic"))
        
        return {
            "result": default_response,
            "fallback_used": "default_response", 
            "error_details": str(error),
            "warning": "Default response provided due to processing failure"
        }
    
    def _classify_input(self, input_data):
        """Classify input to determine appropriate default response."""
        if isinstance(input_data, dict):
            if "query" in input_data:
                return "search"
            elif "text" in input_data:
                return "text_processing"
        return "generic"

class FallbackManager:
    """Manages fallback strategies for graceful degradation."""
    
    def __init__(self):
        self.strategies: List[FallbackStrategy] = []
        self.fallback_stats = {
            "total_fallbacks": 0,
            "strategy_usage": {},
            "success_rate": {}
        }
    
    def add_strategy(self, strategy: FallbackStrategy):
        """Add a fallback strategy."""
        self.strategies.append(strategy)
        # Sort by priority (lower numbers first)
        self.strategies.sort(key=lambda s: s.priority)
    
    def execute_fallback(self, original_input, error, context=None):
        """Execute the first applicable fallback strategy."""
        
        context = context or {}
        self.fallback_stats["total_fallbacks"] += 1
        
        for strategy in self.strategies:
            if strategy.can_handle(error, context):
                try:
                    result = strategy.execute(original_input, error, context)
                    
                    # Update statistics
                    strategy_name = strategy.name
                    self.fallback_stats["strategy_usage"][strategy_name] = \
                        self.fallback_stats["strategy_usage"].get(strategy_name, 0) + 1
                    
                    print(f"✅ Fallback strategy '{strategy_name}' succeeded")
                    return result
                    
                except Exception as fallback_error:
                    print(f"❌ Fallback strategy '{strategy.name}' failed: {fallback_error}")
                    continue
        
        # All fallback strategies failed
        raise Exception(f"All fallback strategies failed for error: {error}")
    
    def get_fallback_stats(self):
        """Get fallback usage statistics."""
        return {
            "total_fallbacks": self.fallback_stats["total_fallbacks"],
            "strategy_usage": self.fallback_stats["strategy_usage"]
        }

def create_graceful_degradation_workflow():
    """Create workflow with graceful degradation capabilities."""
    
    builder = graphbit.PyWorkflowBuilder("Graceful Degradation Workflow")
    
    # Service availability checker
    availability_checker = graphbit.PyWorkflowNode.agent_node(
        name="Service Availability Checker",
        description="Checks service availability and quality",
        agent_id="availability_checker",
        prompt="""
        Check service availability and quality:
        
        Service Status: {service_status}
        Performance Metrics: {performance_metrics}
        
        Assess:
        1. Service availability and responsiveness
        2. Current performance levels
        3. Error rates and patterns
        4. Capacity and load levels
        5. Degradation risk factors
        
        Return availability assessment and recommendations.
        """
    )
    
    # Quality gate
    quality_gate = graphbit.PyWorkflowNode.condition_node(
        name="Quality Gate",
        description="Determines if full processing is feasible",
        expression="service_quality >= 'acceptable' && availability >= 90"
    )
    
    # Full processor
    full_processor = graphbit.PyWorkflowNode.agent_node(
        name="Full Processor",
        description="Full-featured processing when service is healthy",
        agent_id="full_processor",
        prompt="""
        Perform full-featured processing:
        
        Input: {input}
        Service Quality: {service_quality}
        
        Process with:
        1. Complete feature set
        2. Highest quality algorithms
        3. Comprehensive analysis
        4. Full validation
        5. Rich output format
        
        Return complete processing results.
        """
    )
    
    # Degraded processor
    degraded_processor = graphbit.PyWorkflowNode.agent_node(
        name="Degraded Processor",
        description="Simplified processing when service is degraded",
        agent_id="degraded_processor",
        prompt="""
        Perform degraded processing with reduced features:
        
        Input: {input}
        Degradation Context: {degradation_context}
        
        Process with:
        1. Essential features only
        2. Faster algorithms
        3. Reduced complexity
        4. Basic validation
        5. Simplified output
        
        Return best-effort results with degradation notice.
        """
    )
    
    # Build graceful degradation workflow
    check_id = builder.add_node(availability_checker)
    gate_id = builder.add_node(quality_gate)
    full_id = builder.add_node(full_processor)
    degraded_id = builder.add_node(degraded_processor)
    
    builder.connect(check_id, gate_id, graphbit.PyWorkflowEdge.data_flow())
    builder.connect(gate_id, full_id, 
                   graphbit.PyWorkflowEdge.conditional("service_quality >= 'acceptable'"))
    builder.connect(gate_id, degraded_id,
                   graphbit.PyWorkflowEdge.conditional("service_quality < 'acceptable'"))
    
    return builder.build()
```

## Comprehensive Reliability Executor

### All-in-One Reliable Executor

```python
class ReliableExecutor:
    """Comprehensive executor with all reliability features."""
    
    def __init__(self, base_executor, config=None):
        self.base_executor = base_executor
        
        # Initialize reliability components
        self.retry_config = config.get("retry") if config else {"max_attempts": 3}
        self.circuit_breaker = CircuitBreaker(
            failure_threshold=config.get("circuit_failure_threshold", 5) if config else 5,
            timeout_duration=config.get("circuit_timeout", 60000) if config else 60000
        )
        self.timeout_manager = TimeoutManager()
        self.fallback_manager = FallbackManager()
        
        # Setup default fallback strategies
        self._setup_default_fallbacks()
        
        # Statistics
        self.execution_stats = {
            "total_executions": 0,
            "successful_executions": 0,
            "failed_executions": 0,
            "fallback_executions": 0,
            "retry_executions": 0,
            "circuit_breaker_blocks": 0,
            "timeout_executions": 0
        }
    
    def _setup_default_fallbacks(self):
        """Setup default fallback strategies."""
        
        # Simple cache for demonstration
        cache_store = {}
        
        # Add fallback strategies in priority order
        self.fallback_manager.add_strategy(
            CachedResponseFallback(cache_store)
        )
        
        self.fallback_manager.add_strategy(
            DefaultResponseFallback({
                "search": {"results": [], "message": "Search temporarily unavailable"},
                "text_processing": {"processed_text": "", "message": "Processing temporarily unavailable"},
                "generic": {"result": None, "message": "Service temporarily unavailable"}
            })
        )
    
    def execute_reliably(self, workflow, input_data, timeout=None, context=None):
        """Execute workflow with all reliability features."""
        
        self.execution_stats["total_executions"] += 1
        operation_name = f"workflow_{getattr(workflow, 'name', 'unknown')}"
        
        try:
            # Check circuit breaker
            if not self.circuit_breaker.can_execute():
                self.execution_stats["circuit_breaker_blocks"] += 1
                return self._execute_fallback(input_data, 
                                            Exception("Circuit breaker is OPEN"), 
                                            context)
            
            # Execute with timeout and retries
            with self.timeout_manager.timeout_context(timeout or 300, operation_name):
                result = self._execute_with_retries(workflow, input_data, context)
                
                self.circuit_breaker.record_success()
                self.execution_stats["successful_executions"] += 1
                
                return result
                
        except TimeoutError as e:
            self.execution_stats["timeout_executions"] += 1
            self.circuit_breaker.record_failure()
            return self._execute_fallback(input_data, e, context)
            
        except Exception as e:
            self.execution_stats["failed_executions"] += 1
            self.circuit_breaker.record_failure()
            return self._execute_fallback(input_data, e, context)
    
    def _execute_with_retries(self, workflow, input_data, context):
        """Execute workflow with retry logic."""
        
        retry_executor = RetryableExecutor(self.base_executor, self.retry_config)
        result = retry_executor.execute_with_retries(workflow, input_data, context)
        
        # Update retry statistics
        retry_stats = retry_executor.get_retry_stats()
        if retry_stats.get("successful_retries", 0) > 0:
            self.execution_stats["retry_executions"] += 1
        
        return result
    
    def _execute_fallback(self, input_data, error, context):
        """Execute fallback strategy."""
        
        try:
            self.execution_stats["fallback_executions"] += 1
            return self.fallback_manager.execute_fallback(input_data, error, context)
        except Exception as fallback_error:
            # Final failure - all reliability mechanisms exhausted
            raise Exception(f"All reliability mechanisms failed. Original error: {error}, Fallback error: {fallback_error}")
    
    def get_reliability_stats(self):
        """Get comprehensive reliability statistics."""
        
        total = self.execution_stats["total_executions"]
        if total == 0:
            return {"message": "No executions performed"}
        
        return {
            "execution_stats": self.execution_stats.copy(),
            "success_rate": (self.execution_stats["successful_executions"] / total) * 100,
            "fallback_rate": (self.execution_stats["fallback_executions"] / total) * 100,
            "circuit_breaker": self.circuit_breaker.get_state(),
            "timeout_stats": self.timeout_manager.get_timeout_stats(),
            "fallback_stats": self.fallback_manager.get_fallback_stats()
        }

def demo_reliable_execution():
    """Demonstrate reliable execution with all features."""
    
    # Create base executor
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-3.5-turbo"
    )
    base_executor = graphbit.PyWorkflowExecutor(config)
    
    # Create reliable executor
    reliable_executor = ReliableExecutor(base_executor)
    
    # Create test workflow
    workflow = create_reliable_workflow_with_retries()
    
    # Test data
    test_inputs = [
        {"input": "Test data 1", "quality_standards": "high"},
        {"input": "Test data 2", "quality_standards": "medium"},
        {"input": "Test data 3", "quality_standards": "high"}
    ]
    
    print("🚀 Starting reliable execution demo...")
    
    for i, input_data in enumerate(test_inputs):
        print(f"\n📊 Executing test {i+1}")
        
        try:
            result = reliable_executor.execute_reliably(
                workflow, 
                input_data,
                timeout=60,  # 1 minute timeout
                context={"test_id": i+1}
            )
            print(f"✅ Execution {i+1} completed successfully")
            
        except Exception as e:
            print(f"❌ Execution {i+1} failed: {e}")
    
    # Print reliability statistics
    stats = reliable_executor.get_reliability_stats()
    print(f"\n📈 Reliability Statistics:")
    print(f"Total Executions: {stats['execution_stats']['total_executions']}")
    print(f"Success Rate: {stats['success_rate']:.1f}%")
    print(f"Fallback Rate: {stats['fallback_rate']:.1f}%")
    print(f"Circuit Breaker State: {stats['circuit_breaker']['state']}")
    
    return reliable_executor
```

## Best Practices

### 1. Reliability Strategy Selection

```python
def choose_reliability_strategy(use_case_requirements):
    """Choose appropriate reliability strategy based on requirements."""
    
    strategies = {
        "high_availability": {
            "retry_attempts": 5,
            "circuit_breaker_threshold": 3,
            "timeout": 30,
            "fallback_required": True
        },
        "cost_optimized": {
            "retry_attempts": 2,
            "circuit_breaker_threshold": 10,
            "timeout": 60,
            "fallback_required": False
        },
        "performance_critical": {
            "retry_attempts": 1,
            "circuit_breaker_threshold": 5,
            "timeout": 10,
            "fallback_required": True
        }
    }
    
    use_case = use_case_requirements.get("type", "cost_optimized")
    return strategies.get(use_case, strategies["cost_optimized"])
```

### 2. Monitoring and Alerting

```python
def setup_reliability_monitoring():
    """Setup monitoring for reliability features."""
    
    monitoring_metrics = {
        "retry_rate": "Percentage of executions requiring retries",
        "circuit_breaker_trips": "Number of circuit breaker activations",
        "timeout_rate": "Percentage of executions timing out",
        "fallback_rate": "Percentage of executions using fallbacks",
        "overall_success_rate": "Overall execution success rate"
    }
    
    alert_thresholds = {
        "retry_rate": 20,  # Alert if >20% require retries
        "timeout_rate": 5,   # Alert if >5% timeout
        "fallback_rate": 10, # Alert if >10% use fallbacks
        "overall_success_rate": 95  # Alert if <95% success
    }
    
    return {
        "metrics": monitoring_metrics,
        "thresholds": alert_thresholds
    }
```

GraphBit's comprehensive reliability features ensure your workflows can handle failures gracefully, maintain high availability, and provide consistent user experiences even under adverse conditions. Use these patterns to build robust, production-ready AI workflows. 
