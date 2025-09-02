//! Text splitter configuration for GraphBit Python bindings

use graphbit_core::text_splitter::{
    SplitterStrategy as CoreSplitterStrategy, TextSplitterConfig as CoreTextSplitterConfig,
};
use pyo3::prelude::*;
use std::collections::HashMap;

/// Text splitter configuration
#[pyclass]
#[derive(Clone)]
pub struct TextSplitterConfig {
    pub(crate) inner: CoreTextSplitterConfig,
}

#[pymethods]
impl TextSplitterConfig {
    /// Create a character-based splitter configuration
    #[staticmethod]
    #[pyo3(signature = (chunk_size, chunk_overlap=0))]
    fn character(chunk_size: usize, chunk_overlap: usize) -> PyResult<Self> {
        if chunk_size == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk size must be greater than 0",
            ));
        }

        if chunk_overlap >= chunk_size {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk overlap must be less than chunk size",
            ));
        }

        Ok(Self {
            inner: CoreTextSplitterConfig {
                strategy: CoreSplitterStrategy::Character {
                    chunk_size,
                    chunk_overlap,
                },
                preserve_word_boundaries: true,
                trim_whitespace: true,
                include_metadata: true,
                extra_params: HashMap::new(),
            },
        })
    }

    /// Create a token-based splitter configuration
    #[staticmethod]
    #[pyo3(signature = (chunk_size, chunk_overlap=0, token_pattern=None))]
    fn token(
        chunk_size: usize,
        chunk_overlap: usize,
        token_pattern: Option<String>,
    ) -> PyResult<Self> {
        if chunk_size == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk size must be greater than 0",
            ));
        }

        if chunk_overlap >= chunk_size {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk overlap must be less than chunk size",
            ));
        }

        Ok(Self {
            inner: CoreTextSplitterConfig {
                strategy: CoreSplitterStrategy::Token {
                    chunk_size,
                    chunk_overlap,
                    token_pattern,
                },
                preserve_word_boundaries: true,
                trim_whitespace: true,
                include_metadata: true,
                extra_params: HashMap::new(),
            },
        })
    }

    /// Create a sentence-based splitter configuration
    #[staticmethod]
    #[pyo3(signature = (chunk_size, chunk_overlap=0, sentence_endings=None))]
    fn sentence(
        chunk_size: usize,
        chunk_overlap: usize,
        sentence_endings: Option<Vec<String>>,
    ) -> PyResult<Self> {
        if chunk_size == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk size must be greater than 0",
            ));
        }

        Ok(Self {
            inner: CoreTextSplitterConfig {
                strategy: CoreSplitterStrategy::Sentence {
                    chunk_size,
                    chunk_overlap,
                    sentence_endings,
                },
                preserve_word_boundaries: true,
                trim_whitespace: true,
                include_metadata: true,
                extra_params: HashMap::new(),
            },
        })
    }

    /// Create a recursive splitter configuration
    #[staticmethod]
    #[pyo3(signature = (chunk_size, chunk_overlap=0, separators=None))]
    fn recursive(
        chunk_size: usize,
        chunk_overlap: usize,
        separators: Option<Vec<String>>,
    ) -> PyResult<Self> {
        if chunk_size == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk size must be greater than 0",
            ));
        }

        Ok(Self {
            inner: CoreTextSplitterConfig {
                strategy: CoreSplitterStrategy::Recursive {
                    chunk_size,
                    chunk_overlap,
                    separators,
                },
                preserve_word_boundaries: true,
                trim_whitespace: true,
                include_metadata: true,
                extra_params: HashMap::new(),
            },
        })
    }

    /// Create a paragraph-based splitter configuration
    #[staticmethod]
    #[pyo3(signature = (chunk_size, chunk_overlap=0, min_paragraph_length=None))]
    fn paragraph(
        chunk_size: usize,
        chunk_overlap: usize,
        min_paragraph_length: Option<usize>,
    ) -> PyResult<Self> {
        if chunk_size == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk size must be greater than 0",
            ));
        }

        Ok(Self {
            inner: CoreTextSplitterConfig {
                strategy: CoreSplitterStrategy::Paragraph {
                    chunk_size,
                    chunk_overlap,
                    min_paragraph_length,
                },
                preserve_word_boundaries: true,
                trim_whitespace: true,
                include_metadata: true,
                extra_params: HashMap::new(),
            },
        })
    }

    /// Create a Markdown splitter configuration
    #[staticmethod]
    #[pyo3(signature = (chunk_size, chunk_overlap=0, split_by_headers=true))]
    fn markdown(chunk_size: usize, chunk_overlap: usize, split_by_headers: bool) -> PyResult<Self> {
        if chunk_size == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk size must be greater than 0",
            ));
        }

        Ok(Self {
            inner: CoreTextSplitterConfig {
                strategy: CoreSplitterStrategy::Markdown {
                    chunk_size,
                    chunk_overlap,
                    split_by_headers,
                },
                preserve_word_boundaries: true,
                trim_whitespace: true,
                include_metadata: true,
                extra_params: HashMap::new(),
            },
        })
    }

    /// Create a code splitter configuration
    #[staticmethod]
    #[pyo3(signature = (chunk_size, chunk_overlap=0, language=None))]
    fn code(chunk_size: usize, chunk_overlap: usize, language: Option<String>) -> PyResult<Self> {
        if chunk_size == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk size must be greater than 0",
            ));
        }

        Ok(Self {
            inner: CoreTextSplitterConfig {
                strategy: CoreSplitterStrategy::Code {
                    chunk_size,
                    chunk_overlap,
                    language,
                },
                preserve_word_boundaries: true,
                trim_whitespace: false, // Don't trim whitespace for code
                include_metadata: true,
                extra_params: HashMap::new(),
            },
        })
    }

    /// Create a regex-based splitter configuration
    #[staticmethod]
    #[pyo3(signature = (pattern, chunk_size, chunk_overlap=0))]
    fn regex(pattern: String, chunk_size: usize, chunk_overlap: usize) -> PyResult<Self> {
        if chunk_size == 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Chunk size must be greater than 0",
            ));
        }

        if pattern.is_empty() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Pattern cannot be empty",
            ));
        }

        Ok(Self {
            inner: CoreTextSplitterConfig {
                strategy: CoreSplitterStrategy::Regex {
                    pattern,
                    chunk_size,
                    chunk_overlap,
                },
                preserve_word_boundaries: true,
                trim_whitespace: true,
                include_metadata: true,
                extra_params: HashMap::new(),
            },
        })
    }

    /// Set whether to preserve word boundaries
    fn set_preserve_word_boundaries(&mut self, preserve: bool) {
        self.inner.preserve_word_boundaries = preserve;
    }

    /// Set whether to trim whitespace from chunks
    fn set_trim_whitespace(&mut self, trim: bool) {
        self.inner.trim_whitespace = trim;
    }

    /// Set whether to include metadata with chunks
    fn set_include_metadata(&mut self, include: bool) {
        self.inner.include_metadata = include;
    }

    /// Get preserve word boundaries setting
    #[getter]
    fn preserve_word_boundaries(&self) -> bool {
        self.inner.preserve_word_boundaries
    }

    /// Get trim whitespace setting
    #[getter]
    fn trim_whitespace(&self) -> bool {
        self.inner.trim_whitespace
    }

    /// Get include metadata setting
    #[getter]
    fn include_metadata(&self) -> bool {
        self.inner.include_metadata
    }

    /// Get the strategy type as string
    #[getter]
    fn strategy_type(&self) -> String {
        match &self.inner.strategy {
            CoreSplitterStrategy::Character { .. } => "character".to_string(),
            CoreSplitterStrategy::Token { .. } => "token".to_string(),
            CoreSplitterStrategy::Sentence { .. } => "sentence".to_string(),
            CoreSplitterStrategy::Recursive { .. } => "recursive".to_string(),
            CoreSplitterStrategy::Paragraph { .. } => "paragraph".to_string(),
            CoreSplitterStrategy::Semantic { .. } => "semantic".to_string(),
            CoreSplitterStrategy::Markdown { .. } => "markdown".to_string(),
            CoreSplitterStrategy::Code { .. } => "code".to_string(),
            CoreSplitterStrategy::Regex { .. } => "regex".to_string(),
        }
    }

    /// Get chunk size
    #[getter]
    fn chunk_size(&self) -> usize {
        match &self.inner.strategy {
            CoreSplitterStrategy::Character { chunk_size, .. }
            | CoreSplitterStrategy::Token { chunk_size, .. }
            | CoreSplitterStrategy::Sentence { chunk_size, .. }
            | CoreSplitterStrategy::Recursive { chunk_size, .. }
            | CoreSplitterStrategy::Paragraph { chunk_size, .. }
            | CoreSplitterStrategy::Markdown { chunk_size, .. }
            | CoreSplitterStrategy::Code { chunk_size, .. }
            | CoreSplitterStrategy::Regex { chunk_size, .. } => *chunk_size,
            CoreSplitterStrategy::Semantic { max_chunk_size, .. } => *max_chunk_size,
        }
    }

    /// Get chunk overlap
    #[getter]
    fn chunk_overlap(&self) -> usize {
        match &self.inner.strategy {
            CoreSplitterStrategy::Character { chunk_overlap, .. }
            | CoreSplitterStrategy::Token { chunk_overlap, .. }
            | CoreSplitterStrategy::Sentence { chunk_overlap, .. }
            | CoreSplitterStrategy::Recursive { chunk_overlap, .. }
            | CoreSplitterStrategy::Paragraph { chunk_overlap, .. }
            | CoreSplitterStrategy::Markdown { chunk_overlap, .. }
            | CoreSplitterStrategy::Code { chunk_overlap, .. }
            | CoreSplitterStrategy::Regex { chunk_overlap, .. } => *chunk_overlap,
            CoreSplitterStrategy::Semantic { .. } => 0,
        }
    }

    /// String representation
    fn __str__(&self) -> String {
        format!(
            "TextSplitterConfig(strategy={}, chunk_size={}, chunk_overlap={})",
            self.strategy_type(),
            self.chunk_size(),
            self.chunk_overlap()
        )
    }

    /// Representation
    fn __repr__(&self) -> String {
        self.__str__()
    }
}
