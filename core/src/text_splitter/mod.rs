//! Text splitting functionality for `GraphBit`
//!
//! This module provides various text splitting strategies for processing
//! large documents into manageable chunks while maintaining context.

mod character;
mod config;
mod recursive;
mod sentence;
mod token;

pub use character::CharacterSplitter;
pub use config::{SplitterStrategy, TextChunk, TextSplitterConfig};
pub use recursive::RecursiveSplitter;
pub use sentence::SentenceSplitter;
pub use token::TokenSplitter;

use crate::errors::{GraphBitError, GraphBitResult};

/// Trait for all text splitter implementations
pub trait TextSplitterTrait: Send + Sync {
    /// Split text into chunks
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>>;

    /// Get the splitter configuration
    fn config(&self) -> &TextSplitterConfig;

    /// Validate configuration parameters
    fn validate_config(&self) -> GraphBitResult<()>;
}

/// Factory for creating text splitters
pub struct TextSplitterFactory;

impl TextSplitterFactory {
    /// Create a text splitter from configuration
    pub fn create_splitter(
        config: TextSplitterConfig,
    ) -> GraphBitResult<Box<dyn TextSplitterTrait>> {
        match &config.strategy {
            SplitterStrategy::Character {
                chunk_size,
                chunk_overlap,
            } => Ok(Box::new(CharacterSplitter::new(
                *chunk_size,
                *chunk_overlap,
            )?)),
            SplitterStrategy::Token {
                chunk_size,
                chunk_overlap,
                token_pattern,
            } => {
                if let Some(pattern) = token_pattern {
                    Ok(Box::new(TokenSplitter::with_pattern(
                        *chunk_size,
                        *chunk_overlap,
                        pattern,
                    )?))
                } else {
                    Ok(Box::new(TokenSplitter::new(*chunk_size, *chunk_overlap)?))
                }
            }
            SplitterStrategy::Sentence {
                chunk_size,
                chunk_overlap,
                sentence_endings,
            } => {
                if let Some(endings) = sentence_endings {
                    let endings_refs: Vec<&str> = endings.iter().map(String::as_str).collect();
                    Ok(Box::new(SentenceSplitter::with_endings(
                        *chunk_size,
                        *chunk_overlap,
                        endings_refs,
                    )?))
                } else {
                    Ok(Box::new(SentenceSplitter::new(
                        *chunk_size,
                        *chunk_overlap,
                    )?))
                }
            }
            SplitterStrategy::Recursive {
                chunk_size,
                chunk_overlap,
                separators,
            } => {
                if let Some(seps) = separators {
                    Ok(Box::new(RecursiveSplitter::with_separators(
                        *chunk_size,
                        *chunk_overlap,
                        seps.clone(),
                    )?))
                } else {
                    Ok(Box::new(RecursiveSplitter::new(
                        *chunk_size,
                        *chunk_overlap,
                    )?))
                }
            }
            _ => Err(GraphBitError::validation(
                "text_splitter",
                format!("Unsupported splitter strategy: {:?}", config.strategy),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_splitter() {
        let splitter = CharacterSplitter::new(10, 2).unwrap();
        let text = "This is a test text for splitting.";
        let chunks = splitter.split_text(text).unwrap();

        assert!(!chunks.is_empty());
        assert!(chunks.iter().all(|c| c.content.len() <= 10));
    }

    #[test]
    fn test_token_splitter() {
        let splitter = TokenSplitter::new(5, 1).unwrap();
        let text = "This is a test text for token splitting.";
        let chunks = splitter.split_text(text).unwrap();

        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_sentence_splitter() {
        let splitter = SentenceSplitter::new(50, 10).unwrap();
        let text =
            "This is the first sentence. This is the second sentence! And this is the third?";
        let chunks = splitter.split_text(text).unwrap();

        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_recursive_splitter() {
        let splitter = RecursiveSplitter::new(20, 5).unwrap();
        let text = "This is paragraph one.\n\nThis is paragraph two.\n\nThis is paragraph three.";
        let chunks = splitter.split_text(text).unwrap();

        assert!(!chunks.is_empty());
    }
}
