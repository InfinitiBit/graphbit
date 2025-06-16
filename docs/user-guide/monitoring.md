# Monitoring and Observability

GraphBit provides comprehensive monitoring capabilities to track workflow performance, health, and reliability. This guide covers metrics collection, alerting, logging, and observability best practices.

## Overview

GraphBit monitoring encompasses:
- **Execution Metrics**: Performance, success rates, and timing data
- **System Health**: Resource usage and availability monitoring
- **Error Tracking**: Failure detection and analysis
- **Business Metrics**: Custom metrics for business logic
- **Real-time Dashboards**: Live monitoring and visualization

## Basic Monitoring Setup

### Core Metrics Collection

```python
import graphbit
import time
import json
from dataclasses import dataclass
from typing import Dict, List, Optional
from datetime import datetime, timedelta
import os

@dataclass
class WorkflowMetrics:
    """Core workflow execution metrics."""
    workflow_id: str
    workflow_name: str
    execution_id: str
    start_time: datetime
    end_time: Optional[datetime] = None
    duration_ms: Optional[int] = None
    status: str = "running"  # running, completed, failed
    node_count: int = 0
    nodes_executed: int = 0
    error_message: Optional[str] = None

class WorkflowMonitor:
    """Monitors workflow execution and collects metrics."""
    
    def __init__(self):
        self.metrics_store: List[WorkflowMetrics] = []
        self.active_executions: Dict[str, WorkflowMetrics] = {}
    
    def start_execution(self, workflow, execution_id=None):
        """Start monitoring a workflow execution."""
        
        if execution_id is None:
            execution_id = f"exec_{int(time.time())}"
        
        metrics = WorkflowMetrics(
            workflow_id=str(hash(str(workflow))),
            workflow_name=getattr(workflow, 'name', 'Unknown'),
            execution_id=execution_id,
            start_time=datetime.now(),
            node_count=workflow.node_count()
        )
        
        self.active_executions[execution_id] = metrics
        return execution_id
    
    def end_execution(self, execution_id, status="completed", error_message=None):
        """End monitoring a workflow execution."""
        
        if execution_id not in self.active_executions:
            return
        
        metrics = self.active_executions[execution_id]
        metrics.end_time = datetime.now()
        metrics.duration_ms = int(
            (metrics.end_time - metrics.start_time).total_seconds() * 1000
        )
        metrics.status = status
        metrics.error_message = error_message
        
        # Move to permanent storage
        self.metrics_store.append(metrics)
        del self.active_executions[execution_id]
    
    def get_metrics_summary(self, time_window_hours=24):
        """Get metrics summary for specified time window."""
        
        cutoff_time = datetime.now() - timedelta(hours=time_window_hours)
        recent_metrics = [
            m for m in self.metrics_store 
            if m.start_time > cutoff_time
        ]
        
        if not recent_metrics:
            return {"message": "No metrics in time window"}
        
        total_executions = len(recent_metrics)
        successful_executions = len([m for m in recent_metrics if m.status == "completed"])
        failed_executions = len([m for m in recent_metrics if m.status == "failed"])
        
        return {
            "time_window_hours": time_window_hours,
            "total_executions": total_executions,
            "successful_executions": successful_executions,
            "failed_executions": failed_executions,
            "success_rate": (successful_executions / total_executions) * 100 if total_executions > 0 else 0,
            "active_executions": len(self.active_executions)
        }

def create_monitored_workflow():
    """Create workflow with built-in monitoring."""
    
    builder = graphbit.PyWorkflowBuilder("Monitored Workflow")
    
    # Monitoring node
    monitor = graphbit.PyWorkflowNode.agent_node(
        name="Execution Monitor",
        description="Monitors workflow execution",
        agent_id="monitor",
        prompt="""
        Monitor this workflow execution:
        
        Input: {input}
        
        Report:
        - Processing status
        - Performance metrics
        - Any issues detected
        - Resource usage estimates
        """
    )
    
    # Main processor
    processor = graphbit.PyWorkflowNode.agent_node(
        name="Main Processor",
        description="Main workflow processing",
        agent_id="processor",
        prompt="Process this data: {input}"
    )
    
    # Build monitored workflow
    monitor_id = builder.add_node(monitor)
    processor_id = builder.add_node(processor)
    
    builder.connect(monitor_id, processor_id, graphbit.PyWorkflowEdge.data_flow())
    
    return builder.build()
```

## Performance Monitoring

