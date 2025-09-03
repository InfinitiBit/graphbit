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
import time
import json
import uuid
import os

from dataclasses import dataclass
from typing import Dict, List, Optional
from datetime import datetime, timedelta
from graphbit import Workflow, Node, LlmConfig, Executor

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
            execution_id = f"exec_{int(time.time())}_{uuid.uuid4().hex[:8]}"
        
        # Count nodes in workflow
        try:
            node_count = len(workflow.nodes()) if hasattr(workflow, 'nodes') 
        except Exception:
            node_count = 1  # Default assumption
        
        metrics = WorkflowMetrics(
            workflow_id=str(hash(str(workflow))),
            workflow_name=getattr(workflow, 'name', 'Unknown'),
            execution_id=execution_id,
            start_time=datetime.now(),
            node_count=node_count
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
        
        # Assume all nodes executed if none recorded
        if metrics.nodes_executed == 0:
            metrics.nodes_executed = metrics.node_count
        
        # Move to permanent storage
        self.metrics_store.append(metrics)
        del self.active_executions[execution_id]
    
    def get_metrics_summary(self, time_window_hours=24):
        """Get metrics summary for specified time window."""
        cutoff_time = datetime.now() - timedelta(hours=time_window_hours)
        recent_metrics = [m for m in self.metrics_store if m.start_time > cutoff_time]
        
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
    workflow = Workflow("Monitored Workflow")

    monitor = Node.agent(
        name="Execution Monitor",
        prompt='''
            "Monitor this workflow execution:\n\n"
            
            "Report:\n"
            "- Processing status\n"
            "- Performance metrics\n"
            "- Any issues detected\n"
            "- Resource usage estimates\n"
            ''',
        agent_id="monitor"
    )
    
    processor = Node.agent(
        name="Main Processor",
        prompt="Process this data.",
        agent_id="processor"
    )
    
    monitor_id = workflow.add_node(monitor)
    processor_id = workflow.add_node(processor)
    workflow.connect(monitor_id, processor_id)
    return workflow

def run_with_monitor(workflow: Workflow, monitor: WorkflowMonitor) -> dict:
    """Execute a workflow while collecting metrics."""
    exec_id = monitor.start_execution(workflow)
    try:
        config = LlmConfig.openai(os.getenv('OPENAI_API_KEY'))

        executor = Executor(config)
        result = executor.execute(workflow)
        
        monitor.end_execution(exec_id, status="completed")
        return {"execution_id": exec_id, "status": "completed", "result": result.get_all_node_outputs()}
    except Exception as e:
        monitor.end_execution(exec_id, status="failed", error_message=str(e))
        return {"execution_id": exec_id, "status": "failed", "error": str(e)}

# ---------- example usage ----------
if __name__ == "__main__":
    wf = create_monitored_workflow()
    wm = WorkflowMonitor()
    
    outcome = run_with_monitor(wf, wm)
    print("Run outcome:", json.dumps(outcome, default=str, indent=2))
    print("Summary (24h):", json.dumps(wm.get_metrics_summary(24), default=str, indent=2))
```

## Performance Monitoring

### Execution Time Tracking

```python
import statistics
import threading
from collections import defaultdict

class PerformanceMonitor:
    """Monitors workflow performance metrics."""
    
    def __init__(self):
        self.execution_times: List[float] = []
        self.node_execution_times: Dict[str, List[float]] = defaultdict(list)
        self.client_stats: Dict[str, Dict] = {}
        self._lock = threading.Lock()
    
    def record_execution_time(self, duration_ms, workflow_name="unknown"):
        """Record workflow execution time."""
        with self._lock:
            self.execution_times.append(duration_ms)
    
    def record_node_execution_time(self, node_name, duration_ms):
        """Record individual node execution time."""
        with self._lock:
            self.node_execution_times[node_name].append(duration_ms)
    
    def track_llm_performance(self, client):
        """Track LLM client performance metrics."""
        try:
            stats = client.get_stats()
            provider = stats.get("provider", "unknown")
            
            with self._lock:
                if provider not in self.client_stats:
                    self.client_stats[provider] = {
                        "total_requests": 0,
                        "total_tokens": 0,
                        "average_latency": 0,
                        "error_count": 0
                    }
                
                # Update stats
                self.client_stats[provider]["total_requests"] = stats.get("total_requests", 0)
                self.client_stats[provider]["total_tokens"] = stats.get("total_tokens", 0)
                self.client_stats[provider]["average_latency"] = stats.get("average_latency_ms", 0)
                self.client_stats[provider]["error_count"] = stats.get("error_count", 0)
                
        except Exception as e:
            print(f"Failed to track LLM performance: {e}")
    
    def get_performance_stats(self):
        """Get comprehensive performance statistics."""
        
        with self._lock:
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
                "node_performance": node_stats,
                "llm_performance": dict(self.client_stats)
            }

