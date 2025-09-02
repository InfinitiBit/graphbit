//! Text splitter for GraphBit Node.js bindings

use crate::validation::{validate_non_empty_string, validate_positive_number};
use graphbit_core::text_splitter::TextSplitterTrait;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashMap;

#[napi(object)]
pub struct TextSplitterConfig {
    /// Chunk size in characters or tokens
    pub chunk_size: i32,
    /// Overlap between chunks
    pub chunk_overlap: i32,
}

#[napi(object)]
pub struct TextChunk {
    /// The text content of the chunk
    pub content: String,
    /// Metadata for the chunk
    pub metadata: HashMap<String, String>,
    /// Start position in the original text
    pub start_pos: i32,
    /// End position in the original text
    pub end_pos: i32,
    /// Chunk index
    pub index: i32,
}

#[napi]
pub struct CharacterSplitter {
    chunk_size: i32,
    chunk_overlap: i32,
}

#[napi]
impl CharacterSplitter {
    /// Create a new character-based text splitter
    #[napi(constructor)]
    pub fn new(chunk_size: i32, chunk_overlap: i32) -> Result<Self> {
        validate_positive_number(chunk_size, "chunk_size")?;

        if chunk_overlap >= chunk_size {
            return Err(Error::new(
                Status::InvalidArg,
                "chunk_overlap must be less than chunk_size",
            ));
        }

        Ok(Self {
            chunk_size,
            chunk_overlap,
        })
    }

    /// Split text into chunks
    #[napi]
    pub fn split(&self, text: String) -> Result<Vec<TextChunk>> {
        validate_non_empty_string(&text, "text")?;

        let splitter = graphbit_core::text_splitter::CharacterSplitter::new(
            self.chunk_size as usize,
            self.chunk_overlap as usize,
        )
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to create splitter: {}", e),
            )
        })?;

        let core_chunks = splitter.split_text(&text).map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to split text: {}", e),
            )
        })?;

        let chunks = core_chunks
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                // Convert metadata
                let mut metadata = HashMap::new();
                for (key, value) in chunk.metadata {
                    metadata.insert(key, value.to_string());
                }

                TextChunk {
                    content: chunk.content,
                    metadata,
                    start_pos: chunk.start_index as i32,
                    end_pos: chunk.end_index as i32,
                    index: i as i32,
                }
            })
            .collect();

        Ok(chunks)
    }
}

#[napi]
pub struct TokenSplitter {
    chunk_size: i32,
    chunk_overlap: i32,
}

#[napi]
impl TokenSplitter {
    /// Create a new token-based text splitter
    #[napi(constructor)]
    pub fn new(chunk_size: i32, chunk_overlap: i32) -> Result<Self> {
        validate_positive_number(chunk_size, "chunk_size")?;

        if chunk_overlap >= chunk_size {
            return Err(Error::new(
                Status::InvalidArg,
                "chunk_overlap must be less than chunk_size",
            ));
        }

        Ok(Self {
            chunk_size,
            chunk_overlap,
        })
    }

    /// Split text into chunks based on tokens
    #[napi]
    pub fn split(&self, text: String) -> Result<Vec<TextChunk>> {
        validate_non_empty_string(&text, "text")?;

        let splitter = graphbit_core::text_splitter::TokenSplitter::new(
            self.chunk_size as usize,
            self.chunk_overlap as usize,
        )
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to create splitter: {}", e),
            )
        })?;

        let core_chunks = splitter.split_text(&text).map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to split text: {}", e),
            )
        })?;

        let chunks = core_chunks
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                // Convert metadata
                let mut metadata = HashMap::new();
                for (key, value) in chunk.metadata {
                    metadata.insert(key, value.to_string());
                }

                TextChunk {
                    content: chunk.content,
                    metadata,
                    start_pos: chunk.start_index as i32,
                    end_pos: chunk.end_index as i32,
                    index: i as i32,
                }
            })
            .collect();

        Ok(chunks)
    }
}

#[napi]
pub struct SentenceSplitter {
    chunk_size: i32,
    chunk_overlap: i32,
}

#[napi]
impl SentenceSplitter {
    /// Create a new sentence-based text splitter
    #[napi(constructor)]
    pub fn new(chunk_size: i32, chunk_overlap: i32) -> Result<Self> {
        validate_positive_number(chunk_size, "chunk_size")?;

        if chunk_overlap >= chunk_size {
            return Err(Error::new(
                Status::InvalidArg,
                "chunk_overlap must be less than chunk_size",
            ));
        }

        Ok(Self {
            chunk_size,
            chunk_overlap,
        })
    }

    /// Split text into chunks based on sentences
    #[napi]
    pub fn split(&self, text: String) -> Result<Vec<TextChunk>> {
        validate_non_empty_string(&text, "text")?;

        let splitter = graphbit_core::text_splitter::SentenceSplitter::new(
            self.chunk_size as usize,
            self.chunk_overlap as usize,
        )
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to create splitter: {}", e),
            )
        })?;

        let core_chunks = splitter.split_text(&text).map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to split text: {}", e),
            )
        })?;

        let chunks = core_chunks
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                // Convert metadata
                let mut metadata = HashMap::new();
                for (key, value) in chunk.metadata {
                    metadata.insert(key, value.to_string());
                }

                TextChunk {
                    content: chunk.content,
                    metadata,
                    start_pos: chunk.start_index as i32,
                    end_pos: chunk.end_index as i32,
                    index: i as i32,
                }
            })
            .collect();

        Ok(chunks)
    }
}

#[napi]
pub struct RecursiveSplitter {
    chunk_size: i32,
    chunk_overlap: i32,
}

#[napi]
impl RecursiveSplitter {
    /// Create a new recursive text splitter
    #[napi(constructor)]
    pub fn new(chunk_size: i32, chunk_overlap: i32) -> Result<Self> {
        validate_positive_number(chunk_size, "chunk_size")?;

        if chunk_overlap >= chunk_size {
            return Err(Error::new(
                Status::InvalidArg,
                "chunk_overlap must be less than chunk_size",
            ));
        }

        Ok(Self {
            chunk_size,
            chunk_overlap,
        })
    }

    /// Split text into chunks using recursive approach
    #[napi]
    pub fn split(&self, text: String) -> Result<Vec<TextChunk>> {
        validate_non_empty_string(&text, "text")?;

        let splitter = graphbit_core::text_splitter::RecursiveSplitter::new(
            self.chunk_size as usize,
            self.chunk_overlap as usize,
        )
        .map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to create splitter: {}", e),
            )
        })?;

        let core_chunks = splitter.split_text(&text).map_err(|e| {
            Error::new(
                Status::GenericFailure,
                format!("Failed to split text: {}", e),
            )
        })?;

        let chunks = core_chunks
            .into_iter()
            .enumerate()
            .map(|(i, chunk)| {
                // Convert metadata
                let mut metadata = HashMap::new();
                for (key, value) in chunk.metadata {
                    metadata.insert(key, value.to_string());
                }

                TextChunk {
                    content: chunk.content,
                    metadata,
                    start_pos: chunk.start_index as i32,
                    end_pos: chunk.end_index as i32,
                    index: i as i32,
                }
            })
            .collect();

        Ok(chunks)
    }
}