### Execution Time Tracking

```python
import statistics

class PerformanceMonitor:
    """Monitors workflow performance metrics."""
    
    def __init__(self):
        self.execution_times: List[float] = []
        self.node_execution_times: Dict[str, List[float]] = {}
    
    def record_execution_time(self, duration_ms, workflow_name="unknown"):
        """Record workflow execution time."""
        self.execution_times.append(duration_ms)
    
    def record_node_execution_time(self, node_name, duration_ms):
        """Record individual node execution time."""
        if node_name not in self.node_execution_times:
            self.node_execution_times[node_name] = []
        self.node_execution_times[node_name].append(duration_ms)
    
    def get_performance_stats(self):
        """Get comprehensive performance statistics."""
        
        if not self.execution_times:
            return {"message": "No performance data available"}
        
        # Overall execution time stats
        avg_time = statistics.mean(self.execution_times)
        median_time = statistics.median(self.execution_times)
        
        # Node performance breakdown
        node_stats = {}
        for node_name, times in self.node_execution_times.items():
            if times:
                node_stats[node_name] = {
                    "average_ms": statistics.mean(times),
                    "median_ms": statistics.median(times),
                    "max_ms": max(times),
                    "execution_count": len(times)
                }
        
        return {
            "overall_performance": {
                "average_execution_ms": avg_time,
                "median_execution_ms": median_time,
                "total_executions": len(self.execution_times)
            },
            "node_performance": node_stats
        }

def create_performance_monitored_executor():
    """Create executor with performance monitoring."""
    
    class MonitoredExecutor:
        def __init__(self, base_executor):
            self.base_executor = base_executor
            self.perf_monitor = PerformanceMonitor()
        
        def execute_with_input(self, workflow, input_data):
            """Execute workflow with performance monitoring."""
            
            start_time = time.time()
            
            try:
                result = self.base_executor.execute_with_input(workflow, input_data)
                success = True
            except Exception as e:
                result = None
                success = False
                raise e
            finally:
                end_time = time.time()
                duration_ms = (end_time - start_time) * 1000
                
                if success:
                    self.perf_monitor.record_execution_time(duration_ms)
            
            return result
        
        def get_performance_stats(self):
            return self.perf_monitor.get_performance_stats()
    
    # Create base executor
    config = graphbit.PyLlmConfig.openai(
        api_key=os.getenv("OPENAI_API_KEY"),
        model="gpt-3.5-turbo"
    )
    
    base_executor = graphbit.PyWorkflowExecutor(config)
    return MonitoredExecutor(base_executor)
```

## Error Monitoring

### Error Tracking and Analysis

```python
@dataclass
class ErrorEvent:
    """Represents an error event in workflow execution."""
    timestamp: datetime
    workflow_id: str
    execution_id: str
    error_type: str
    error_message: str
    node_name: Optional[str] = None
    context: Optional[Dict] = None

class ErrorMonitor:
    """Monitors and analyzes workflow errors."""
    
    def __init__(self):
        self.error_events: List[ErrorEvent] = []
        self.error_patterns: Dict[str, int] = {}
    
    def record_error(self, workflow_id, execution_id, error, node_name=None, context=None):
        """Record an error event."""
        
        error_type = type(error).__name__
        error_message = str(error)
        
        event = ErrorEvent(
            timestamp=datetime.now(),
            workflow_id=workflow_id,
            execution_id=execution_id,
            error_type=error_type,
            error_message=error_message,
            node_name=node_name,
            context=context or {}
        )
        
        self.error_events.append(event)
        
        # Track error patterns
        pattern_key = f"{error_type}:{node_name or 'unknown'}"
        self.error_patterns[pattern_key] = self.error_patterns.get(pattern_key, 0) + 1
    
    def get_error_summary(self, time_window_hours=24):
        """Get error summary for specified time window."""
        
        cutoff_time = datetime.now() - timedelta(hours=time_window_hours)
        recent_errors = [
            e for e in self.error_events 
            if e.timestamp > cutoff_time
        ]
        
        if not recent_errors:
            return {"message": "No errors in time window"}
        
        # Group errors by type
        error_counts = {}
        for error in recent_errors:
            error_counts[error.error_type] = error_counts.get(error.error_type, 0) + 1
        
        return {
            "time_window_hours": time_window_hours,
            "total_errors": len(recent_errors),
            "error_types": error_counts,
            "error_rate_per_hour": len(recent_errors) / time_window_hours
        }

def create_error_monitored_workflow():
    """Create workflow with comprehensive error monitoring."""
    
    builder = graphbit.PyWorkflowBuilder("Error Monitored Workflow")
    
    # Error detector node
    error_detector = graphbit.PyWorkflowNode.agent_node(
        name="Error Detector",
        description="Detects and analyzes potential errors",
        agent_id="error_detector",
        prompt="""
        Analyze this input for potential issues:
        
        Input: {input}
        
        Check for:
        1. Data format issues
        2. Missing required fields
        3. Invalid values
        4. Potential processing failures
        
        Report any issues found and risk level (LOW/MEDIUM/HIGH).
        """
    )
    
    builder.add_node(error_detector)
    
    return builder.build()
```

