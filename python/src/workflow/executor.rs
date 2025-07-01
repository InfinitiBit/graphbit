//! Production-grade workflow executor for GraphBit Python bindings
//!
//! This module provides a robust, high-performance workflow executor with:
//! - Comprehensive input validation
//! - Configurable execution modes and timeouts
//! - Resource monitoring and management
//! - Detailed execution metrics and logging
//! - Graceful error handling and recovery

use graphbit_core::workflow::WorkflowExecutor as CoreWorkflowExecutor;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict};
use std::time::{Duration, Instant};
use tracing::{debug, error, info, instrument, warn};

use super::{result::WorkflowResult, workflow::Workflow};
use crate::errors::{timeout_error, to_py_runtime_error, validation_error};
use crate::llm::config::LlmConfig;
use crate::runtime::get_runtime;

/// Execution mode for different performance characteristics
#[derive(Debug, Clone, Copy)]
pub(crate) enum ExecutionMode {
    /// High-throughput mode for batch processing
    HighThroughput,
    /// Low-latency mode for real-time applications
    LowLatency,
    /// Memory-optimized mode for resource-constrained environments
    MemoryOptimized,
    /// Balanced mode for general use
    Balanced,
}

/// Execution configuration for fine-tuning performance
#[derive(Debug, Clone)]
pub(crate) struct ExecutionConfig {
    /// Execution mode
    pub mode: ExecutionMode,
    /// Request timeout in seconds
    pub timeout: Duration,
    /// Maximum retries for failed operations
    pub max_retries: u32,
    /// Enable detailed execution metrics
    pub enable_metrics: bool,
    /// Enable execution tracing
    pub enable_tracing: bool,
    /// Maximum concurrent operations
    pub max_concurrency: Option<usize>,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            mode: ExecutionMode::Balanced,
            timeout: Duration::from_secs(300), // 5 minutes
            max_retries: 3,
            enable_metrics: true,
            enable_tracing: false, // Default to false to reduce debug output
            max_concurrency: Some(num_cpus::get() * 2),
        }
    }
}

/// Execution statistics for monitoring
#[derive(Debug, Clone)]
pub(crate) struct ExecutionStats {
    pub total_executions: u64,
    pub successful_executions: u64,
    pub failed_executions: u64,
    pub average_duration_ms: f64,
    pub total_duration_ms: u64,
    pub created_at: Instant,
}

impl Default for ExecutionStats {
    fn default() -> Self {
        Self {
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            average_duration_ms: 0.0,
            total_duration_ms: 0,
            created_at: Instant::now(),
        }
    }
}

/// Production-grade workflow executor with comprehensive features
#[pyclass]
pub struct Executor {
    /// Execution configuration
    config: ExecutionConfig,
    /// LLM configuration for auto-generating agents
    llm_config: LlmConfig,
    /// Execution statistics
    stats: ExecutionStats,
}

#[pymethods]
impl Executor {
    #[new]
    #[pyo3(signature = (config, lightweight_mode=None, timeout_seconds=None, debug=None))]
    fn new(
        config: LlmConfig,
        lightweight_mode: Option<bool>,
        timeout_seconds: Option<u64>,
        debug: Option<bool>,
    ) -> PyResult<Self> {
        // Validate inputs
        if let Some(timeout) = timeout_seconds {
            if timeout == 0 || timeout > 3600 {
                return Err(validation_error(
                    "timeout_seconds",
                    Some(&timeout.to_string()),
                    "Timeout must be between 1 and 3600 seconds",
                ));
            }
        }

        let mut exec_config = ExecutionConfig::default();

        // Configure based on lightweight mode for backward compatibility
        if let Some(lightweight) = lightweight_mode {
            exec_config.mode = if lightweight {
                ExecutionMode::LowLatency
            } else {
                ExecutionMode::HighThroughput
            };
        }

        // Set timeout if specified
        if let Some(timeout) = timeout_seconds {
            exec_config.timeout = Duration::from_secs(timeout);
        }

        // Set debug mode - defaults to false
        exec_config.enable_tracing = debug.unwrap_or(false);

        if exec_config.enable_tracing {
            info!(
                "Created executor with mode: {:?}, timeout: {:?}",
                exec_config.mode, exec_config.timeout
            );
        }

        Ok(Self {
            config: exec_config,
            llm_config: config,
            stats: ExecutionStats::default(),
        })
    }

