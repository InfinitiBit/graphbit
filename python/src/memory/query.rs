//! Memory query for GraphBit Python bindings

use super::types::MemoryType;
use graphbit_core::memory::types::MemoryQuery as CoreMemoryQuery;
use pyo3::prelude::*;

/// Query for retrieving memories
#[pyclass]
#[derive(Clone)]
pub struct MemoryQuery {
    pub(crate) inner: CoreMemoryQuery,
}

#[pymethods]
impl MemoryQuery {
    /// Create a new memory query
    #[new]
    #[pyo3(signature = (
        query_text,
        limit=None,
        memory_types=None,
        session_id=None,
        tags=None,
        min_similarity=None
    ))]
    fn new(
        query_text: String,
        limit: Option<usize>,
        memory_types: Option<Vec<MemoryType>>,
        session_id: Option<String>,
        tags: Option<Vec<String>>,
        min_similarity: Option<f32>,
    ) -> Self {
        let mut query = CoreMemoryQuery::new(query_text);

        if let Some(limit) = limit {
            query.limit = limit;
        }

        if let Some(types) = memory_types {
            query.memory_types = Some(types.into_iter().map(|t| t.into()).collect());
        }

        if let Some(session) = session_id {
            query.session_id = Some(session);
        }

        if let Some(tags) = tags {
            query.tags = Some(tags);
        }

        if let Some(similarity) = min_similarity {
            query.min_similarity = similarity;
        }

        Self { inner: query }
    }

    /// Set the query limit
    fn with_limit(mut slf: PyRefMut<'_, Self>, limit: usize) -> PyRefMut<'_, Self> {
        slf.inner.limit = limit;
        slf
    }

    /// Set memory types filter
    fn with_types(mut slf: PyRefMut<'_, Self>, types: Vec<MemoryType>) -> PyRefMut<'_, Self> {
        slf.inner.memory_types = Some(types.into_iter().map(|t| t.into()).collect());
        slf
    }

    /// Set session ID filter
    fn with_session(mut slf: PyRefMut<'_, Self>, session_id: String) -> PyRefMut<'_, Self> {
        slf.inner.session_id = Some(session_id);
        slf
    }

    /// Set tags filter
    fn with_tags(mut slf: PyRefMut<'_, Self>, tags: Vec<String>) -> PyRefMut<'_, Self> {
        slf.inner.tags = Some(tags);
        slf
    }

    /// Set minimum similarity threshold
    fn with_min_similarity(mut slf: PyRefMut<'_, Self>, similarity: f32) -> PyRefMut<'_, Self> {
        slf.inner.min_similarity = similarity;
        slf
    }

    /// Get the query text
    #[getter]
    fn query_text(&self) -> String {
        self.inner.query.clone()
    }

    /// Get the limit
    #[getter]
    fn limit(&self) -> usize {
        self.inner.limit
    }

    /// Get session ID if set
    #[getter]
    fn session_id(&self) -> Option<String> {
        self.inner.session_id.clone()
    }

    /// Get tags if set
    #[getter]
    fn tags(&self) -> Option<Vec<String>> {
        self.inner.tags.clone()
    }

    /// Get minimum similarity if set
    #[getter]
    fn min_similarity(&self) -> f32 {
        self.inner.min_similarity
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "MemoryQuery(query='{}', limit={})",
            self.query_text(),
            self.limit()
        )
    }
}
