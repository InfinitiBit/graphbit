//! Text splitter bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::text_splitter::{
    TextSplitterFactory,
    TextSplitterTrait,
    TextSplitterConfig as CoreTextSplitterConfig,
    SplitterStrategy,
};
use std::collections::HashMap;



/// Text splitter
#[napi]
pub struct TextSplitter {
    splitter: Box<dyn TextSplitterTrait>,
}

#[napi]
impl TextSplitter {
    /// Create a character-based text splitter
    #[napi(factory)]
    pub fn character(chunk_size: u32, chunk_overlap: Option<u32>) -> Result<Self> {
        let config = CoreTextSplitterConfig {
            strategy: SplitterStrategy::Character {
                chunk_size: chunk_size as usize,
                chunk_overlap: chunk_overlap.unwrap_or(0) as usize,
            },
            preserve_word_boundaries: true,
            trim_whitespace: true,
            include_metadata: true,
            extra_params: HashMap::new(),
        };

        let splitter = TextSplitterFactory::create_splitter(config)
            .map_err(crate::errors::to_napi_error)?;

        Ok(Self { splitter })
    }

    /// Create a recursive text splitter
    #[napi(factory)]
    pub fn recursive(chunk_size: u32, chunk_overlap: Option<u32>) -> Result<Self> {
        let config = CoreTextSplitterConfig {
            strategy: SplitterStrategy::Recursive {
                chunk_size: chunk_size as usize,
                chunk_overlap: chunk_overlap.unwrap_or(0) as usize,
                separators: None,
            },
            preserve_word_boundaries: true,
            trim_whitespace: true,
            include_metadata: true,
            extra_params: HashMap::new(),
        };

        let splitter = TextSplitterFactory::create_splitter(config)
            .map_err(crate::errors::to_napi_error)?;

        Ok(Self { splitter })
    }

    /// Create a sentence-based text splitter
    #[napi(factory)]
    pub fn sentence(chunk_size: Option<u32>, chunk_overlap: Option<u32>) -> Result<Self> {
        let config = CoreTextSplitterConfig {
            strategy: SplitterStrategy::Sentence {
                chunk_size: chunk_size.unwrap_or(1000) as usize,
                chunk_overlap: chunk_overlap.unwrap_or(0) as usize,
                sentence_endings: None,
            },
            preserve_word_boundaries: true,
            trim_whitespace: true,
            include_metadata: true,
            extra_params: HashMap::new(),
        };

        let splitter = TextSplitterFactory::create_splitter(config)
            .map_err(crate::errors::to_napi_error)?;

        Ok(Self { splitter })
    }

    /// Create a token-based text splitter
    #[napi(factory)]
    pub fn token(chunk_size: u32, chunk_overlap: Option<u32>) -> Result<Self> {
        let config = CoreTextSplitterConfig {
            strategy: SplitterStrategy::Token {
                chunk_size: chunk_size as usize,
                chunk_overlap: chunk_overlap.unwrap_or(0) as usize,
                token_pattern: None,
            },
            preserve_word_boundaries: true,
            trim_whitespace: true,
            include_metadata: true,
            extra_params: HashMap::new(),
        };

        let splitter = TextSplitterFactory::create_splitter(config)
            .map_err(crate::errors::to_napi_error)?;

        Ok(Self { splitter })
    }

    /// Split text into chunks
    #[napi]
    pub fn split(&self, text: String) -> Result<Vec<TextChunk>> {
        let chunks = self.splitter.split_text(&text)
            .map_err(crate::errors::to_napi_error)?;

        Ok(chunks.into_iter().map(TextChunk::from).collect())
    }

    /// Get text splitter configuration
    #[napi]
    pub fn config(&self) -> Result<TextSplitterConfig> {
        let config = self.splitter.config();
        
        Ok(TextSplitterConfig {
            strategy: serde_json::to_value(&config.strategy).map_err(|e| Error::from_reason(e.to_string()))?,
            preserve_word_boundaries: config.preserve_word_boundaries,
            trim_whitespace: config.trim_whitespace,
            include_metadata: config.include_metadata,
            extra_params: serde_json::to_value(&config.extra_params).map_err(|e| Error::from_reason(e.to_string()))?,
        })
    }
}

/// Text chunk with metadata
#[napi(object)]
pub struct TextChunk {
    /// The text content of the chunk
    pub content: String,
    /// Start position in the original text
    pub start_index: u32,
    /// End position in the original text
    pub end_index: u32,
    /// Chunk index in the sequence
    pub chunk_index: u32,
    /// Metadata about the chunk
    #[napi(ts_type = "Record<string, any>")]
    pub metadata: serde_json::Value,
}

impl From<graphbit_core::text_splitter::TextChunk> for TextChunk {
    fn from(chunk: graphbit_core::text_splitter::TextChunk) -> Self {
        Self {
            content: chunk.content,
            start_index: chunk.start_index as u32,
            end_index: chunk.end_index as u32,
            chunk_index: chunk.chunk_index as u32,
            metadata: serde_json::to_value(&chunk.metadata).unwrap_or(serde_json::Value::Null),
        }
    }
}

/// Text splitter configuration
#[napi(object)]
pub struct TextSplitterConfig {
    /// Strategy to use for splitting
    #[napi(ts_type = "any")]
    pub strategy: serde_json::Value,
    /// Whether to preserve word boundaries
    pub preserve_word_boundaries: bool,
    /// Whether to trim whitespace from chunks
    pub trim_whitespace: bool,
    /// Metadata to include with each chunk
    pub include_metadata: bool,
    /// Additional strategy-specific parameters
    #[napi(ts_type = "any")]
    pub extra_params: serde_json::Value,
}