    /// Create a high-throughput executor with optimized configuration
    #[staticmethod]
    #[pyo3(signature = (llm_config, timeout_seconds=None, debug=None))]
    fn new_high_throughput(
        llm_config: LlmConfig,
        timeout_seconds: Option<u64>,
        debug: Option<bool>,
    ) -> PyResult<Self> {
        let mut config = ExecutionConfig {
            mode: ExecutionMode::HighThroughput,
            max_concurrency: Some(num_cpus::get() * 4), // Higher concurrency
            enable_tracing: debug.unwrap_or(false),     // Default to false
            ..Default::default()
        };

        if let Some(timeout) = timeout_seconds {
            if timeout == 0 || timeout > 3600 {
                return Err(validation_error(
                    "timeout_seconds",
                    Some(&timeout.to_string()),
                    "Timeout must be between 1 and 3600 seconds",
                ));
            }
            config.timeout = Duration::from_secs(timeout);
        }

        Ok(Self {
            config,
            llm_config,
            stats: ExecutionStats::default(),
        })
    }

    /// Create a low-latency executor with optimized configuration
    #[staticmethod]
    #[pyo3(signature = (llm_config, timeout_seconds=None, debug=None))]
    fn new_low_latency(
        llm_config: LlmConfig,
        timeout_seconds: Option<u64>,
        debug: Option<bool>,
    ) -> PyResult<Self> {
        let mut config = ExecutionConfig {
            mode: ExecutionMode::LowLatency,
            timeout: Duration::from_secs(30), // Shorter timeout for low latency
            max_retries: 1,                   // Fewer retries for faster response
            enable_tracing: debug.unwrap_or(false), // Default to false
            ..Default::default()
        };

        if let Some(timeout) = timeout_seconds {
            if timeout == 0 || timeout > 300 {
                return Err(validation_error(
                    "timeout_seconds",
                    Some(&timeout.to_string()),
                    "Low-latency timeout must be between 1 and 300 seconds",
                ));
            }
            config.timeout = Duration::from_secs(timeout);
        }

        Ok(Self {
            config,
            llm_config,
            stats: ExecutionStats::default(),
        })
    }

    /// Create a memory-optimized executor for resource-constrained environments
    #[staticmethod]
    #[pyo3(signature = (llm_config, timeout_seconds=None, debug=None))]
    fn new_memory_optimized(
        llm_config: LlmConfig,
        timeout_seconds: Option<u64>,
        debug: Option<bool>,
    ) -> PyResult<Self> {
        let mut config = ExecutionConfig {
            mode: ExecutionMode::MemoryOptimized,
            max_concurrency: Some(2), // Lower concurrency to save memory
            enable_metrics: false,    // Disable metrics to save memory
            enable_tracing: debug.unwrap_or(false), // Default to false
            ..Default::default()
        };

        if let Some(timeout) = timeout_seconds {
            if timeout == 0 || timeout > 3600 {
                return Err(validation_error(
                    "timeout_seconds",
                    Some(&timeout.to_string()),
                    "Timeout must be between 1 and 3600 seconds",
                ));
            }
            config.timeout = Duration::from_secs(timeout);
        }

        Ok(Self {
            config,
            llm_config,
            stats: ExecutionStats::default(),
        })
    }