def monitor_workflow_execution(workflow, executor, monitor=None):
    """Execute workflow with performance monitoring."""
    
    if monitor is None:
        monitor = PerformanceMonitor()
    
    start_time = time.time()
    
    try:
        # Execute workflow
        result = executor.execute(workflow)
        
        # Calculate execution time
        duration_ms = (time.time() - start_time) * 1000
        monitor.record_execution_time(duration_ms, workflow.name if hasattr(workflow, 'name') else 'Unknown')
        
        # Track LLM performance if possible
        if hasattr(executor, 'client'):
            monitor.track_llm_performance(executor.client)
        
        return result, monitor
        
    except Exception as e:
        duration_ms = (time.time() - start_time) * 1000
        monitor.record_execution_time(duration_ms, workflow.name if hasattr(workflow, 'name') else 'Unknown')
        raise e
```

## System Health Monitoring

### Health Check Implementation

```python
import time
import threading
from typing import List, Dict

from datetime import datetime, timedelta
from graphbit import health_check, get_system_info


class SystemHealthMonitor:
    """Monitor GraphBit system health."""
    
    def __init__(self, check_interval_seconds=60):
        self.check_interval = check_interval_seconds
        self.health_history: List[Dict] = []
        self.alerts: List[Dict] = []
        self._monitoring = False
        
    def start_monitoring(self):
        """Start continuous health monitoring."""
        
        if self._monitoring:
            return
            
        self._monitoring = True
        
        def monitor_loop():
            while self._monitoring:
                try:
                    health = self.check_system_health()
                    self.health_history.append({
                        "timestamp": datetime.now(),
                        "health": health
                    })
                    
                    # Keep only last 100 health checks
                    if len(self.health_history) > 100:
                        self.health_history = self.health_history[-100:]
                    
                    # Check for alerts
                    self._check_alerts(health)
                    
                except Exception as e:
                    print(f"Health monitoring error: {e}")
                
                time.sleep(self.check_interval)
        
        # Start monitoring in background thread
        monitor_thread = threading.Thread(target=monitor_loop, daemon=True)
        monitor_thread.start()
    
    def stop_monitoring(self):
        """Stop health monitoring."""
        self._monitoring = False
    
    def check_system_health(self):
        """Check current system health."""
        
        try:
            # Use GraphBit's built-in health check
            health = health_check()
            
            # Add custom health metrics
            custom_health = {
                **health,
                "timestamp": datetime.now().isoformat(),
                "custom_checks": self._perform_custom_checks()
            }
            
            return custom_health
            
        except Exception as e:
            return {
                "overall_healthy": False,
                "error": str(e),
                "timestamp": datetime.now().isoformat()
            }
    
    def _perform_custom_checks(self):
        """Perform custom health checks."""
        
        checks = {}
        
        # Check system info availability
        try:
            info = get_system_info()
            checks["system_info_available"] = True
            checks["runtime_initialized"] = info.get("runtime_initialized", False)
        except:
            checks["system_info_available"] = False
            checks["runtime_initialized"] = False
        
        # Check memory usage
        try:
            import psutil
            memory = psutil.virtual_memory()
            checks["memory_usage_percent"] = memory.percent
            checks["memory_available_gb"] = memory.available / (1024**3)
        except:
            checks["memory_usage_percent"] = None
            checks["memory_available_gb"] = None
        
        return checks
    
    def _check_alerts(self, health):
        """Check for alert conditions."""
        
        alerts = []
        
        # Check overall health
        if not health.get("overall_healthy", False):
            alerts.append({
                "level": "critical",
                "message": "System health check failed",
                "timestamp": datetime.now()
            })
        
        # Check memory usage
        custom_checks = health.get("custom_checks", {})
        memory_usage = custom_checks.get("memory_usage_percent")
        
        if memory_usage and memory_usage > 90:
            alerts.append({
                "level": "warning",
                "message": f"High memory usage: {memory_usage:.1f}%",
                "timestamp": datetime.now()
            })
        
        # Store alerts
        self.alerts.extend(alerts)
        
        # Keep only recent alerts (last 24 hours)
        cutoff = datetime.now() - timedelta(hours=24)
        self.alerts = [a for a in self.alerts if a["timestamp"] > cutoff]
    
    def get_health_summary(self):
        """Get health monitoring summary."""
        
        if not self.health_history:
            return {"message": "No health data available"}
        
        recent_health = self.health_history[-10:]  # Last 10 checks
        healthy_count = sum(1 for h in recent_health if h["health"].get("overall_healthy", False))
        
        return {
            "current_health": self.health_history[-1]["health"] if self.health_history else None,
            "recent_success_rate": (healthy_count / len(recent_health)) * 100,
            "total_health_checks": len(self.health_history),
            "active_alerts": len([a for a in self.alerts if a["level"] == "critical"]),
            "warnings": len([a for a in self.alerts if a["level"] == "warning"])
        }