## Health Monitoring

### System Health Checks

```python
import psutil
from typing import NamedTuple

class HealthStatus(NamedTuple):
    """System health status."""
    status: str  # healthy, warning, critical
    checks: Dict[str, Dict]
    timestamp: datetime

class HealthMonitor:
    """Monitors system health and availability."""
    
    def __init__(self):
        self.health_history: List[HealthStatus] = []
        self.thresholds = {
            "memory_usage_percent": 80,
            "cpu_usage_percent": 85
        }
    
    def run_health_check(self):
        """Run comprehensive health check."""
        
        checks = {}
        overall_status = "healthy"
        
        # Memory check
        memory = psutil.virtual_memory()
        checks["memory"] = {
            "status": "healthy" if memory.percent < self.thresholds["memory_usage_percent"] else "warning",
            "usage_percent": memory.percent,
            "available_gb": memory.available / (1024**3),
            "total_gb": memory.total / (1024**3)
        }
        
        # CPU check
        cpu_percent = psutil.cpu_percent(interval=1)
        checks["cpu"] = {
            "status": "healthy" if cpu_percent < self.thresholds["cpu_usage_percent"] else "warning",
            "usage_percent": cpu_percent,
            "core_count": psutil.cpu_count()
        }
        
        # Determine overall status
        warning_checks = [name for name, check in checks.items() if check["status"] == "warning"]
        critical_checks = [name for name, check in checks.items() if check["status"] == "critical"]
        
        if critical_checks:
            overall_status = "critical"
        elif warning_checks:
            overall_status = "warning"
        
        health_status = HealthStatus(
            status=overall_status,
            checks=checks,
            timestamp=datetime.now()
        )
        
        self.health_history.append(health_status)
        
        # Keep only last 100 health checks
        if len(self.health_history) > 100:
            self.health_history = self.health_history[-100:]
        
        return health_status
    
    def get_health_summary(self):
        """Get health summary and trends."""
        
        if not self.health_history:
            return {"message": "No health data available"}
        
        current_health = self.health_history[-1]
        
        return {
            "current_status": current_health.status,
            "last_check_time": current_health.timestamp.isoformat(),
            "current_checks": current_health.checks
        }
```

## Custom Metrics

### Business Logic Monitoring

```python
class CustomMetricsCollector:
    """Collects custom business metrics."""
    
    def __init__(self):
        self.counters: Dict[str, int] = {}
        self.gauges: Dict[str, float] = {}
        self.timestamps: Dict[str, datetime] = {}
    
    def increment_counter(self, name, value=1, labels=None):
        """Increment a counter metric."""
        metric_key = self._build_metric_key(name, labels)
        self.counters[metric_key] = self.counters.get(metric_key, 0) + value
        self.timestamps[metric_key] = datetime.now()
    
    def set_gauge(self, name, value, labels=None):
        """Set a gauge metric value."""
        metric_key = self._build_metric_key(name, labels)
        self.gauges[metric_key] = value
        self.timestamps[metric_key] = datetime.now()
    
    def _build_metric_key(self, name, labels):
        """Build metric key with labels."""
        if labels:
            label_str = ",".join(f"{k}={v}" for k, v in sorted(labels.items()))
            return f"{name}{{{label_str}}}"
        return name
    
    def get_metrics_export(self):
        """Export metrics in a standard format."""
        
        export_data = {
            "timestamp": datetime.now().isoformat(),
            "counters": self.counters,
            "gauges": self.gauges
        }
        
        return export_data

def create_business_monitored_workflow():
    """Create workflow with business metrics monitoring."""
    
    metrics = CustomMetricsCollector()
    
    builder = graphbit.PyWorkflowBuilder("Business Monitored Workflow")
    
    # Business logic processor with metrics
    processor = graphbit.PyWorkflowNode.agent_node(
        name="Business Processor",
        description="Processes business logic with metrics",
        agent_id="business_processor",
        prompt="""
        Process this business data and track metrics:
        
        Input: {input}
        
        Process the data and report:
        1. Processing outcome
        2. Business value generated
        3. Quality score (1-10)
        4. Processing complexity (simple/medium/complex)
        5. Any business rule violations
        """
    )
    
    builder.add_node(processor)
    
    return builder.build(), metrics
```

