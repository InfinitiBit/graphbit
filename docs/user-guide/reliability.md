# Reliability and Fault Tolerance

GraphBit provides comprehensive reliability features to ensure robust production workflows. This guide covers error handling, fault tolerance, recovery strategies, and building resilient workflow systems.

## Overview

Reliability in GraphBit encompasses:
- **Error Handling**: Graceful handling of failures and exceptions
- **Fault Tolerance**: Continuing operation despite component failures
- **Recovery Strategies**: Automatic and manual recovery mechanisms
- **Circuit Breakers**: Preventing cascading failures
- **Retry Logic**: Intelligent retry patterns for transient failures
- **Health Monitoring**: Continuous system health assessment

## Error Handling Patterns

### Basic Error Handling

```python
from graphbit import Workflow, Node
import time


def safe_workflow_execution(workflow, executor, max_retries=3):
    """Execute workflow with comprehensive error handling."""
    
    for attempt in range(max_retries + 1):
        try:
            print(f"Execution attempt {attempt + 1}/{max_retries + 1}")
            
            result = executor.execute(workflow)
            
            if result.is_success():
                print("✅ Workflow executed successfully")
                return result
            else:
                error_msg = result.error()
                print(f"❌ Workflow failed: {error_msg}")
                
                if attempt < max_retries:
                    wait_time = 2 ** attempt  # Exponential backoff
                    print(f"⏳ Retrying in {wait_time} seconds...")
                    time.sleep(wait_time)
                else:
                    print("❌ Max retries exceeded")
                    return result
                    
        except Exception as e:
            print(f"❌ Execution exception: {e}")
            
            if attempt < max_retries:
                wait_time = 2 ** attempt
                print(f"⏳ Retrying in {wait_time} seconds...")
                time.sleep(wait_time)
            else:
                print("❌ Max retries exceeded")
                raise e
    
    return None

def create_fault_tolerant_workflow():
    """Create workflow with built-in fault tolerance."""
    
    workflow = Workflow("Fault Tolerant Workflow")
    
    # Input validator with error handling
    validator = Node.agent(
        name="Input Validator",
        prompt=f"""
        Validate this input and handle any issues gracefully:
        
        Input: {input}
        
        If the input is invalid:
        1. Identify the specific issues
        2. Suggest corrections if possible
        3. Return a status indicating validation result
        
        If valid, return the input with validation confirmation.
        """,
        agent_id="validator"
    )
    
    # Robust processor with fallback logic
    processor = Node.agent(
        name="Robust Processor",
        prompt="""
        Process the validated input with error resilience:
        
        If processing encounters issues:
        1. Try alternative processing methods
        2. Provide partial results if possible
        3. Report any limitations or warnings
        
        Always return some form of useful output.
        """,
        agent_id="processor"
    )
    
    # Error recovery node
    recovery_handler = Node.agent(
        name="Recovery Handler",
        prompt="""
        Handle any errors or partial results in the input:
        
        If there are errors or incomplete results:
        1. Attempt data recovery
        2. Fill in missing information where possible
        3. Flag areas that need manual review
        """,
        agent_id="recovery_handler"
    )
    
    # Build fault-tolerant chain
    validator_id = workflow.add_node(validator)
    processor_id = workflow.add_node(processor)
    recovery_id = workflow.add_node(recovery_handler)
    
    workflow.connect(validator_id, processor_id)
    workflow.connect(processor_id, recovery_id)
    
    return workflow
```

## Circuit Breaker Pattern

### Implementing Circuit Breakers