```

## Error Tracking

### Comprehensive Error Monitoring

```python
class ErrorTracker:
    """Track and analyze errors in GraphBit workflows."""
    
    def __init__(self):
        self.errors: List[Dict] = []
        self.error_patterns: Dict[str, int] = defaultdict(int)
        
    def record_error(self, error, context=None):
        """Record an error with context."""
        
        error_record = {
            "timestamp": datetime.now(),
            "error_type": type(error).__name__,
            "error_message": str(error),
            "context": context or {},
            "stack_trace": None
        }
        
        # Capture stack trace if available
        try:
            import traceback
            error_record["stack_trace"] = traceback.format_exc()
        except:
            pass
        
        self.errors.append(error_record)
        
        # Track error patterns
        self.error_patterns[error_record["error_type"]] += 1
        
        # Keep only recent errors (last 1000)
        if len(self.errors) > 1000:
            self.errors = self.errors[-1000:]
    
    def get_error_summary(self, time_window_hours=24):
        """Get error summary for time window."""
        
        cutoff = datetime.now() - timedelta(hours=time_window_hours)
        recent_errors = [e for e in self.errors if e["timestamp"] > cutoff]
        
        if not recent_errors:
            return {"message": "No errors in time window"}
        
        # Group by error type
        error_counts = defaultdict(int)
        for error in recent_errors:
            error_counts[error["error_type"]] += 1
        
        return {
            "total_errors": len(recent_errors),
            "unique_error_types": len(error_counts),
            "error_breakdown": dict(error_counts),
            "most_common_error": max(error_counts.items(), key=lambda x: x[1]) if error_counts else None
        }
    
    def analyze_error_trends(self):
        """Analyze error trends over time."""
        
        if len(self.errors) < 2:
            return {"message": "Insufficient data for trend analysis"}
        
        # Group errors by hour
        hourly_errors = defaultdict(int)
        
        for error in self.errors:
            hour_key = error["timestamp"].strftime("%Y-%m-%d %H:00")
            hourly_errors[hour_key] += 1
        
        # Calculate trend
        error_counts = list(hourly_errors.values())
        
        if len(error_counts) >= 2:
            recent_avg = sum(error_counts[-3:]) / min(3, len(error_counts))
            older_avg = sum(error_counts[:-3]) / max(1, len(error_counts) - 3)
            
            trend = "increasing" if recent_avg > older_avg else "decreasing"
        else:
            trend = "stable"
        
        return {
            "trend": trend,
            "hourly_breakdown": dict(hourly_errors),
            "peak_error_hour": max(hourly_errors.items(), key=lambda x: x[1])[0] if hourly_errors else None
        }

def execute_with_error_tracking(workflow, executor, error_tracker=None):
    """Execute workflow with comprehensive error tracking."""
    
    if error_tracker is None:
        error_tracker = ErrorTracker()
    
    try:
        result = executor.execute(workflow)
        
        # Check if execution failed
        if result.is_failed():
            error_context = {
                "workflow_name": getattr(workflow, 'name', 'Unknown'),
                "execution_failed": True
            }
            
            try:
                error_tracker.record_error(
                    Exception(f"Workflow execution failed: {error_message}"),
                    error_context
                )
            except Exception as e:
                error_tracker.record_error(e, error_context)
        
        return result, error_tracker
        
    except Exception as e:
        error_context = {
            "workflow_name": getattr(workflow, 'name', 'Unknown'),
            "execution_exception": True
        }
        error_tracker.record_error(e, error_context)
        raise e
