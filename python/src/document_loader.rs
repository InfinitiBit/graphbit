//! Python bindings for document loading functionality.
//!
//! This module provides Python bindings for `GraphBit`'s document loader,
//! supporting multiple formats including PDF, DOCX, TXT, JSON, CSV, XML, and HTML.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use graphbit_core::{
    document_loader::{DocumentContent, DocumentLoader, DocumentLoaderConfig},
    GraphBitResult,
};

use crate::errors::to_py_error;
use crate::runtime::get_runtime;

/// Python wrapper for DocumentLoaderConfig
#[pyclass(name = "DocumentLoaderConfig")]
#[derive(Clone)]
pub struct PyDocumentLoaderConfig {
    pub(crate) inner: DocumentLoaderConfig,
}

#[pymethods]
impl PyDocumentLoaderConfig {
    /// Create a new DocumentLoaderConfig with default settings
    #[new]
    #[pyo3(signature = (
        max_file_size=None,
        default_encoding=None,
        preserve_formatting=None
    ))]
    fn new(
        max_file_size: Option<usize>,
        default_encoding: Option<String>,
        preserve_formatting: Option<bool>,
    ) -> PyResult<Self> {
        let mut config = DocumentLoaderConfig::default();

        if let Some(size) = max_file_size {
            if size == 0 {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "max_file_size must be greater than 0",
                ));
            }
            config.max_file_size = size;
        }

        if let Some(encoding) = default_encoding {
            if encoding.trim().is_empty() {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "default_encoding cannot be empty",
                ));
            }
            config.default_encoding = encoding;
        }

        if let Some(preserve) = preserve_formatting {
            config.preserve_formatting = preserve;
        }

        Ok(Self { inner: config })
    }

    /// Get the maximum file size
    #[getter]
    fn max_file_size(&self) -> usize {
        self.inner.max_file_size
    }

    /// Set the maximum file size
    #[setter]
    fn set_max_file_size(&mut self, size: usize) -> PyResult<()> {
        if size == 0 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "max_file_size must be greater than 0",
            ));
        }
        self.inner.max_file_size = size;
        Ok(())
    }

    /// Get the default encoding
    #[getter]
    fn default_encoding(&self) -> String {
        self.inner.default_encoding.clone()
    }

    /// Set the default encoding
    #[setter]
    fn set_default_encoding(&mut self, encoding: String) -> PyResult<()> {
        if encoding.trim().is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "default_encoding cannot be empty",
            ));
        }
        self.inner.default_encoding = encoding;
        Ok(())
    }

    /// Get the preserve formatting flag
    #[getter]
    fn preserve_formatting(&self) -> bool {
        self.inner.preserve_formatting
    }

    /// Set the preserve formatting flag
    #[setter]
    fn set_preserve_formatting(&mut self, preserve: bool) {
        self.inner.preserve_formatting = preserve;
    }

    /// Get extraction settings as a dictionary
    #[getter]
    fn extraction_settings<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (key, value) in &self.inner.extraction_settings {
            let py_value = serde_json_to_py_object(py, value)?;
            dict.set_item(key, py_value)?;
        }
        Ok(dict)
    }

    /// Set extraction settings from a dictionary
    #[setter]
    fn set_extraction_settings(
        &mut self,
        py: Python<'_>,
        settings: Bound<'_, PyDict>,
    ) -> PyResult<()> {
        let mut new_settings = HashMap::new();
        for (key, value) in settings.iter() {
            let key_str: String = key.extract()?;
            let json_value = py_object_to_serde_json(py, &value)?;
            new_settings.insert(key_str, json_value);
        }
        self.inner.extraction_settings = new_settings;
        Ok(())
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "DocumentLoaderConfig(max_file_size={}, default_encoding='{}', preserve_formatting={})",
            self.inner.max_file_size, self.inner.default_encoding, self.inner.preserve_formatting
        )
    }
}

/// Python wrapper for DocumentContent
#[pyclass(name = "DocumentContent")]
#[derive(Clone)]
pub struct PyDocumentContent {
    pub(crate) inner: DocumentContent,
}

#[pymethods]
impl PyDocumentContent {
    /// Get the source path or URL
    #[getter]
    fn source(&self) -> String {
        self.inner.source.clone()
    }

    /// Get the document type
    #[getter]
    fn document_type(&self) -> String {
        self.inner.document_type.clone()
    }

    /// Get the extracted content
    #[getter]
    fn content(&self) -> String {
        self.inner.content.clone()
    }

    /// Get the file size in bytes
    #[getter]
    fn file_size(&self) -> usize {
        self.inner.file_size
    }

    /// Get the extraction timestamp as a UTC timestamp
    #[getter]
    fn extracted_at(&self) -> i64 {
        self.inner.extracted_at.timestamp()
    }