```python
from enum import Enum
from datetime import datetime
from graphbit import LlmConfig, Executor
import os
import time

class CircuitState(Enum):
    CLOSED = "closed"
    OPEN = "open"
    HALF_OPEN = "half_open"

class CircuitBreaker:
    """Circuit breaker for preventing cascading failures."""
    
    def __init__(self, failure_threshold=5, recovery_timeout=60, timeout_seconds=30):
        self.failure_threshold = failure_threshold
        self.recovery_timeout = recovery_timeout  # seconds
        self.timeout_seconds = timeout_seconds
        
        self.failure_count = 0
        self.last_failure_time = None
        self.state = CircuitState.CLOSED
        
    def can_execute(self):
        """Check if execution is allowed."""
        
        if self.state == CircuitState.CLOSED:
            return True
        elif self.state == CircuitState.OPEN:
            if self.last_failure_time and \
               (datetime.now() - self.last_failure_time).seconds >= self.recovery_timeout:
                self.state = CircuitState.HALF_OPEN
                return True
            return False
        elif self.state == CircuitState.HALF_OPEN:
            return True
        
        return False
    
    def record_success(self):
        """Record successful execution."""
        self.failure_count = 0
        self.state = CircuitState.CLOSED
        
    def record_failure(self):
        """Record failed execution."""
        self.failure_count += 1
        self.last_failure_time = datetime.now()
        
        if self.failure_count >= self.failure_threshold:
            self.state = CircuitState.OPEN
    
    def get_state(self):
        """Get current circuit breaker state."""
        return self.state

class ReliableExecutor:
    """Executor with circuit breaker protection."""
    
    def __init__(self, base_executor, circuit_breaker=None):
        self.base_executor = base_executor
        self.circuit_breaker = circuit_breaker or CircuitBreaker()
        
    def execute(self, workflow):
        """Execute workflow with circuit breaker protection."""
        
        if not self.circuit_breaker.can_execute():
            raise Exception(f"Circuit breaker is {self.circuit_breaker.get_state().value}")
        
        try:
            start_time = time.time()
            result = self.base_executor.execute(workflow)
            duration = time.time() - start_time
            
            if result.is_success():
                self.circuit_breaker.record_success()
                return result
            else:
                self.circuit_breaker.record_failure()
                return result
                
        except Exception as e:
            self.circuit_breaker.record_failure()
            raise e

def create_circuit_breaker_executor():
    """Create executor with circuit breaker protection."""
    
    # Base executor
    llm_config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    base_executor = Executor(llm_config)
    
    # Circuit breaker configuration
    circuit_breaker = CircuitBreaker(
        failure_threshold=3,  # Open after 3 failures
        recovery_timeout=30,  # Try again after 30 seconds
        timeout_seconds=60    # Individual execution timeout
    )
    
    reliable_executor = ReliableExecutor(base_executor, circuit_breaker)
    
    return reliable_executor
```

## Retry Strategies

### Advanced Retry Logic

```python
import random
import time
from typing import Callable, Optional

class RetryStrategy:
    """Base class for retry strategies."""
    
    def get_wait_time(self, attempt: int) -> float:
        """Get wait time for given attempt number."""
        raise NotImplementedError

class ExponentialBackoff(RetryStrategy):
    """Exponential backoff retry strategy."""
    
    def __init__(self, base_delay=1.0, max_delay=60.0, multiplier=2.0):
        self.base_delay = base_delay
        self.max_delay = max_delay
        self.multiplier = multiplier
    
    def get_wait_time(self, attempt: int) -> float:
        delay = self.base_delay * (self.multiplier ** attempt)
        return min(delay, self.max_delay)

class JitteredBackoff(RetryStrategy):
    """Exponential backoff with jitter to avoid thundering herd."""
    
    def __init__(self, base_delay=1.0, max_delay=60.0, multiplier=2.0, jitter_ratio=0.1):
        self.base_delay = base_delay
        self.max_delay = max_delay
        self.multiplier = multiplier
        self.jitter_ratio = jitter_ratio
    
    def get_wait_time(self, attempt: int) -> float:
        delay = self.base_delay * (self.multiplier ** attempt)
        
        # Add jitter
        jitter = delay * self.jitter_ratio * random.random()
        delay += jitter
        
        return min(delay, self.max_delay)

class RetryableExecutor:
    """Executor with configurable retry strategies."""
    
    def __init__(self, base_executor, retry_strategy=None, max_retries=3):
        self.base_executor = base_executor
        self.retry_strategy = retry_strategy or ExponentialBackoff()
        self.max_retries = max_retries
        
    def execute(self, workflow, retry_condition: Optional[Callable] = None):
        """Execute workflow with retry logic."""
        
        last_exception = None
        
        for attempt in range(self.max_retries + 1):
            try:
                result = self.base_executor.execute(workflow)
                
                if result.is_success():
                    if attempt > 0:
                        print(f"✅ Workflow succeeded on attempt {attempt + 1}")
                    return result
                else:
                    # Check if this failure should trigger a retry
                    if retry_condition and not retry_condition(result):
                        print(f"❌ Non-retryable failure: {result.error()}")
                        return result
                    
                    if attempt < self.max_retries:
                        wait_time = self.retry_strategy.get_wait_time(attempt)
                        print(f"⏳ Workflow failed, retrying in {wait_time:.1f}s (attempt {attempt + 1})")
                        time.sleep(wait_time)
                    else:
                        print(f"❌ Workflow failed after {self.max_retries + 1} attempts")
                        return result
                        
            except Exception as e:
                last_exception = e
                
                if attempt < self.max_retries:
                    wait_time = self.retry_strategy.get_wait_time(attempt)
                    print(f"⏳ Exception occurred, retrying in {wait_time:.1f}s: {e}")
                    time.sleep(wait_time)
                else:
                    print(f"❌ Exception after {self.max_retries + 1} attempts: {e}")
                    raise e
        
        if last_exception:
            raise last_exception
```