```

## Custom Metrics and Dashboards

### Business Metrics Collection

```python
class BusinessMetricsCollector:
    """Collect custom business metrics."""
    
    def __init__(self):
        self.metrics: Dict[str, List] = defaultdict(list)
        
    def record_metric(self, metric_name, value, tags=None):
        """Record a custom business metric."""
        
        metric_record = {
            "timestamp": datetime.now(),
            "value": value,
            "tags": tags or {}
        }
        
        self.metrics[metric_name].append(metric_record)
        
        # Keep only recent metrics (last 10000 per metric)
        if len(self.metrics[metric_name]) > 10000:
            self.metrics[metric_name] = self.metrics[metric_name][-10000:]
    
    def get_metric_summary(self, metric_name, time_window_hours=24):
        """Get summary for a specific metric."""
        
        if metric_name not in self.metrics:
            return {"message": f"Metric '{metric_name}' not found"}
        
        cutoff = datetime.now() - timedelta(hours=time_window_hours)
        recent_values = [
            m["value"] for m in self.metrics[metric_name]
            if m["timestamp"] > cutoff
        ]
        
        if not recent_values:
            return {"message": "No recent data for metric"}
        
        return {
            "metric_name": metric_name,
            "count": len(recent_values),
            "average": statistics.mean(recent_values),
            "median": statistics.median(recent_values),
            "min": min(recent_values),
            "max": max(recent_values),
            "total": sum(recent_values)
        }

# Example usage with workflow execution
def execute_with_business_metrics(workflow, executor, node_name, metrics_collector=None):
    """Execute workflow with business metrics collection."""
    
    if metrics_collector is None:
        metrics_collector = BusinessMetricsCollector()
    
    # Record execution start
    start_time = time.time()
    metrics_collector.record_metric("workflow_executions_started", 1)
    
    try:
        result = executor.execute(workflow)
        
        # Record execution time
        execution_time = (time.time() - start_time) * 1000
        metrics_collector.record_metric("workflow_execution_time_ms", execution_time)
        
        if result.is_success():
            metrics_collector.record_metric("workflow_executions_completed", 1)
            
            # Record output length if available
            try:
                output = result.get_node_output(node_name)
                if isinstance(output, str):
                    metrics_collector.record_metric("workflow_output_length", len(output))
            except:
                pass
        else:
            metrics_collector.record_metric("workflow_executions_failed", 1)
        
        return result, metrics_collector
        
    except Exception as e:
        metrics_collector.record_metric("workflow_executions_failed", 1)
        execution_time = (time.time() - start_time) * 1000
        metrics_collector.record_metric("workflow_execution_time_ms", execution_time)
        raise e
```

## Real-time Monitoring Dashboard

### Simple Text-based Dashboard

```python
from graphbit import init