## Dashboard Integration

### Real-time Monitoring Dashboard

```python
def create_monitoring_dashboard():
    """Create a simple monitoring dashboard."""
    
    class MonitoringDashboard:
        def __init__(self):
            self.workflow_monitor = WorkflowMonitor()
            self.performance_monitor = PerformanceMonitor()
            self.error_monitor = ErrorMonitor()
            self.health_monitor = HealthMonitor()
            self.custom_metrics = CustomMetricsCollector()
        
        def get_dashboard_data(self):
            """Get all dashboard data."""
            
            # Get latest metrics
            workflow_summary = self.workflow_monitor.get_metrics_summary()
            performance_stats = self.performance_monitor.get_performance_stats()
            error_summary = self.error_monitor.get_error_summary()
            health_status = self.health_monitor.run_health_check()
            custom_metrics_export = self.custom_metrics.get_metrics_export()
            
            dashboard_data = {
                "timestamp": datetime.now().isoformat(),
                "workflow_metrics": workflow_summary,
                "performance_metrics": performance_stats,
                "error_metrics": error_summary,
                "health_status": {
                    "status": health_status.status,
                    "checks": health_status.checks
                },
                "custom_metrics": custom_metrics_export
            }
            
            return dashboard_data
        
        def print_dashboard(self):
            """Print dashboard to console."""
            
            data = self.get_dashboard_data()
            
            print("=" * 60)
            print("GraphBit Monitoring Dashboard")
            print("=" * 60)
            print(f"Last Updated: {data['timestamp']}")
            print()
            
            # Workflow metrics
            wf_metrics = data["workflow_metrics"]
            if "total_executions" in wf_metrics:
                print(f"📊 Workflow Metrics (24h):")
                print(f"  Total Executions: {wf_metrics['total_executions']}")
                print(f"  Success Rate: {wf_metrics['success_rate']:.1f}%")
                print(f"  Active Executions: {wf_metrics['active_executions']}")
                print()
            
            # Health status
            health = data["health_status"]
            status_icon = {"healthy": "✅", "warning": "⚠️", "critical": "🚨"}
            print(f"{status_icon.get(health['status'], '❓')} System Health: {health['status'].upper()}")
            
            for check_name, check_result in health["checks"].items():
                check_icon = "✅" if check_result["status"] == "healthy" else "⚠️"
                print(f"  {check_icon} {check_name}: {check_result['status']}")
            print()
            
            print("=" * 60)
    
    return MonitoringDashboard()
```

## Best Practices

### 1. Monitoring Strategy

```python
def create_comprehensive_monitoring_strategy():
    """Create a comprehensive monitoring strategy."""
    
    strategy = {
        "metrics_to_track": [
            "execution_time",
            "success_rate", 
            "error_rate",
            "throughput",
            "resource_usage",
            "business_kpis"
        ],
        "alert_thresholds": {
            "error_rate_percent": 5.0,
            "avg_execution_time_ms": 30000,
            "memory_usage_percent": 80,
            "cpu_usage_percent": 85
        },
        "monitoring_intervals": {
            "health_check": "1_minute",
            "performance_metrics": "30_seconds", 
            "error_analysis": "5_minutes",
            "dashboard_refresh": "10_seconds"
        }
    }
    
    return strategy
```

### 2. Performance Baseline Establishment

```python
def establish_performance_baseline():
    """Establish performance baselines for alerting."""
    
    def run_baseline_tests():
        """Run tests to establish performance baselines."""
        
        baseline_data = {
            "avg_execution_time_ms": 5000,
            "success_rate_percent": 98.5,
            "throughput_per_hour": 100
        }
        
        return baseline_data
    
    return run_baseline_tests()
```

Comprehensive monitoring and observability in GraphBit ensures reliable operation, quick issue detection, and continuous performance optimization. Use these patterns to build robust monitoring into your GraphBit deployments. 