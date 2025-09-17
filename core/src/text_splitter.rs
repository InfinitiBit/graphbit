//! Text splitting functionality for `GraphBit`
//!
//! This module provides various text splitting strategies for processing
//! large documents into manageable chunks while maintaining context.

use crate::errors::{GraphBitError, GraphBitResult};
use regex::Regex;
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
        /// Maximum size of each chunk in characters
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
    },
    /// Split by token count (word-based)
    Token {
        /// Maximum size of each chunk in tokens
        chunk_size: usize,
        /// Number of tokens to overlap between chunks
        chunk_overlap: usize,
        /// Optional regex pattern for token identification
        token_pattern: Option<String>,
    },
    /// Split by sentence boundaries
    Sentence {
        /// Maximum size of each chunk in characters
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
        /// Optional custom sentence ending patterns
        sentence_endings: Option<Vec<String>>,
    },
    /// Recursive splitting with multiple separators
    Recursive {
        /// Maximum size of each chunk in characters
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
        /// Optional custom separator patterns
        separators: Option<Vec<String>>,
    },
    /// Split by paragraphs
    Paragraph {
        /// Maximum size of each chunk in characters
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
        /// Minimum length required for a paragraph
        min_paragraph_length: Option<usize>,
    },
    /// Split by semantic similarity (requires embeddings)
    Semantic {
        /// Maximum size of each chunk in characters
        max_chunk_size: usize,
        /// Similarity threshold for grouping content
        similarity_threshold: f32,
    },
    /// Split Markdown documents preserving structure
    Markdown {
        /// Maximum size of each chunk in characters
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
        /// Whether to split by header boundaries
        split_by_headers: bool,
    },
    /// Split code files preserving syntax
    Code {
        /// Maximum size of each chunk in characters
        chunk_size: usize,
        /// Number of characters to overlap between chunks
        chunk_overlap: usize,
        /// Programming language for syntax-aware splitting
        language: Option<String>,
    },
    /// Custom regex-based splitting
    Regex {
        /// Regex pattern for splitting
        pattern: String,
        /// Maximum size of each chunk in characters
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

/// Trait for all text splitter implementations
pub trait TextSplitterTrait: Send + Sync {
    /// Split text into chunks
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>>;

    /// Get the splitter configuration
    fn config(&self) -> &TextSplitterConfig;

    /// Validate configuration parameters
    fn validate_config(&self) -> GraphBitResult<()>;
}

/// Character-based text splitter
pub struct CharacterSplitter {
    config: TextSplitterConfig,
    chunk_size: usize,
    chunk_overlap: usize,
}

impl CharacterSplitter {
    /// Create a new character splitter
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> GraphBitResult<Self> {
        if chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }

        if chunk_overlap >= chunk_size {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk overlap must be less than chunk size",
            ));
        }

        let config = TextSplitterConfig {
            strategy: SplitterStrategy::Character {
                chunk_size,
                chunk_overlap,
            },
            ..Default::default()
        };

        Ok(Self {
            config,
            chunk_size,
            chunk_overlap,
        })
    }
}

impl TextSplitterTrait for CharacterSplitter {
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        let mut chunks = Vec::new();
        let mut chunk_index = 0;

        // Build a mapping of character index to byte index
        let char_to_byte: Vec<(usize, usize)> = text
            .char_indices()
            .enumerate()
            .map(|(char_idx, (byte_idx, _))| (char_idx, byte_idx))
            .collect();

        // Add the end position
        let total_chars = text.chars().count();
        let mut char_to_byte_map = char_to_byte;
        char_to_byte_map.push((total_chars, text.len()));

        let mut char_start = 0;