## Health Monitoring and Recovery

### Health Check System

```python
from typing import Callable
from graphbit import get_system_info, health_check, LlmConfig, LlmClient
from datetime import datetime
import time
import os

class HealthChecker:
    """Comprehensive health monitoring system."""
    
    def __init__(self):
        self.health_checks = {}
        self.health_history = []
        
    def register_health_check(self, name: str, check_func: Callable, critical: bool = True):
        """Register a health check function."""
        self.health_checks[name] = {
            "func": check_func,
            "critical": critical,
            "last_result": None,
            "last_check": None
        }
    
    def run_health_checks(self):
        """Run all registered health checks."""
        
        results = {}
        overall_healthy = True
        
        for name, check_config in self.health_checks.items():
            try:
                start_time = time.time()
                result = check_config["func"]()
                duration = (time.time() - start_time) * 1000
                
                check_result = {
                    "healthy": bool(result),
                    "duration_ms": duration,
                    "timestamp": datetime.now(),
                    "details": result if isinstance(result, dict) else {}
                }
                
                results[name] = check_result
                check_config["last_result"] = check_result
                check_config["last_check"] = datetime.now()
                
                # Update overall health
                if check_config["critical"] and not check_result["healthy"]:
                    overall_healthy = False
                    
            except Exception as e:
                check_result = {
                    "healthy": False,
                    "error": str(e),
                    "timestamp": datetime.now()
                }
                
                results[name] = check_result
                check_config["last_result"] = check_result
                check_config["last_check"] = datetime.now()
                
                if check_config["critical"]:
                    overall_healthy = False
        
        health_report = {
            "overall_healthy": overall_healthy,
            "timestamp": datetime.now(),
            "checks": results
        }
        
        self.health_history.append(health_report)
        
        # Keep only last 100 health checks
        if len(self.health_history) > 100:
            self.health_history = self.health_history[-100:]
        
        return health_report

def create_health_checks():
    """Create standard health checks for GraphBit."""
    
    def check_graphbit_system():
        """Check GraphBit system health."""
        try:
            system_info = get_system_info()
            health_check_info = health_check()
            
            return {
                "system_available": True,
                "runtime_initialized": system_info.get("runtime_initialized", False),
                "health_check": health_check_info
            }
        except Exception as e:
            return False
    
    def check_llm_connectivity():
        """Check LLM provider connectivity."""
        try:
            config = LlmConfig.openai(
                api_key=os.getenv("OPENAI_API_KEY"),
                model="gpt-3.5-turbo"
            )
            
            client = LlmClient(config)
            
            # Simple test completion
            response = client.complete("Test")
            
            return {
                "provider_accessible": True,
                "response_received": len(response) > 0
            }
        except Exception as e:
            return False
    
    # Create health checker and register checks
    health_checker = HealthChecker()
    
    health_checker.register_health_check("graphbit_system", check_graphbit_system, critical=True)
    health_checker.register_health_check("llm_connectivity", check_llm_connectivity, critical=True)
    
    return health_checker
```

## Fallback and Degradation Strategies

### Graceful Degradation