    /// Execute a workflow with comprehensive error handling and monitoring
    #[instrument(skip(self, workflow), fields(workflow_name = %workflow.inner.name))]
    fn execute(&mut self, workflow: &Workflow) -> PyResult<WorkflowResult> {
        let start_time = Instant::now();

        // Validate workflow
        if workflow.inner.graph.node_count() == 0 {
            return Err(validation_error(
                "workflow",
                None,
                "Workflow cannot be empty",
            ));
        }

        // Validate the workflow structure
        if let Err(e) = workflow.inner.validate() {
            return Err(validation_error(
                "workflow",
                None,
                &format!("Invalid workflow: {}", e),
            ));
        }

        let llm_config = self.llm_config.inner.clone();
        let workflow_clone = workflow.inner.clone();
        let config = self.config.clone();
        let timeout_duration = config.timeout;
        let debug = config.enable_tracing; // Capture debug flag

        if debug {
            debug!("Starting workflow execution with mode: {:?}", config.mode);
        }

        let result = get_runtime().block_on(async move {
            // Apply timeout to the entire execution
            tokio::time::timeout(timeout_duration, async move {
                Self::execute_workflow_internal(llm_config, workflow_clone, config).await
            })
            .await
        });

        let duration = start_time.elapsed();
        self.update_stats(result.is_ok(), duration);

        match result {
            Ok(Ok(workflow_result)) => {
                if debug {
                    info!(
                        "Workflow execution completed successfully in {:?}",
                        duration
                    );
                }
                Ok(WorkflowResult::new(workflow_result))
            }
            Ok(Err(e)) => {
                if debug {
                    error!("Workflow execution failed: {}", e);
                }
                Err(to_py_runtime_error(e))
            }
            Err(_) => {
                if debug {
                    error!("Workflow execution timed out after {:?}", duration);
                }
                Err(timeout_error(
                    "workflow_execution",
                    duration.as_millis() as u64,
                    &format!("Workflow execution timed out after {:?}", timeout_duration),
                ))
            }
        }
    }

    /// Async execution with enhanced performance optimizations
    #[instrument(skip(self, workflow, py), fields(workflow_name = %workflow.inner.name))]
    fn run_async<'a>(&mut self, workflow: &Workflow, py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
        // Validate workflow
        if let Err(e) = workflow.inner.validate() {
            return Err(validation_error(
                "workflow",
                None,
                &format!("Invalid workflow: {}", e),
            ));
        }

        let workflow_clone = workflow.inner.clone();
        let llm_config = self.llm_config.inner.clone();
        let config = self.config.clone();
        let timeout_duration = config.timeout;
        let start_time = Instant::now();
        let debug = config.enable_tracing; // Capture debug flag

        if debug {
            debug!(
                "Starting async workflow execution with mode: {:?}",
                config.mode
            );
        }

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            // Apply timeout to the entire execution
            let result = tokio::time::timeout(timeout_duration, async move {
                Self::execute_workflow_internal(llm_config, workflow_clone, config).await
            })
            .await;