        while char_start < total_chars {
            let mut char_end = (char_start + self.chunk_size).min(total_chars);

            // Get byte positions
            let mut byte_start = char_to_byte_map[char_start].1;
            let mut byte_end = char_to_byte_map[char_end].1;

            // Preserve word boundaries if enabled
            if self.config.preserve_word_boundaries && char_end < total_chars {
                // Look for the last whitespace in the chunk
                let chunk_text = &text[byte_start..byte_end];
                if let Some(last_space_pos) = chunk_text.rfind(char::is_whitespace) {
                    // Adjust to the position after the whitespace
                    byte_end = byte_start
                        + last_space_pos
                        + chunk_text[last_space_pos..]
                            .chars()
                            .next()
                            .unwrap()
                            .len_utf8();

                    // Find the corresponding character position
                    for i in (char_start..char_end).rev() {
                        if char_to_byte_map[i + 1].1 == byte_end {
                            char_end = i + 1;
                            break;
                        }
                    }
                }
            }

            let chunk_content = &text[byte_start..byte_end];
            let content = if self.config.trim_whitespace {
                chunk_content.trim()
            } else {
                chunk_content
            };

            if !content.is_empty() {
                // When trimming, we need to adjust the byte positions to match the trimmed content
                if self.config.trim_whitespace {
                    let trim_start = chunk_content.len() - chunk_content.trim_start().len();
                    let trim_end = chunk_content.trim_end().len() - content.len();
                    byte_start += trim_start;
                    byte_end -= trim_end;
                }

                chunks.push(TextChunk::new(
                    content.to_string(),
                    byte_start,
                    byte_end,
                    chunk_index,
                ));
                chunk_index += 1;
            }

            // Move to next chunk with overlap
            let next_char_start = if self.chunk_overlap > 0 && char_end < total_chars {
                char_end.saturating_sub(self.chunk_overlap)
            } else {
                char_end
            };

            char_start = next_char_start.max(char_start + 1);
        }

        Ok(chunks)
    }

    fn config(&self) -> &TextSplitterConfig {
        &self.config
    }

    fn validate_config(&self) -> GraphBitResult<()> {
        if self.chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }
        Ok(())
    }
}

/// Token-based text splitter
pub struct TokenSplitter {
    config: TextSplitterConfig,
    chunk_size: usize,
    chunk_overlap: usize,
    token_pattern: Regex,
}

impl TokenSplitter {
    /// Create a new token splitter
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> GraphBitResult<Self> {
        Self::with_pattern(chunk_size, chunk_overlap, r"\b\w+\b|\s+|[^\w\s]+")
    }

    /// Create a token splitter with custom token pattern
    pub fn with_pattern(
        chunk_size: usize,
        chunk_overlap: usize,
        pattern: &str,
    ) -> GraphBitResult<Self> {
        if chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }

        if chunk_overlap >= chunk_size {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk overlap must be less than chunk size",
            ));
        }

        let token_pattern = Regex::new(pattern).map_err(|e| {
            GraphBitError::validation("text_splitter", format!("Invalid regex pattern: {}", e))
        })?;

        let config = TextSplitterConfig {
            strategy: SplitterStrategy::Token {
                chunk_size,
                chunk_overlap,
                token_pattern: Some(pattern.to_string()),
            },
            ..Default::default()
        };

        Ok(Self {
            config,
            chunk_size,
            chunk_overlap,
            token_pattern,
        })
    }
}

impl TextSplitterTrait for TokenSplitter {
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        // Tokenize the text
        let tokens: Vec<&str> = self
            .token_pattern
            .find_iter(text)
            .map(|m| m.as_str())
            .collect();

        if tokens.is_empty() {
            return Ok(Vec::new());
        }

        let mut chunks = Vec::new();
        let mut chunk_index = 0;
        let mut i = 0;

        while i < tokens.len() {
            let chunk_end = (i + self.chunk_size).min(tokens.len());
            let chunk_tokens = &tokens[i..chunk_end];

            // Calculate start and end positions
            let start_pos = tokens[i].as_ptr() as usize - text.as_ptr() as usize;
            let last_token = tokens[chunk_end - 1];
            let end_pos = last_token.as_ptr() as usize - text.as_ptr() as usize + last_token.len();

            let content = chunk_tokens.join("");
            let content = if self.config.trim_whitespace {
                content.trim().to_string()
            } else {
                content
            };

            if !content.is_empty() {
                let mut chunk = TextChunk::new(content, start_pos, end_pos, chunk_index);
                chunk.metadata.insert(
                    "token_count".to_string(),
                    serde_json::Value::Number(chunk_tokens.len().into()),
                );
                chunks.push(chunk);
                chunk_index += 1;
            }

            // Move to next chunk with overlap
            i = if self.chunk_overlap > 0 && chunk_end < tokens.len() {
                chunk_end.saturating_sub(self.chunk_overlap)
            } else {
                chunk_end
            };
        }

        Ok(chunks)
    }

    fn config(&self) -> &TextSplitterConfig {
        &self.config
    }

    fn validate_config(&self) -> GraphBitResult<()> {
        if self.chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }
        Ok(())
    }
}

/// Sentence-based text splitter
pub struct SentenceSplitter {
    config: TextSplitterConfig,
    chunk_size: usize,
    chunk_overlap: usize,
    sentence_pattern: Regex,
}