```python
from graphbit import Workflow, Node

class FallbackWorkflow:
    """Workflow with multiple fallback levels."""
    
    def __init__(self, name):
        self.name = name
        self.primary_workflow = None
        self.fallback_workflows = []
        
    def set_primary_workflow(self, workflow):
        """Set the primary workflow."""
        self.primary_workflow = workflow
    
    def add_fallback_workflow(self, workflow, priority=1):
        """Add a fallback workflow with priority."""
        self.fallback_workflows.append({
            "workflow": workflow,
            "priority": priority
        })
        
        # Sort by priority (lower numbers = higher priority)
        self.fallback_workflows.sort(key=lambda x: x["priority"])
    
    def execute(self, executor):
        """Execute workflow with fallback chain."""
        
        # Try primary workflow first
        if self.primary_workflow:
            try:
                print("🚀 Attempting primary workflow...")
                result = executor.execute(self.primary_workflow)
                
                if result.is_success():
                    print("✅ Primary workflow succeeded")
                    return result
                else:
                    print(f"❌ Primary workflow failed: {result.error()}")
                    
            except Exception as e:
                print(f"❌ Primary workflow exception: {e}")
        
        # Try fallback workflows in priority order
        for i, fallback in enumerate(self.fallback_workflows):
            try:
                print(f"🔄 Attempting fallback {i+1}...")
                result = executor.execute(fallback["workflow"])
                
                if result.is_success():
                    print(f"✅ Fallback {i+1} succeeded")
                    return result
                else:
                    print(f"❌ Fallback {i+1} failed: {result.error()}")
                    
            except Exception as e:
                print(f"❌ Fallback {i+1} exception: {e}")
        
        print("❌ All workflows failed")
        return None

def create_degraded_processing_workflows():
    """Create workflows with different levels of processing."""
    
    # High-quality processing (primary)
    primary_workflow = Workflow("High Quality Processing")
    
    primary_processor = Node.agent(
        name="Advanced Processor",
        prompt=f"""
        Perform comprehensive analysis of this input:
        
        Input: {input}
        
        Provide:
        1. Detailed analysis
        2. Multiple perspectives
        3. Confidence scores
        4. Recommendations
        5. Risk assessment
        """,
        agent_id="advanced_processor"
    )
    
    primary_workflow.add_node(primary_processor)
    
    # Basic processing (fallback)
    fallback_workflow = Workflow("Basic Processing")
    
    basic_processor = Node.agent(
        name="Basic Processor",
        prompt=f"Provide a simple summary of: {input}",
        agent_id="basic_processor"
    )
    
    fallback_workflow.add_node(basic_processor)
    
    # Emergency processing (last resort)
    emergency_workflow = Workflow("Emergency Processing")
    
    emergency_processor = Node.agent(
        name="Emergency Processor",
        prompt="""
        As an emergency fallback, perform a simple transformation:
        
        Convert the input to UPPERCASE and return the transformed text.
        If conversion is not possible, return the original input and a short note explaining why.
        """,
        agent_id="emergency_processor"
    )
    
    emergency_workflow.add_node(emergency_processor)
    
    # Create fallback workflow
    fallback_system = FallbackWorkflow("Degraded Processing System")
    fallback_system.set_primary_workflow(primary_workflow)
    fallback_system.add_fallback_workflow(fallback_workflow, priority=1)
    fallback_system.add_fallback_workflow(emergency_workflow, priority=2)
    
    return fallback_system
```

## Production Reliability Patterns

### Complete Reliability Stack

