//! Configuration and shared types for text splitters.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for text splitting operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSplitterConfig {
    /// Strategy to use for splitting
    pub strategy: SplitterStrategy,
    /// Whether to preserve word boundaries
    pub preserve_word_boundaries: bool,
    /// Whether to trim whitespace from chunks
    pub trim_whitespace: bool,
    /// Metadata to include with each chunk
    pub include_metadata: bool,
    /// Additional strategy-specific parameters
    pub extra_params: HashMap<String, serde_json::Value>,
}

impl Default for TextSplitterConfig {
    fn default() -> Self {
        Self {
            strategy: SplitterStrategy::Character {
                chunk_size: 1000,
                chunk_overlap: 200,
            },
            preserve_word_boundaries: true,
            trim_whitespace: true,
            include_metadata: true,
            extra_params: HashMap::new(),
        }
    }
}

/// Different text splitting strategies
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SplitterStrategy {
    /// Split by character count
    Character {
        /// Maximum size of each chunk
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
    },
    /// Split by token count (word-based)
    Token {
        /// Maximum number of tokens per chunk
        chunk_size: usize,
        /// Number of tokens to overlap between chunks
        chunk_overlap: usize,
        /// Optional regex pattern for tokenization
        token_pattern: Option<String>,
    },
    /// Split by sentence boundaries
    Sentence {
        /// Maximum number of sentences per chunk
        chunk_size: usize,
        /// Number of sentences to overlap between chunks
        chunk_overlap: usize,
        /// Optional custom sentence ending markers
        sentence_endings: Option<Vec<String>>,
    },
    /// Recursive splitting with multiple separators
    Recursive {
        /// Maximum size of each chunk
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
        /// Optional list of separators to try in order
        separators: Option<Vec<String>>,
    },
    /// Split by paragraphs
    Paragraph {
        /// Maximum size of each chunk
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
        /// Minimum length to consider as a paragraph
        min_paragraph_length: Option<usize>,
    },
    /// Split by semantic similarity (requires embeddings)
    Semantic {
        /// Maximum size of each chunk
        max_chunk_size: usize,
        /// Similarity threshold for grouping sentences
        similarity_threshold: f32,
    },
    /// Split Markdown documents preserving structure
    Markdown {
        /// Maximum size of each chunk
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
        /// Whether to split at header boundaries
        split_by_headers: bool,
    },
    /// Split code files preserving syntax
    Code {
        /// Maximum size of each chunk
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
        /// Programming language for syntax-aware splitting
        language: Option<String>,
    },
    /// Custom regex-based splitting
    Regex {
        /// Regex pattern to match split boundaries
        pattern: String,
        /// Maximum size of each chunk
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
    },
}

/// A text chunk with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChunk {
    /// The text content of the chunk
    pub content: String,
    /// Start position in the original text
    pub start_index: usize,
    /// End position in the original text
    pub end_index: usize,
    /// Chunk index in the sequence
    pub chunk_index: usize,
    /// Metadata about the chunk
    pub metadata: HashMap<String, serde_json::Value>,
}

impl TextChunk {
    /// Create a new text chunk
    pub fn new(content: String, start_index: usize, end_index: usize, chunk_index: usize) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert(
            "length".to_string(),
            serde_json::Value::Number(content.chars().count().into()),
        );

        Self {
            content,
            start_index,
            end_index,
            chunk_index,
            metadata,
        }
    }

    /// Add metadata to the chunk
    pub fn with_metadata(mut self, key: String, value: serde_json::Value) -> Self {
        self.metadata.insert(key, value);
        self
    }
}
