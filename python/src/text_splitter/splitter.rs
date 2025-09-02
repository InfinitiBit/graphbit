//! Text splitter implementations for GraphBit Python bindings

use crate::errors::to_py_runtime_error;
use graphbit_core::text_splitter::{
    CharacterSplitter as CoreCharacterSplitter, RecursiveSplitter as CoreRecursiveSplitter,
    SentenceSplitter as CoreSentenceSplitter, TextChunk as CoreTextChunk,
    TextSplitterFactory as CoreTextSplitterFactory, TextSplitterTrait,
    TokenSplitter as CoreTokenSplitter,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use super::config::TextSplitterConfig;

/// Text chunk representation
#[pyclass]
#[derive(Clone)]
pub struct TextChunk {
    inner: CoreTextChunk,
}

#[pymethods]
impl TextChunk {
    /// Get the text content
    #[getter]
    fn content(&self) -> String {
        self.inner.content.clone()
    }

    /// Get the start index in the original text
    #[getter]
    fn start_index(&self) -> usize {
        self.inner.start_index
    }

    /// Get the end index in the original text
    #[getter]
    fn end_index(&self) -> usize {
        self.inner.end_index
    }

    /// Get the chunk index in the sequence
    #[getter]
    fn chunk_index(&self) -> usize {
        self.inner.chunk_index
    }

    /// Get the metadata as a dictionary
    #[getter]
    fn metadata(&self, py: Python<'_>) -> PyResult<Py<PyDict>> {
        let dict = PyDict::new(py);

        for (key, value) in &self.inner.metadata {
            let py_value = if let Ok(s) = serde_json::to_string(value) {
                s.trim_matches('"').to_string()
            } else {
                value.to_string()
            };
            dict.set_item(key, py_value)?;
        }

        Ok(dict.into())
    }

    /// String representation
    fn __str__(&self) -> String {
        format!(
            "TextChunk(index={}, start={}, end={}, length={})",
            self.inner.chunk_index,
            self.inner.start_index,
            self.inner.end_index,
            self.inner.content.len()
        )
    }

    /// Representation
    fn __repr__(&self) -> String {
        format!(
            "TextChunk(content='{}...', index={}, start={}, end={})",
            &self.inner.content.chars().take(50).collect::<String>(),
            self.inner.chunk_index,
            self.inner.start_index,
            self.inner.end_index
        )
    }
}

/// Character-based text splitter
#[pyclass]
pub struct CharacterSplitter {
    inner: Box<CoreCharacterSplitter>,
}

#[pymethods]
impl CharacterSplitter {
    /// Create a new character splitter
    #[new]
    #[pyo3(signature = (chunk_size, chunk_overlap=0))]
    fn new(chunk_size: usize, chunk_overlap: usize) -> PyResult<Self> {
        let splitter =
            CoreCharacterSplitter::new(chunk_size, chunk_overlap).map_err(to_py_runtime_error)?;

        Ok(Self {
            inner: Box::new(splitter),
        })
    }

    /// Split text into chunks
    fn split_text(&self, text: &str) -> PyResult<Vec<TextChunk>> {
        let chunks = self.inner.split_text(text).map_err(to_py_runtime_error)?;

        Ok(chunks
            .into_iter()
            .map(|chunk| TextChunk { inner: chunk })
            .collect())
    }

    /// Split a list of texts
    fn split_texts(&self, texts: Vec<String>) -> PyResult<Vec<Vec<TextChunk>>> {
        let mut all_chunks = Vec::new();

        for text in texts {
            let chunks = self.split_text(&text)?;
            all_chunks.push(chunks);
        }

        Ok(all_chunks)
    }

    /// Get the chunk size
    #[getter]
    fn chunk_size(&self) -> usize {
        match &self.inner.config().strategy {
            graphbit_core::text_splitter::SplitterStrategy::Character { chunk_size, .. } => {
                *chunk_size
            }
            _ => 0,
        }
    }

    /// Get the chunk overlap
    #[getter]
    fn chunk_overlap(&self) -> usize {
        match &self.inner.config().strategy {
            graphbit_core::text_splitter::SplitterStrategy::Character { chunk_overlap, .. } => {
                *chunk_overlap
            }
            _ => 0,
        }
    }
}

/// Token-based text splitter
#[pyclass]
pub struct TokenSplitter {
    inner: Box<CoreTokenSplitter>,
}

#[pymethods]
impl TokenSplitter {
    /// Create a new token splitter
    #[new]
    #[pyo3(signature = (chunk_size, chunk_overlap=0, token_pattern=None))]
    fn new(
        chunk_size: usize,
        chunk_overlap: usize,
        token_pattern: Option<String>,
    ) -> PyResult<Self> {
        let splitter = if let Some(pattern) = token_pattern {
            CoreTokenSplitter::with_pattern(chunk_size, chunk_overlap, &pattern)
        } else {
            CoreTokenSplitter::new(chunk_size, chunk_overlap)
        }
        .map_err(to_py_runtime_error)?;

        Ok(Self {
            inner: Box::new(splitter),
        })
    }

    /// Split text into chunks
    fn split_text(&self, text: &str) -> PyResult<Vec<TextChunk>> {
        let chunks = self.inner.split_text(text).map_err(to_py_runtime_error)?;

        Ok(chunks
            .into_iter()
            .map(|chunk| TextChunk { inner: chunk })
            .collect())
    }

    /// Split a list of texts
    fn split_texts(&self, texts: Vec<String>) -> PyResult<Vec<Vec<TextChunk>>> {
        let mut all_chunks = Vec::new();

        for text in texts {
            let chunks = self.split_text(&text)?;
            all_chunks.push(chunks);
        }

        Ok(all_chunks)
    }

    /// Get the chunk size
    #[getter]
    fn chunk_size(&self) -> usize {
        match &self.inner.config().strategy {
            graphbit_core::text_splitter::SplitterStrategy::Token { chunk_size, .. } => *chunk_size,
            _ => 0,
        }
    }

    /// Get the chunk overlap
    #[getter]
    fn chunk_overlap(&self) -> usize {
        match &self.inner.config().strategy {
            graphbit_core::text_splitter::SplitterStrategy::Token { chunk_overlap, .. } => {
                *chunk_overlap
            }
            _ => 0,
        }
    }
}

/// Sentence-based text splitter
#[pyclass]
pub struct SentenceSplitter {
    inner: Box<CoreSentenceSplitter>,
}

#[pymethods]
impl SentenceSplitter {
    /// Create a new sentence splitter
    #[new]
    #[pyo3(signature = (chunk_size, chunk_overlap=0, sentence_endings=None))]
    fn new(
        chunk_size: usize,
        chunk_overlap: usize,
        sentence_endings: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let splitter = if let Some(endings) = sentence_endings {
            let endings_refs: Vec<&str> = endings.iter().map(|s| s.as_str()).collect();
            CoreSentenceSplitter::with_endings(chunk_size, chunk_overlap, endings_refs)
        } else {
            CoreSentenceSplitter::new(chunk_size, chunk_overlap)
        }
        .map_err(to_py_runtime_error)?;

        Ok(Self {
            inner: Box::new(splitter),
        })
    }

    /// Split text into chunks
    fn split_text(&self, text: &str) -> PyResult<Vec<TextChunk>> {
        let chunks = self.inner.split_text(text).map_err(to_py_runtime_error)?;

        Ok(chunks
            .into_iter()
            .map(|chunk| TextChunk { inner: chunk })
            .collect())
    }

    /// Split a list of texts
    fn split_texts(&self, texts: Vec<String>) -> PyResult<Vec<Vec<TextChunk>>> {
        let mut all_chunks = Vec::new();

        for text in texts {
            let chunks = self.split_text(&text)?;
            all_chunks.push(chunks);
        }

        Ok(all_chunks)
    }

    /// Get the chunk size
    #[getter]
    fn chunk_size(&self) -> usize {
        match &self.inner.config().strategy {
            graphbit_core::text_splitter::SplitterStrategy::Sentence { chunk_size, .. } => {
                *chunk_size
            }
            _ => 0,
        }
    }

    /// Get the chunk overlap
    #[getter]
    fn chunk_overlap(&self) -> usize {
        match &self.inner.config().strategy {
            graphbit_core::text_splitter::SplitterStrategy::Sentence { chunk_overlap, .. } => {
                *chunk_overlap
            }
            _ => 0,
        }
    }
}

/// Recursive text splitter
#[pyclass]
pub struct RecursiveSplitter {
    inner: Box<CoreRecursiveSplitter>,
}

#[pymethods]
impl RecursiveSplitter {
    /// Create a new recursive splitter
    #[new]
    #[pyo3(signature = (chunk_size, chunk_overlap=0, separators=None))]
    fn new(
        chunk_size: usize,
        chunk_overlap: usize,
        separators: Option<Vec<String>>,
    ) -> PyResult<Self> {
        let splitter = if let Some(seps) = separators {
            CoreRecursiveSplitter::with_separators(chunk_size, chunk_overlap, seps)
        } else {
            CoreRecursiveSplitter::new(chunk_size, chunk_overlap)
        }
        .map_err(to_py_runtime_error)?;

        Ok(Self {
            inner: Box::new(splitter),
        })
    }

    /// Split text into chunks
    fn split_text(&self, text: &str) -> PyResult<Vec<TextChunk>> {
        let chunks = self.inner.split_text(text).map_err(to_py_runtime_error)?;

        Ok(chunks
            .into_iter()
            .map(|chunk| TextChunk { inner: chunk })
            .collect())
    }

    /// Split a list of texts
    fn split_texts(&self, texts: Vec<String>) -> PyResult<Vec<Vec<TextChunk>>> {
        let mut all_chunks = Vec::new();

        for text in texts {
            let chunks = self.split_text(&text)?;
            all_chunks.push(chunks);
        }

        Ok(all_chunks)
    }

    /// Get the chunk size
    #[getter]
    fn chunk_size(&self) -> usize {
        match &self.inner.config().strategy {
            graphbit_core::text_splitter::SplitterStrategy::Recursive { chunk_size, .. } => {
                *chunk_size
            }
            _ => 0,
        }
    }

    /// Get the chunk overlap
    #[getter]
    fn chunk_overlap(&self) -> usize {
        match &self.inner.config().strategy {
            graphbit_core::text_splitter::SplitterStrategy::Recursive { chunk_overlap, .. } => {
                *chunk_overlap
            }
            _ => 0,
        }
    }

    /// Get the separators
    #[getter]
    fn separators(&self) -> Vec<String> {
        match &self.inner.config().strategy {
            graphbit_core::text_splitter::SplitterStrategy::Recursive { separators, .. } => {
                separators.clone().unwrap_or_default()
            }
            _ => Vec::new(),
        }
    }
}

/// Generic text splitter that can use any strategy
#[pyclass]
pub struct TextSplitter {
    inner: Box<dyn TextSplitterTrait + Send + Sync>,
}

#[pymethods]
impl TextSplitter {
    /// Create a text splitter from configuration
    #[new]
    fn new(config: TextSplitterConfig) -> PyResult<Self> {
        let splitter =
            CoreTextSplitterFactory::create_splitter(config.inner).map_err(to_py_runtime_error)?;

        Ok(Self { inner: splitter })
    }

    /// Split text into chunks
    fn split_text(&self, text: &str) -> PyResult<Vec<TextChunk>> {
        let chunks = self.inner.split_text(text).map_err(to_py_runtime_error)?;

        Ok(chunks
            .into_iter()
            .map(|chunk| TextChunk { inner: chunk })
            .collect())
    }

    /// Split a list of texts
    fn split_texts(&self, texts: Vec<String>) -> PyResult<Vec<Vec<TextChunk>>> {
        let mut all_chunks = Vec::new();

        for text in texts {
            let chunks = self.split_text(&text)?;
            all_chunks.push(chunks);
        }

        Ok(all_chunks)
    }

    /// Create chunks from text and return as list of dictionaries
    fn create_documents(&self, text: &str) -> PyResult<Vec<HashMap<String, String>>> {
        let chunks = self.split_text(text)?;

        Ok(chunks
            .into_iter()
            .map(|chunk| {
                let mut doc = HashMap::new();
                doc.insert("content".to_string(), chunk.content());
                doc.insert("start_index".to_string(), chunk.start_index().to_string());
                doc.insert("end_index".to_string(), chunk.end_index().to_string());
                doc.insert("chunk_index".to_string(), chunk.chunk_index().to_string());
                doc
            })
            .collect())
    }
}