```python
from graphbit import Executor, LlmConfig
import os
import time

class ProductionExecutor:
    """Production-ready executor with full reliability stack."""
    
    def __init__(self, llm_config):
        # Base executor
        self.base_executor = Executor(llm_config)
        
        # Reliability components
        self.circuit_breaker = CircuitBreaker(failure_threshold=3, recovery_timeout=30)
        self.retry_strategy = JitteredBackoff(base_delay=1.0, max_delay=30.0)
        self.health_checker = create_health_checks()
        
        # Metrics
        self.execution_count = 0
        self.success_count = 0
        self.failure_count = 0
        
    def execute(self, workflow, execution_id=None):
        """Execute workflow with full reliability features."""
        
        self.execution_count += 1
        
        # Generate execution ID if not provided
        if execution_id is None:
            execution_id = f"prod_exec_{int(time.time())}_{self.execution_count}"
        
        # Health check before execution
        health_report = self.health_checker.run_health_checks()
        if not health_report["overall_healthy"]:
            self.failure_count += 1
            raise Exception("System health check failed")
        
        # Circuit breaker check
        if not self.circuit_breaker.can_execute():
            self.failure_count += 1
            raise Exception(f"Circuit breaker is {self.circuit_breaker.get_state().value}")
        
        # Retry loop
        max_retries = 3
        for attempt in range(max_retries + 1):
            try:
                # Execute workflow
                result = self.base_executor.execute(workflow)
                
                if result.is_success():
                    self.success_count += 1
                    self.circuit_breaker.record_success()
                    return result
                else:
                    self.failure_count += 1
                    self.circuit_breaker.record_failure()
                    
                    if attempt < max_retries:
                        wait_time = self.retry_strategy.get_wait_time(attempt)
                        print(f"⏳ Retrying in {wait_time:.1f}s (attempt {attempt + 1})")
                        time.sleep(wait_time)
                    else:
                        return result
                        
            except Exception as e:
                self.failure_count += 1
                self.circuit_breaker.record_failure()
                
                if attempt < max_retries:
                    wait_time = self.retry_strategy.get_wait_time(attempt)
                    print(f"⏳ Retrying after exception in {wait_time:.1f}s: {e}")
                    time.sleep(wait_time)
                else:
                    raise e
    
    def get_reliability_metrics(self):
        """Get reliability metrics."""
        
        success_rate = (self.success_count / self.execution_count) * 100 if self.execution_count > 0 else 0
        
        return {
            "total_executions": self.execution_count,
            "successful_executions": self.success_count,
            "failed_executions": self.failure_count,
            "success_rate_percent": success_rate,
            "circuit_breaker_state": self.circuit_breaker.get_state().value
        }

def create_production_executor():
    """Create production-ready executor."""
    
    llm_config = LlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-4o-mini"
    )
    
    return ProductionExecutor(llm_config)
```

## Best Practices

### 1. Reliability Design Principles

```python
def get_reliability_best_practices():
    """Get best practices for building reliable workflows."""
    
    best_practices = {
        "fail_fast": "Detect and report failures quickly",
        "graceful_degradation": "Provide reduced functionality when components fail",
        "idempotency": "Ensure operations can be safely repeated",
        "timeout_management": "Set appropriate timeouts for all operations",
        "resource_cleanup": "Always clean up resources, even on failure",
        "monitoring": "Continuously monitor system health and performance",
        "testing": "Test failure scenarios regularly"
    }
    
    for practice, description in best_practices.items():
        print(f"✅ {practice.replace('_', ' ').title()}: {description}")
    
    return best_practices
```

### 2. Error Classification

```python
def classify_error_type(error):
    """Classify errors for appropriate handling."""
    
    error_message = str(error).lower()
    
    # Transient errors - should retry
    if any(keyword in error_message for keyword in [
        "timeout", "network", "connection", "temporary", "rate limit"
    ]):
        return "transient"
    
    # Permanent errors - should not retry
    if any(keyword in error_message for keyword in [
        "authentication", "permission", "not found", "invalid"
    ]):
        return "permanent"
    
    # System errors - may need recovery
    if any(keyword in error_message for keyword in [
        "memory", "disk", "resource", "capacity"
    ]):
        return "system"
    
    # Unknown errors - handle conservatively
    return "unknown"
```

## Usage Examples

### Production Reliability Example

```python
def example_production_usage():
    """Example of production reliability patterns."""
    
    # Create production executor
    prod_executor = create_production_executor()
    
    # Create reliable workflow
    workflow = create_fault_tolerant_workflow()
    
    try:
        # Execute with full reliability features
        result = prod_executor.execute(workflow, execution_id="example_prod_run")
        
        if result.is_success():
            print(f"✅ Production execution successful: {result.get_all_node_outputs()}")
        else:
            print(f"❌ Production execution failed: {result.error()}")
        
        # Print reliability metrics
        metrics = prod_executor.get_reliability_metrics()
        print(f"📊 Reliability Metrics: {metrics}")
        
    except Exception as e:
        print(f"❌ Production execution exception: {e}")

if __name__ == "__main__":
    example_production_usage()
```

## What's Next

- Learn about [Monitoring](monitoring.md) for tracking reliability metrics
- Explore [Performance](performance.md) optimization for reliable systems  
- Check [Validation](validation.md) for ensuring system correctness
- See [LLM Providers](llm-providers.md) for provider-specific reliability features 