impl SentenceSplitter {
    /// Create a new sentence splitter
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> GraphBitResult<Self> {
        let default_endings = vec![
            r"[.!?]+[\s\n]+",
            r"[。！？]+[\s\n]*", // Chinese/Japanese
            r"[\n\r]+",          // Newlines as sentence boundaries
        ];

        Self::with_endings(chunk_size, chunk_overlap, default_endings)
    }

    /// Create a sentence splitter with custom sentence endings
    pub fn with_endings(
        chunk_size: usize,
        chunk_overlap: usize,
        endings: Vec<&str>,
    ) -> GraphBitResult<Self> {
        if chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }

        let pattern = format!("({})", endings.join("|"));
        let sentence_pattern = Regex::new(&pattern).map_err(|e| {
            GraphBitError::validation("text_splitter", format!("Invalid regex pattern: {}", e))
        })?;

        let config = TextSplitterConfig {
            strategy: SplitterStrategy::Sentence {
                chunk_size,
                chunk_overlap,
                sentence_endings: Some(endings.iter().map(|s| s.to_string()).collect()),
            },
            ..Default::default()
        };

        Ok(Self {
            config,
            chunk_size,
            chunk_overlap,
            sentence_pattern,
        })
    }
}

impl TextSplitterTrait for SentenceSplitter {
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        // Split text into sentences
        let mut sentences = Vec::new();
        let mut last_end = 0;

        for mat in self.sentence_pattern.find_iter(text) {
            let sentence_end = mat.end();
            let sentence = &text[last_end..sentence_end];
            if !sentence.trim().is_empty() {
                sentences.push((sentence, last_end, sentence_end));
            }
            last_end = sentence_end;
        }

        // Don't forget the last sentence if it doesn't end with a sentence ending
        if last_end < text.len() {
            let sentence = &text[last_end..];
            if !sentence.trim().is_empty() {
                sentences.push((sentence, last_end, text.len()));
            }
        }

        // Group sentences into chunks
        let mut chunks = Vec::new();
        let mut chunk_index = 0;
        let mut i = 0;

        while i < sentences.len() {
            let mut current_size = 0;
            let mut j = i;

            // Add sentences until we reach chunk size
            while j < sentences.len() && current_size < self.chunk_size {
                current_size += sentences[j].0.len();
                j += 1;
            }

            // Ensure we have at least one sentence per chunk
            if j == i {
                j = i + 1;
            }

            // Create chunk from sentences[i..j]
            let start_pos = sentences[i].1;
            let end_pos = sentences[j - 1].2;
            let content: String = sentences[i..j].iter().map(|(s, _, _)| *s).collect();

            let content = if self.config.trim_whitespace {
                content.trim().to_string()
            } else {
                content
            };

            if !content.is_empty() {
                let mut chunk = TextChunk::new(content, start_pos, end_pos, chunk_index);
                chunk.metadata.insert(
                    "sentence_count".to_string(),
                    serde_json::Value::Number((j - i).into()),
                );
                chunks.push(chunk);
                chunk_index += 1;
            }

            // Move to next chunk with overlap
            let next_i = if self.chunk_overlap > 0 && j < sentences.len() {
                j.saturating_sub(self.chunk_overlap)
            } else {
                j
            };

            i = next_i.max(i + 1);
        }

        Ok(chunks)
    }

    fn config(&self) -> &TextSplitterConfig {
        &self.config
    }

    fn validate_config(&self) -> GraphBitResult<()> {
        if self.chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }
        Ok(())
    }
}

/// Recursive text splitter that tries multiple separators
pub struct RecursiveSplitter {
    config: TextSplitterConfig,
    chunk_size: usize,
    chunk_overlap: usize,
    separators: Vec<String>,
}

impl RecursiveSplitter {
    /// Create a new recursive splitter with default separators
    pub fn new(chunk_size: usize, chunk_overlap: usize) -> GraphBitResult<Self> {
        let default_separators = vec![
            "\n\n".to_string(), // Double newline (paragraphs)
            "\n".to_string(),   // Single newline
            ". ".to_string(),   // Sentence end
            "! ".to_string(),   // Exclamation
            "? ".to_string(),   // Question
            "; ".to_string(),   // Semicolon
            ": ".to_string(),   // Colon
            " - ".to_string(),  // Dash
            " ".to_string(),    // Space
            "".to_string(),     // Character level
        ];

        Self::with_separators(chunk_size, chunk_overlap, default_separators)
    }

    /// Create a recursive splitter with custom separators
    pub fn with_separators(
        chunk_size: usize,
        chunk_overlap: usize,
        separators: Vec<String>,
    ) -> GraphBitResult<Self> {
        if chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }

        if separators.is_empty() {
            return Err(GraphBitError::validation(
                "text_splitter",
                "At least one separator must be provided",
            ));
        }