    /// Get metadata as a dictionary
    #[getter]
    fn metadata<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (key, value) in &self.inner.metadata {
            let py_value = serde_json_to_py_object(py, value)?;
            dict.set_item(key, py_value)?;
        }
        Ok(dict)
    }

    /// Get content length
    fn content_length(&self) -> usize {
        self.inner.content.len()
    }

    /// Check if content is empty
    fn is_empty(&self) -> bool {
        self.inner.content.trim().is_empty()
    }

    /// Get a preview of the content (first n characters)
    #[pyo3(signature = (max_length=500))]
    fn preview(&self, max_length: usize) -> String {
        if self.inner.content.len() <= max_length {
            self.inner.content.clone()
        } else {
            format!("{}...", &self.inner.content[..max_length])
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "DocumentContent(source='{}', type='{}', size={} bytes, content_length={})",
            self.inner.source,
            self.inner.document_type,
            self.inner.file_size,
            self.inner.content.len()
        )
    }
}

/// Python wrapper for DocumentLoader
#[pyclass(name = "DocumentLoader")]
pub struct PyDocumentLoader {
    loader: DocumentLoader,
}

#[pymethods]
impl PyDocumentLoader {
    /// Create a new DocumentLoader with default configuration
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<PyDocumentLoaderConfig>) -> Self {
        let loader = match config {
            Some(cfg) => DocumentLoader::with_config(cfg.inner),
            None => DocumentLoader::new(),
        };
        Self { loader }
    }

    /// Load and extract content from a document
    ///
    /// Args:
    ///     source_path: Path to the document file or URL
    ///     document_type: Type of document (txt, pdf, docx, json, csv, xml, html)
    ///
    /// Returns:
    ///     DocumentContent: The extracted content and metadata
    fn load_document(
        &self,
        py: Python<'_>,
        source_path: String,
        document_type: String,
    ) -> PyResult<PyDocumentContent> {
        let rt = get_runtime();

        // Validate inputs
        if source_path.trim().is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "source_path cannot be empty",
            ));
        }

        if document_type.trim().is_empty() {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "document_type cannot be empty",
            ));
        }

        let future = async {
            self.loader
                .load_document(&source_path, &document_type)
                .await
        };

        let result: GraphBitResult<DocumentContent> = py.allow_threads(|| rt.block_on(future));

        match result {
            Ok(content) => Ok(PyDocumentContent { inner: content }),
            Err(e) => Err(to_py_error(e)),
        }
    }

    /// Get list of supported document types
    #[staticmethod]
    fn supported_types() -> Vec<String> {
        DocumentLoader::supported_types()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    /// Detect document type from file extension
    #[staticmethod]
    fn detect_document_type(file_path: String) -> Option<String> {
        graphbit_core::document_loader::detect_document_type(&file_path)
    }

    /// Validate document source and type
    #[staticmethod]
    fn validate_document_source(source_path: String, document_type: String) -> PyResult<()> {
        match graphbit_core::document_loader::validate_document_source(&source_path, &document_type)
        {
            Ok(_) => Ok(()),
            Err(e) => Err(to_py_error(e)),
        }
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "DocumentLoader(supported_types={:?})",
            Self::supported_types()
        )
    }
}

/// Helper function to convert serde_json::Value to Python object
fn serde_json_to_py_object(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => {
            let py_bool = b.into_pyobject(py)?;
            Ok(
                <pyo3::Bound<'_, pyo3::types::PyBool> as Clone>::clone(&py_bool)
                    .into_any()
                    .unbind(),
            )
        }
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any().unbind())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any().unbind())
            } else {
                Ok(n.to_string().into_pyobject(py)?.into_any().unbind())
            }
        }
        serde_json::Value::String(s) => Ok(s.into_pyobject(py)?.into_any().unbind()),
        serde_json::Value::Array(arr) => {
            let py_list = pyo3::types::PyList::empty(py);
            for item in arr {
                let py_item = serde_json_to_py_object(py, item)?;
                py_list.append(py_item)?;
            }
            Ok(py_list.into_any().unbind())
        }
        serde_json::Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (key, val) in obj {
                let py_val = serde_json_to_py_object(py, val)?;
                py_dict.set_item(key, py_val)?;
            }
            Ok(py_dict.into_any().unbind())
        }
    }
}

/// Helper function to convert Python object to serde_json::Value
fn py_object_to_serde_json(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    if obj.is_none() {
        Ok(serde_json::Value::Null)
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(serde_json::Value::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(serde_json::Value::Number(serde_json::Number::from(i)))
    } else if let Ok(f) = obj.extract::<f64>() {
        if let Some(n) = serde_json::Number::from_f64(f) {
            Ok(serde_json::Value::Number(n))
        } else {
            Err(pyo3::exceptions::PyValueError::new_err(
                "Invalid floating point number",
            ))
        }
    } else if let Ok(s) = obj.extract::<String>() {
        Ok(serde_json::Value::String(s))
    } else if let Ok(list) = obj.downcast::<pyo3::types::PyList>() {
        let mut vec = Vec::new();
        for item in list.iter() {
            vec.push(py_object_to_serde_json(py, &item)?);
        }
        Ok(serde_json::Value::Array(vec))
    } else if let Ok(dict) = obj.downcast::<pyo3::types::PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str: String = key.extract()?;
            let json_value = py_object_to_serde_json(py, &value)?;
            map.insert(key_str, json_value);
        }
        Ok(serde_json::Value::Object(map))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Unsupported type for JSON conversion",
        ))
    }
}