class MonitoringDashboard:
    """Simple text-based monitoring dashboard."""
    
    def __init__(self, workflow_monitor, performance_monitor, error_tracker, health_monitor):
        self.workflow_monitor = workflow_monitor
        self.performance_monitor = performance_monitor
        self.error_tracker = error_tracker
        self.health_monitor = health_monitor
    
    def display_dashboard(self):
        """Display comprehensive monitoring dashboard."""
        
        print("=" * 60)
        print("GraphBit Monitoring Dashboard")
        print("=" * 60)
        print(f"Updated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
        print()
        
        # System Health
        print("🏥 System Health")
        print("-" * 20)
        health_summary = self.health_monitor.get_health_summary()
        current_health = health_summary.get("current_health", {})
        
        overall_healthy = current_health.get("overall_healthy", False)
        health_icon = "✅" if overall_healthy else "❌"
        print(f"{health_icon} Overall Health: {'Healthy' if overall_healthy else 'Unhealthy'}")
        
        success_rate = health_summary.get("recent_success_rate", 0)
        print(f"📊 Recent Success Rate: {success_rate:.1f}%")
        
        alerts = health_summary.get("active_alerts", 0)
        warnings = health_summary.get("warnings", 0)
        if alerts > 0:
            print(f"🚨 Active Alerts: {alerts}")
        if warnings > 0:
            print(f"⚠️  Warnings: {warnings}")
        print()
        
        # Workflow Metrics
        print("⚡ Workflow Metrics (Last 24 Hours)")
        print("-" * 35)
        workflow_summary = self.workflow_monitor.get_metrics_summary(24)
        
        if "message" not in workflow_summary:
            total_executions = workflow_summary.get("total_executions", 0)
            success_rate = workflow_summary.get("success_rate", 0)
            active_executions = workflow_summary.get("active_executions", 0)
            
            print(f"📈 Total Executions: {total_executions}")
            print(f"🎯 Success Rate: {success_rate:.1f}%")
            print(f"🔄 Active Executions: {active_executions}")
        else:
            print("📝 No workflow data available")
        print()
        
        # Performance Metrics
        print("🚀 Performance Metrics")
        print("-" * 22)
        perf_stats = self.performance_monitor.get_performance_stats()
        
        if "message" not in perf_stats:
            overall_perf = perf_stats.get("overall_performance", {})
            avg_time = overall_perf.get("average_execution_ms", 0)
            total_executions = overall_perf.get("total_executions", 0)
            
            print(f"⏱️  Average Execution Time: {avg_time:.1f}ms")
            print(f"📊 Total Executions: {total_executions}")
            
            # LLM Performance
            llm_perf = perf_stats.get("llm_performance", {})
            for provider, stats in llm_perf.items():
                print(f"🤖 {provider} - Requests: {stats.get('total_requests', 0)}, "
                      f"Avg Latency: {stats.get('average_latency', 0):.1f}ms")
        else:
            print("📝 No performance data available")
        print()
        
        # Error Summary
        print("🐛 Error Summary (Last 24 Hours)")
        print("-" * 30)
        error_summary = self.error_tracker.get_error_summary(24)
        
        if "message" not in error_summary:
            total_errors = error_summary.get("total_errors", 0)
            unique_types = error_summary.get("unique_error_types", 0)
            most_common = error_summary.get("most_common_error")
            
            print(f"🔴 Total Errors: {total_errors}")
            print(f"🔢 Unique Error Types: {unique_types}")
            
            if most_common:
                print(f"📈 Most Common: {most_common[0]} ({most_common[1]} occurrences)")
        else:
            print("📝 No error data available")
        
        print("=" * 60)
    
    def start_live_dashboard(self, refresh_interval=30):
        """Start live dashboard with auto-refresh."""
        
        import os
        
        try:
            while True:
                # Clear screen (works on most terminals)
                os.system('clear' if os.name == 'posix' else 'cls')
                
                self.display_dashboard()
                
                print(f"\nRefreshing in {refresh_interval} seconds... (Ctrl+C to stop)")
                time.sleep(refresh_interval)
                
        except KeyboardInterrupt:
            print("\n\nDashboard stopped.")

# Complete monitoring setup
def setup_comprehensive_monitoring():
    """Set up comprehensive monitoring for GraphBit workflows."""
    
    # Initialize all monitors
    workflow_monitor = WorkflowMonitor()
    performance_monitor = PerformanceMonitor()
    error_tracker = ErrorTracker()
    health_monitor = SystemHealthMonitor()
    
    # Start health monitoring
    health_monitor.start_monitoring()
    
    # Create dashboard
    dashboard = MonitoringDashboard(
        workflow_monitor, 
        performance_monitor, 
        error_tracker, 
        health_monitor
    )
    
    return {
        "workflow_monitor": workflow_monitor,
        "performance_monitor": performance_monitor,
        "error_tracker": error_tracker,
        "health_monitor": health_monitor,
        "dashboard": dashboard
    }

# Usage example
if __name__ == "__main__":
    # Initialize GraphBit
    init()
    
    # Set up monitoring
    monitoring = setup_comprehensive_monitoring()
    
    # Display dashboard once
    monitoring["dashboard"].display_dashboard()
    
    # Or start live dashboard
    # monitoring["dashboard"].start_live_dashboard(refresh_interval=30)
```

## Integration with External Systems

### Exporting Metrics

```python
from graphbit import get_system_info

def export_metrics_to_json(monitors, output_file="graphbit_metrics.json"):
    """Export all metrics to JSON file."""
    
    workflow_monitor, performance_monitor, error_tracker, health_monitor = monitors
    
    export_data = {
        "timestamp": datetime.now().isoformat(),
        "workflow_metrics": workflow_monitor.get_metrics_summary(24),
        "performance_metrics": performance_monitor.get_performance_stats(),
        "error_summary": error_tracker.get_error_summary(24),
        "health_summary": health_monitor.get_health_summary(),
        "system_info": None
    }
    
    # Add system info if available
    try:
        export_data["system_info"] = get_system_info()
    except:
        pass
    
    with open(output_file, 'w') as f:
        json.dump(export_data, f, indent=2, default=str)
    
    print(f"Metrics exported to {output_file}")

def send_alerts_to_webhook(health_monitor, webhook_url):
    """Send alerts to external webhook."""
    
    try:
        import requests
        
        health_summary = health_monitor.get_health_summary()
        alerts = health_summary.get("active_alerts", 0)
        
        if alerts > 0:
            payload = {
                "message": f"GraphBit Health Alert: {alerts} active alerts",
                "timestamp": datetime.now().isoformat(),
                "health_summary": health_summary
            }
            
            response = requests.post(webhook_url, json=payload, timeout=10)
            
            if response.status_code == 200:
                print("Alert sent successfully")
            else:
                print(f"Failed to send alert: {response.status_code}")
                
    except Exception as e:
        print(f"Failed to send alert: {e}")
```

## Best Practices

### 1. Monitoring Strategy

Implement layered monitoring:

```python
def create_production_monitoring_setup():
    """Create production-ready monitoring setup."""
    
    # Core monitoring components
    monitors = setup_comprehensive_monitoring()
    
    # Configure alerts
    health_monitor = monitors["health_monitor"]
    
    # Set up metric exports
    def export_metrics_periodically():
        while True:
            try:
                export_metrics_to_json([
                    monitors["workflow_monitor"],
                    monitors["performance_monitor"], 
                    monitors["error_tracker"],
                    monitors["health_monitor"]
                ])
                time.sleep(300)  # Export every 5 minutes
            except Exception as e:
                print(f"Metric export failed: {e}")
                time.sleep(60)  # Retry in 1 minute
    
    # Start background export
    import threading
    export_thread = threading.Thread(target=export_metrics_periodically, daemon=True)
    export_thread.start()
    
    return monitors
```

### 2. Performance Baselines

Establish performance baselines:

```python
def establish_performance_baseline(workflow, executor, iterations=10):
    """Establish performance baseline for a workflow."""
    
    performance_monitor = PerformanceMonitor()
    execution_times = []
    
    print(f"Establishing baseline with {iterations} iterations...")
    
    for i in range(iterations):
        start_time = time.time()
        
        try:
            result = executor.execute(workflow)
            duration_ms = (time.time() - start_time) * 1000
            execution_times.append(duration_ms)
            performance_monitor.record_execution_time(duration_ms)
            
            print(f"  Iteration {i+1}: {duration_ms:.1f}ms")
            
        except Exception as e:
            print(f"  Iteration {i+1}: FAILED - {e}")
    
    if execution_times:
        baseline = {
            "average_ms": statistics.mean(execution_times),
            "median_ms": statistics.median(execution_times),
            "p95_ms": sorted(execution_times)[int(len(execution_times) * 0.95)],
            "iterations": len(execution_times)
        }
        
        print(f"\nBaseline established:")
        print(f"  Average: {baseline['average_ms']:.1f}ms")
        print(f"  Median: {baseline['median_ms']:.1f}ms")
        print(f"  P95: {baseline['p95_ms']:.1f}ms")
        
        return baseline
    else:
        print("Failed to establish baseline - no successful executions")
        return None
```

### 3. Automated Alerting

Set up automated alerting:

```python
def setup_automated_alerting(health_monitor, thresholds=None):
    """Set up automated alerting based on health metrics."""
    
    if thresholds is None:
        thresholds = {
            "error_rate_percent": 10,
            "memory_usage_percent": 85,
            "success_rate_percent": 95
        }
    
    def check_and_alert():
        health_summary = health_monitor.get_health_summary()
        current_health = health_summary.get("current_health", {})
        
        alerts = []
        
        # Check memory usage
        custom_checks = current_health.get("custom_checks", {})
        memory_usage = custom_checks.get("memory_usage_percent")
        
        if memory_usage and memory_usage > thresholds["memory_usage_percent"]:
            alerts.append(f"High memory usage: {memory_usage:.1f}%")
        
        # Check success rate
        success_rate = health_summary.get("recent_success_rate", 100)
        if success_rate < thresholds["success_rate_percent"]:
            alerts.append(f"Low success rate: {success_rate:.1f}%")
        
        # Send alerts if any
        if alerts:
            print("🚨 ALERTS TRIGGERED:")
            for alert in alerts:
                print(f"  - {alert}")
        
        return alerts
    
    return check_and_alert
```

## What's Next

- Learn about [Performance](performance.md) optimization techniques
- Explore [Reliability](reliability.md) patterns for production systems
- Check [Validation](validation.md) for comprehensive testing strategies
- See [LLM Providers](llm-providers.md) for provider-specific monitoring