        let config = TextSplitterConfig {
            strategy: SplitterStrategy::Recursive {
                chunk_size,
                chunk_overlap,
                separators: Some(separators.clone()),
            },
            ..Default::default()
        };

        Ok(Self {
            config,
            chunk_size,
            chunk_overlap,
            separators,
        })
    }

    /// Recursively split text using the best separator
    fn recursive_split(&self, text: &str, separators: &[String]) -> Vec<String> {
        if separators.is_empty() || text.len() <= self.chunk_size {
            return vec![text.to_string()];
        }

        let separator = &separators[0];
        let remaining_separators = &separators[1..];

        if separator.is_empty() {
            // Character-level split
            return self.split_by_characters(text);
        }

        let parts: Vec<&str> = text.split(separator).collect();

        let mut result = Vec::new();
        let mut current_chunk = String::new();

        for part in parts {
            let potential_chunk = if current_chunk.is_empty() {
                part.to_string()
            } else {
                format!("{}{}{}", current_chunk, separator, part)
            };

            if potential_chunk.len() <= self.chunk_size {
                current_chunk = potential_chunk;
            } else {
                // Current chunk is getting too big
                if !current_chunk.is_empty() {
                    result.push(current_chunk);
                    current_chunk = String::new();
                }

                if part.len() > self.chunk_size {
                    // Recursively split the part
                    let sub_parts = self.recursive_split(part, remaining_separators);
                    result.extend(sub_parts);
                } else {
                    current_chunk = part.to_string();
                }
            }
        }

        if !current_chunk.is_empty() {
            result.push(current_chunk);
        }

        result
    }

    /// Split by characters when no other separator works
    fn split_by_characters(&self, text: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut start = 0;

        while start < text.len() {
            let end = (start + self.chunk_size).min(text.len());
            result.push(text[start..end].to_string());
            start = end;
        }

        result
    }
}

impl TextSplitterTrait for RecursiveSplitter {
    fn split_text(&self, text: &str) -> GraphBitResult<Vec<TextChunk>> {
        if text.is_empty() {
            return Ok(Vec::new());
        }

        let parts = self.recursive_split(text, &self.separators);
        let mut chunks = Vec::new();
        let mut position = 0;

        for (chunk_index, part) in parts.iter().enumerate() {
            let content = if self.config.trim_whitespace {
                part.trim().to_string()
            } else {
                part.clone()
            };

            if !content.is_empty() {
                // Find the actual position in the original text
                if let Some(start_pos) = text[position..].find(&content) {
                    let actual_start = position + start_pos;
                    let actual_end = actual_start + content.len();

                    chunks.push(TextChunk::new(
                        content,
                        actual_start,
                        actual_end,
                        chunk_index,
                    ));

                    position = actual_end;
                }
            }
        }

        // Apply overlap if needed
        if self.chunk_overlap > 0 && chunks.len() > 1 {
            let mut overlapped_chunks = Vec::new();

            for i in 0..chunks.len() {
                let mut chunk_content = chunks[i].content.clone();

                // Add overlap from previous chunk
                if i > 0 {
                    let prev_chunk = &chunks[i - 1];
                    let overlap_start = prev_chunk.content.len().saturating_sub(self.chunk_overlap);
                    let overlap_text = &prev_chunk.content[overlap_start..];
                    chunk_content = format!("{}{}", overlap_text, chunk_content);
                }

                // Add overlap from next chunk
                if i < chunks.len() - 1 {
                    let next_chunk = &chunks[i + 1];
                    let overlap_end = self.chunk_overlap.min(next_chunk.content.len());
                    let overlap_text = &next_chunk.content[..overlap_end];
                    chunk_content = format!("{}{}", chunk_content, overlap_text);
                }

                overlapped_chunks.push(TextChunk::new(
                    chunk_content,
                    chunks[i].start_index,
                    chunks[i].end_index,
                    i,
                ));
            }

            return Ok(overlapped_chunks);
        }

        Ok(chunks)
    }

    fn config(&self) -> &TextSplitterConfig {
        &self.config
    }

    fn validate_config(&self) -> GraphBitResult<()> {
        if self.chunk_size == 0 {
            return Err(GraphBitError::validation(
                "text_splitter",
                "Chunk size must be greater than 0",
            ));
        }
        if self.separators.is_empty() {
            return Err(GraphBitError::validation(
                "text_splitter",
                "At least one separator must be provided",
            ));
        }
        Ok(())
    }
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
                    let endings_refs: Vec<&str> = endings.iter().map(|s| s.as_str()).collect();
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
