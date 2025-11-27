//! Text splitter bindings for JavaScript

use napi::bindgen_prelude::*;
use napi_derive::napi;
use graphbit_core::text_splitter::{
    TextSplitterFactory,
    TextSplitterTrait,
    TextSplitterConfig,
    SplitterStrategy,
    TextChunk as CoreTextChunk,
};
use std::collections::HashMap;

/// Text chunk
#[napi(object)]
pub struct TextChunk {
    pub content: String,
    pub start_index: u32,
    pub end_index: u32,
    pub metadata: Option<String>,
}

impl From<CoreTextChunk> for TextChunk {
    fn from(chunk: CoreTextChunk) -> Self {
        Self {
            content: chunk.content,
            start_index: chunk.start_index as u32,
            end_index: chunk.end_index as u32,
            metadata: serde_json::to_string(&chunk.metadata).ok(),
        }
    }
}

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
        let config = TextSplitterConfig {
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
        let config = TextSplitterConfig {
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
        let config = TextSplitterConfig {
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
        let config = TextSplitterConfig {
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
}