            match result {
                Ok(Ok(workflow_result)) => {
                    let duration = start_time.elapsed();
                    if debug {
                        info!(
                            "Async workflow execution completed successfully in {:?}",
                            duration
                        );
                    }
                    Ok(WorkflowResult {
                        inner: workflow_result,
                    })
                }
                Ok(Err(e)) => {
                    let duration = start_time.elapsed();
                    if debug {
                        error!(
                            "Async workflow execution failed after {:?}: {}",
                            duration, e
                        );
                    }
                    Err(to_py_runtime_error(e))
                }
                Err(_) => {
                    let duration = start_time.elapsed();
                    if debug {
                        error!("Async workflow execution timed out after {:?}", duration);
                    }
                    Err(timeout_error(
                        "async_workflow_execution",
                        duration.as_millis() as u64,
                        &format!(
                            "Async workflow execution timed out after {:?}",
                            timeout_duration
                        ),
                    ))
                }
            }
        })
    }

    /// Configure the executor with new settings
    #[pyo3(signature = (timeout_seconds=None, max_retries=None, enable_metrics=None, debug=None))]
    fn configure(
        &mut self,
        timeout_seconds: Option<u64>,
        max_retries: Option<u32>,
        enable_metrics: Option<bool>,
        debug: Option<bool>,
    ) -> PyResult<()> {
        // Validate timeout
        if let Some(timeout) = timeout_seconds {
            if timeout == 0 || timeout > 3600 {
                return Err(validation_error(
                    "timeout_seconds",
                    Some(&timeout.to_string()),
                    "Timeout must be between 1 and 3600 seconds",
                ));
            }
            self.config.timeout = Duration::from_secs(timeout);
        }

        // Validate retries
        if let Some(retries) = max_retries {
            if retries == 0 || retries > 10 {
                return Err(validation_error(
                    "max_retries",
                    Some(&retries.to_string()),
                    "Maximum retries must be between 1 and 10",
                ));
            }
            self.config.max_retries = retries;
        }

        if let Some(metrics) = enable_metrics {
            self.config.enable_metrics = metrics;
        }

        if let Some(debug_mode) = debug {
            self.config.enable_tracing = debug_mode;
        }

        if self.config.enable_tracing {
            info!(
                "Executor configuration updated: timeout={:?}, retries={}, metrics={}, debug={}",
                self.config.timeout,
                self.config.max_retries,
                self.config.enable_metrics,
                self.config.enable_tracing
            );
        }

        Ok(())
    }

    /// Get comprehensive execution statistics
    fn get_stats<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let dict = PyDict::new(py);

        dict.set_item("total_executions", self.stats.total_executions)?;
        dict.set_item("successful_executions", self.stats.successful_executions)?;
        dict.set_item("failed_executions", self.stats.failed_executions)?;
        dict.set_item(
            "success_rate",
            if self.stats.total_executions > 0 {
                self.stats.successful_executions as f64 / self.stats.total_executions as f64
            } else {
                0.0
            },
        )?;
        dict.set_item("average_duration_ms", self.stats.average_duration_ms)?;
        dict.set_item("total_duration_ms", self.stats.total_duration_ms)?;
        dict.set_item("uptime_seconds", self.stats.created_at.elapsed().as_secs())?;

        // Configuration info
        dict.set_item("execution_mode", format!("{:?}", self.config.mode))?;
        dict.set_item("timeout_seconds", self.config.timeout.as_secs())?;
        dict.set_item("max_retries", self.config.max_retries)?;
        dict.set_item("metrics_enabled", self.config.enable_metrics)?;

        Ok(dict)
    }

    /// Reset execution statistics
    fn reset_stats(&mut self) -> PyResult<()> {
        self.stats = ExecutionStats::default();
        if self.config.enable_tracing {
            info!("Execution statistics reset");
        }
        Ok(())
    }

    /// Check execution mode
    fn get_execution_mode(&self) -> String {
        format!("{:?}", self.config.mode)
    }

    /// Legacy method for backward compatibility
    fn set_lightweight_mode(&mut self, enabled: bool) {
        self.config.mode = if enabled {
            ExecutionMode::LowLatency
        } else {
            ExecutionMode::HighThroughput
        };
        if self.config.enable_tracing {
            info!("Execution mode changed to: {:?}", self.config.mode);
        }
    }

    /// Legacy method for backward compatibility
    fn is_lightweight_mode(&self) -> bool {
        matches!(self.config.mode, ExecutionMode::LowLatency)
    }
}

impl Executor {
    /// Internal workflow execution with mode-specific optimizations
    async fn execute_workflow_internal(
        llm_config: graphbit_core::llm::LlmConfig,
        workflow: graphbit_core::workflow::Workflow,
        config: ExecutionConfig,
    ) -> Result<graphbit_core::types::WorkflowContext, graphbit_core::errors::GraphBitError> {
        let executor = match config.mode {
            ExecutionMode::HighThroughput => {
                CoreWorkflowExecutor::new_high_throughput().with_default_llm_config(llm_config)
            }
            ExecutionMode::LowLatency => CoreWorkflowExecutor::new_low_latency()
                .with_default_llm_config(llm_config)
                .without_retries()
                .with_fail_fast(true),
            ExecutionMode::MemoryOptimized => {
                CoreWorkflowExecutor::new_high_throughput().with_default_llm_config(llm_config)
            }
            ExecutionMode::Balanced => {
                CoreWorkflowExecutor::new_high_throughput().with_default_llm_config(llm_config)
            }
        };

        executor.execute(workflow).await
    }

    /// Update execution statistics
    fn update_stats(&mut self, success: bool, duration: Duration) {
        if !self.config.enable_metrics {
            return;
        }

        self.stats.total_executions += 1;
        let duration_ms = duration.as_millis() as u64;
        self.stats.total_duration_ms += duration_ms;

        if success {
            self.stats.successful_executions += 1;
        } else {
            self.stats.failed_executions += 1;
        }

        // Update average duration (simple moving average)
        self.stats.average_duration_ms =
            self.stats.total_duration_ms as f64 / self.stats.total_executions as f64;
    }
}
