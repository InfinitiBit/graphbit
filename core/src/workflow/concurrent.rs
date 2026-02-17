//! Concurrent task execution with retry logic.

use futures::future::join_all;
use std::sync::Arc;

use crate::errors::{GraphBitError, GraphBitResult};
use crate::types::{ConcurrencyManager, NodeId, RetryConfig, TaskInfo};

/// Execute concurrent tasks with retry logic
pub async fn execute_concurrent_tasks_with_retry<T, F, R>(
    tasks: Vec<T>,
    task_fn: F,
    retry_config: Option<RetryConfig>,
    max_execution_time_ms: Option<u64>,
    concurrency_manager: Arc<ConcurrencyManager>,
) -> GraphBitResult<Vec<Result<R, GraphBitError>>>
where
    T: Send + Clone + 'static,
    F: Fn(T) -> futures::future::BoxFuture<'static, GraphBitResult<R>>
        + Send
        + Sync
        + Clone
        + 'static,
    R: Send + 'static,
{
    if tasks.is_empty() {
        return Ok(Vec::new());
    }

    let task_futures: Vec<_> = tasks
        .into_iter()
        .enumerate()
        .map(|(index, task)| {
            let task_fn = task_fn.clone();
            let max_execution_time = max_execution_time_ms;
            let retry_config = retry_config.clone();
            let concurrency_manager = concurrency_manager.clone();

            tokio::spawn(async move {
                let task_info = TaskInfo {
                    node_type: "concurrent_task".to_string(),
                    task_id: NodeId::new(),
                };

                let _permits = concurrency_manager
                    .acquire_permits(&task_info)
                    .await
                    .map_err(|e| {
                        GraphBitError::workflow_execution(format!(
                            "Failed to acquire permits for concurrent task {index}: {e}",
                        ))
                    })?;

                execute_task_with_retry(task, task_fn, retry_config, max_execution_time).await
            })
        })
        .collect();

    let mut results = Vec::with_capacity(task_futures.len());
    let join_results = join_all(task_futures).await;

    for join_result in join_results {
        match join_result {
            Ok(task_result) => results.push(task_result),
            Err(e) => results.push(Err(GraphBitError::workflow_execution(format!(
                "Task join failed: {e}",
            )))),
        }
    }

    Ok(results)
}

/// Execute a single task with retry logic
pub async fn execute_task_with_retry<T, F, R>(
    task: T,
    task_fn: F,
    retry_config: Option<RetryConfig>,
    max_execution_time: Option<u64>,
) -> Result<R, GraphBitError>
where
    T: Send + Clone + 'static,
    F: Fn(T) -> futures::future::BoxFuture<'static, GraphBitResult<R>>
        + Send
        + Sync
        + Clone
        + 'static,
    R: Send + 'static,
{
    let mut attempt = 0;
    let max_attempts = retry_config.as_ref().map(|c| c.max_attempts).unwrap_or(1);

    loop {
        let task_to_execute = task.clone();

        let result = if let Some(timeout_ms) = max_execution_time {
            let task_future = task_fn(task_to_execute);
            let timeout_duration = tokio::time::Duration::from_millis(timeout_ms);

            match tokio::time::timeout(timeout_duration, task_future).await {
                Ok(result) => result,
                Err(_) => Err(GraphBitError::workflow_execution(format!(
                    "Task execution timed out after {timeout_ms}ms",
                ))),
            }
        } else {
            task_fn(task_to_execute).await
        };

        match result {
            Ok(output) => return Ok(output),
            Err(error) => {
                attempt += 1;

                if let Some(ref config) = retry_config {
                    if attempt < max_attempts && config.should_retry(&error, attempt - 1) {
                        let delay_ms = config.calculate_delay(attempt - 1);
                        if delay_ms > 0 {
                            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms))
                                .await;
                        }

                        continue;
                    }
                }

                return Err(GraphBitError::workflow_execution(format!(
                    "Task failed after {attempt} attempts: {error}",
                )));
            }
        }
    }
}
