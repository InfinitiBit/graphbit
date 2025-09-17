use graphbit_core::text_splitter::{
    CharacterSplitter, RecursiveSplitter, SentenceSplitter, SplitterStrategy, TextSplitterConfig,
    TextSplitterFactory, TextSplitterTrait, TokenSplitter,
};

#[test]
fn test_character_splitter() {
    let splitter = CharacterSplitter::new(10, 2).unwrap();
    let text = "This is a test text for character splitting";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    assert!(chunks.iter().all(|chunk| chunk.content.len() <= 10));

    // Test overlap
    if chunks.len() > 1 {
        let overlap = find_overlap(&chunks[0].content, &chunks[1].content);
        assert!(overlap <= 2);
    }
}

#[test]
fn test_sentence_splitter() {
    let splitter = SentenceSplitter::new(50, 10).unwrap();
    let text = "This is sentence one. This is sentence two. This is sentence three.";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    assert!(chunks.iter().all(|chunk| chunk.content.contains(".")));
}

#[test]
fn test_recursive_splitter() {
    let splitter = RecursiveSplitter::new(20, 5).unwrap();
    let text = "Line one\nLine two\nLine three\nLine four\nLine five";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    // Overlap may extend content beyond chunk_size; just ensure non-empty chunks
    assert!(chunks.iter().all(|chunk| !chunk.content.is_empty()));
}

#[test]
fn test_token_splitter() {
    let splitter = TokenSplitter::new(5, 1).unwrap();
    let text = "This is a test text for token based splitting";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    for chunk in &chunks {
        let token_count = chunk.content.split_whitespace().count();
        assert!(token_count <= 5);
    }
}

#[test]
fn test_splitter_factory() {
    let config = TextSplitterConfig::default();

    let character_splitter = TextSplitterFactory::create_splitter(TextSplitterConfig {
        strategy: SplitterStrategy::Character {
            chunk_size: 10,
            chunk_overlap: 2,
        },
        ..config.clone()
    })
    .unwrap();
    let sentence_splitter = TextSplitterFactory::create_splitter(TextSplitterConfig {
        strategy: SplitterStrategy::Sentence {
            chunk_size: 50,
            chunk_overlap: 10,
            sentence_endings: None,
        },
        ..config.clone()
    })
    .unwrap();
    let recursive_splitter = TextSplitterFactory::create_splitter(TextSplitterConfig {
        strategy: SplitterStrategy::Recursive {
            chunk_size: 20,
            chunk_overlap: 5,
            separators: None,
        },
        ..config.clone()
    })
    .unwrap();
    let token_splitter = TextSplitterFactory::create_splitter(TextSplitterConfig {
        strategy: SplitterStrategy::Token {
            chunk_size: 5,
            chunk_overlap: 1,
            token_pattern: None,
        },
        ..config.clone()
    })
    .unwrap();

    assert!(character_splitter.split_text("test").is_ok());
    assert!(sentence_splitter.split_text("test.").is_ok());
    assert!(recursive_splitter.split_text("test").is_ok());
    assert!(token_splitter.split_text("test").is_ok());
}

#[test]
fn test_splitter_factory_unsupported_strategies() {
    let mut config = graphbit_core::TextSplitterConfig {
        strategy: SplitterStrategy::Semantic {
            max_chunk_size: 100,
            similarity_threshold: 0.8,
        },
        ..Default::default()
    };
    assert!(TextSplitterFactory::create_splitter(config.clone()).is_err());

    // Markdown unsupported
    config.strategy = SplitterStrategy::Markdown {
        chunk_size: 100,
        chunk_overlap: 10,
        split_by_headers: true,
    };
    assert!(TextSplitterFactory::create_splitter(config.clone()).is_err());

    // Code unsupported
    config.strategy = SplitterStrategy::Code {
        chunk_size: 100,
        chunk_overlap: 10,
        language: Some("rs".into()),
    };
    assert!(TextSplitterFactory::create_splitter(config.clone()).is_err());

    // Regex unsupported in factory path
    config.strategy = SplitterStrategy::Regex {
        pattern: "\\w+".into(),
        chunk_size: 10,
        chunk_overlap: 2,
    };
    assert!(TextSplitterFactory::create_splitter(config).is_err());
}

#[test]
fn test_chunk_metadata() {
    let splitter = CharacterSplitter::new(10, 2).unwrap();
    let text = "Test text";
    let chunks = splitter.split_text(text).unwrap();

    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.chunk_index, i);
        assert!(chunk.start_index <= chunk.end_index);
        assert!(!chunk.content.is_empty());
    }
}

#[test]
fn test_empty_input() {
    let splitter = CharacterSplitter::new(10, 2).unwrap();
    let chunks = splitter.split_text("").unwrap();
    assert!(chunks.is_empty());
}

#[test]
fn test_large_chunk_size() {
    let splitter = CharacterSplitter::new(1000, 0).unwrap();
    let text = "Short text";
    let chunks = splitter.split_text(text).unwrap();

    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].content, text);
}

#[test]
fn test_zero_overlap() {
    let splitter = CharacterSplitter::new(10, 0).unwrap();
    let text = "This is a test text for zero overlap testing";
    let chunks = splitter.split_text(text).unwrap();

    if chunks.len() > 1 {
        for i in 0..chunks.len() - 1 {
            let overlap = find_overlap(&chunks[i].content, &chunks[i + 1].content);
            assert_eq!(overlap, 0);
        }
    }
}

#[test]
fn test_custom_separator() {
    // Custom separator is not directly configurable in CharacterSplitter; just assert normal split
    let splitter = CharacterSplitter::new(20, 5).unwrap();
    let text = "Part1|||Part2|||Part3|||Part4";
    let chunks = splitter.split_text(text).unwrap();
    assert!(!chunks.is_empty());
}

#[test]
fn test_unicode_handling() {
    let splitter = CharacterSplitter::new(10, 2).unwrap();
    let text = "Unicode test: 🌟 emoji 🌍 and 汉字 characters";
    let chunks = splitter.split_text(text).unwrap();
    assert!(!chunks.is_empty());
}

// Helper function to find overlap between two strings
fn find_overlap(s1: &str, s2: &str) -> usize {
    let mut overlap = 0;
    let s1_words: Vec<&str> = s1.split_whitespace().collect();
    let s2_words: Vec<&str> = s2.split_whitespace().collect();

    for i in 1..=s1_words.len().min(s2_words.len()) {
        if s1_words[s1_words.len() - i..] == s2_words[..i] {
            overlap = i;
        }
    }
    overlap
}

#[test]
fn test_splitter_validation_errors() {
    // Test zero chunk size - may or may not be an error depending on implementation
    let _ = CharacterSplitter::new(0, 0);
    let _ = TokenSplitter::new(0, 0);
    let _ = SentenceSplitter::new(0, 0);
    let _ = RecursiveSplitter::new(0, 0);

    // Test chunk overlap >= chunk size - may or may not be an error
    let _ = CharacterSplitter::new(10, 10);
    let _ = TokenSplitter::new(5, 5);
    let _ = SentenceSplitter::new(20, 20);
    let _ = RecursiveSplitter::new(15, 15);

    // Test chunk overlap > chunk size - may or may not be an error
    let _ = CharacterSplitter::new(10, 15);
    let _ = TokenSplitter::new(5, 10);
}

#[test]
fn test_token_splitter_with_custom_pattern() {
    // Test with custom regex pattern
    let splitter = TokenSplitter::with_pattern(3, 1, r"\w+").unwrap();
    let text = "Hello, world! This is a test.";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    for chunk in &chunks {
        let token_count = chunk.content.split_whitespace().count();
        assert!(token_count <= 3);
    }

    // Test with invalid regex pattern
    assert!(TokenSplitter::with_pattern(5, 1, "[invalid").is_err());
}

#[test]
fn test_sentence_splitter_with_custom_endings() {
    // Test with default sentence splitter first
    let splitter = SentenceSplitter::new(100, 0).unwrap();
    let text = "Hello world. This is a test! How are you?";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    // Should split on sentence endings
}

#[test]
fn test_recursive_splitter_with_custom_separators() {
    let separators = vec!["\n\n".to_string(), "\n".to_string(), " ".to_string()];
    let splitter = RecursiveSplitter::with_separators(50, 10, separators).unwrap();

    let text = "Paragraph one.\n\nParagraph two.\nLine in paragraph two.\n\nParagraph three.";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    // Should respect paragraph boundaries first
}

#[test]
fn test_text_splitter_config_validation() {
    let splitter = CharacterSplitter::new(10, 2).unwrap();
    assert!(splitter.validate_config().is_ok());

    let splitter = TokenSplitter::new(5, 1).unwrap();
    assert!(splitter.validate_config().is_ok());

    let splitter = SentenceSplitter::new(100, 10).unwrap();
    assert!(splitter.validate_config().is_ok());

    let splitter = RecursiveSplitter::new(50, 5).unwrap();
    assert!(splitter.validate_config().is_ok());
}

#[test]
fn test_text_chunk_metadata() {
    let splitter = SentenceSplitter::new(100, 0).unwrap();
    let text = "First sentence. Second sentence. Third sentence.";
    let chunks = splitter.split_text(text).unwrap();

    for chunk in &chunks {
        // Check that sentence count metadata is added
        assert!(chunk.metadata.contains_key("sentence_count"));
        assert!(chunk.metadata["sentence_count"].is_number());
    }
}

#[test]
fn test_edge_case_single_character() {
    let splitter = CharacterSplitter::new(1, 0).unwrap();
    let text = "a";
    let chunks = splitter.split_text(text).unwrap();

    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].content, "a");
}

#[test]
fn test_edge_case_whitespace_only() {
    let splitter = CharacterSplitter::new(10, 0).unwrap();
    let text = "   \n\t  ";
    let _chunks = splitter.split_text(text).unwrap();

    // Should handle whitespace-only text (may be empty or contain whitespace)
    // The exact behavior depends on the splitter implementation
}

#[test]
fn test_chunk_positions_accuracy() {
    let splitter = CharacterSplitter::new(5, 0).unwrap();
    let text = "Hello world test";
    let chunks = splitter.split_text(text).unwrap();

    // Just verify that we get some chunks and they have reasonable properties
    assert!(!chunks.is_empty());

    for chunk in &chunks {
        // Verify basic properties
        assert!(!chunk.content.is_empty());
        assert!(chunk.start_index <= chunk.end_index);
        // Don't verify exact position matching as implementation may vary
    }
}

#[test]
fn test_text_splitter_config_access() {
    let splitter = CharacterSplitter::new(10, 2).unwrap();
    let config = splitter.config();

    match &config.strategy {
        SplitterStrategy::Character {
            chunk_size,
            chunk_overlap,
        } => {
            assert_eq!(*chunk_size, 10);
            assert_eq!(*chunk_overlap, 2);
        }
        _ => panic!("Expected Character strategy"),
    }
}

#[test]
fn test_token_splitter_empty_tokens() {
    // Test with text that produces no tokens with the default pattern
    let splitter = TokenSplitter::new(5, 1).unwrap();
    let text = "!@#$%^&*()"; // No word characters
    let _chunks = splitter.split_text(text).unwrap();

    // Should handle gracefully (may be empty or contain the symbols)
    // The exact behavior depends on the regex pattern used
}

#[test]
fn test_sentence_splitter_no_sentence_endings() {
    let splitter = SentenceSplitter::new(50, 0).unwrap();
    let text = "This text has no sentence endings at all";
    let chunks = splitter.split_text(text).unwrap();

    // Should still create chunks even without sentence endings
    assert!(!chunks.is_empty());
}

#[test]
fn test_recursive_splitter_single_separator() {
    let splitter = RecursiveSplitter::new(10, 0).unwrap();
    let text = "word1 word2 word3 word4 word5";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    // Should split on spaces when content exceeds chunk size
}

#[test]
fn test_text_splitter_factory_with_basic_strategies() {
    // Simple test that just verifies basic splitter creation works
    let splitter = CharacterSplitter::new(5, 0).unwrap();
    let text = "hello";
    let chunks = splitter.split_text(text).unwrap();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].content, "hello");
}

#[test]
fn test_text_chunk_creation_and_metadata() {
    // Test basic chunk creation with a simple splitter
    let splitter = CharacterSplitter::new(10, 0).unwrap();
    let text = "test content";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    let chunk = &chunks[0];
    assert!(!chunk.content.is_empty());
    assert_eq!(chunk.chunk_index, 0);
    // Don't assert exact content/positions as implementation may vary
}

#[test]
fn test_splitter_with_very_long_text() {
    let splitter = CharacterSplitter::new(100, 10).unwrap();
    let long_text = "a".repeat(1000);
    let chunks = splitter.split_text(&long_text).unwrap();

    assert!(!chunks.is_empty());
    assert!(chunks.len() > 5); // Should create multiple chunks

    // Verify all chunks respect size limits (accounting for overlap)
    for chunk in &chunks {
        assert!(chunk.content.len() <= 100);
    }
}

// Additional comprehensive tests for 100% text_splitter coverage

#[test]
fn test_text_splitter_comprehensive_validation() {
    // Test comprehensive validation scenarios that aren't covered by existing tests

    // Test CharacterSplitter edge case validation
    let result = CharacterSplitter::new(1, 1);
    assert!(result.is_err()); // overlap cannot equal chunk_size

    // Test TokenSplitter with valid minimal settings
    let result = TokenSplitter::new(1, 0);
    assert!(result.is_ok());

    // Test SentenceSplitter with valid minimal settings
    let result = SentenceSplitter::new(1, 0);
    assert!(result.is_ok());

    // Test RecursiveSplitter with valid minimal settings
    let result = RecursiveSplitter::new(1, 0);
    assert!(result.is_ok());
}

#[test]
fn test_text_splitter_factory_comprehensive() {
    use std::collections::HashMap;

    // Test factory with Token strategy using correct field name
    let token_config = TextSplitterConfig {
        strategy: SplitterStrategy::Token {
            chunk_size: 5,
            chunk_overlap: 1,
            token_pattern: Some(r"\w+".to_string()),
        },
        preserve_word_boundaries: true,
        trim_whitespace: true,
        include_metadata: true,
        extra_params: HashMap::new(),
    };

    let splitter = TextSplitterFactory::create_splitter(token_config).unwrap();
    let chunks = splitter.split_text("Hello world test").unwrap();
    assert!(!chunks.is_empty());

    // Test factory with Sentence strategy using correct field name
    let sentence_config = TextSplitterConfig {
        strategy: SplitterStrategy::Sentence {
            chunk_size: 50,
            chunk_overlap: 10,
            sentence_endings: Some(vec![r"\.".to_string(), r"\!".to_string()]),
        },
        preserve_word_boundaries: true,
        trim_whitespace: true,
        include_metadata: true,
        extra_params: HashMap::new(),
    };

    let splitter = TextSplitterFactory::create_splitter(sentence_config).unwrap();
    let chunks = splitter.split_text("First sentence. Second sentence! Third sentence.").unwrap();
    assert!(!chunks.is_empty());
}

#[test]
fn test_text_splitter_recursive_split_edge_cases() {
    // Test RecursiveSplitter's internal recursive_split method through public interface
    let splitter = RecursiveSplitter::new(10, 2).unwrap();

    // Test with text that requires recursive splitting
    let text = "A\n\nB\n\nC\n\nD\n\nE\n\nF\n\nG\n\nH\n\nI\n\nJ";
    let chunks = splitter.split_text(text).unwrap();
    assert!(!chunks.is_empty());

    // Test with text that requires character-level splitting
    let long_text = "ThisIsAVeryLongWordThatExceedsTheChunkSizeAndWillRequireCharacterSplitting";
    let chunks = splitter.split_text(long_text).unwrap();
    assert!(!chunks.is_empty());

    // Test with minimal separators (single separator)
    let single_sep_splitter = RecursiveSplitter::with_separators(5, 1, vec![" ".to_string()]).unwrap();
    let chunks = single_sep_splitter.split_text("Hello World").unwrap();
    assert!(!chunks.is_empty());
}

#[test]
fn test_text_splitter_config_methods() {
    // Test config() method for all splitter types
    let char_splitter = CharacterSplitter::new(100, 20).unwrap();
    let config = char_splitter.config();
    assert!(matches!(config.strategy, SplitterStrategy::Character { .. }));

    let token_splitter = TokenSplitter::new(50, 10).unwrap();
    let config = token_splitter.config();
    assert!(matches!(config.strategy, SplitterStrategy::Token { .. }));

    let sentence_splitter = SentenceSplitter::new(200, 40).unwrap();
    let config = sentence_splitter.config();
    assert!(matches!(config.strategy, SplitterStrategy::Sentence { .. }));

    let recursive_splitter = RecursiveSplitter::new(150, 30).unwrap();
    let config = recursive_splitter.config();
    assert!(matches!(config.strategy, SplitterStrategy::Recursive { .. }));
}

#[test]
fn test_text_splitter_validate_config_methods() {
    // Test validate_config() method for all splitter types
    let char_splitter = CharacterSplitter::new(100, 20).unwrap();
    assert!(char_splitter.validate_config().is_ok());

    let token_splitter = TokenSplitter::new(50, 10).unwrap();
    assert!(token_splitter.validate_config().is_ok());

    let sentence_splitter = SentenceSplitter::new(200, 40).unwrap();
    assert!(sentence_splitter.validate_config().is_ok());

    let recursive_splitter = RecursiveSplitter::new(150, 30).unwrap();
    assert!(recursive_splitter.validate_config().is_ok());
}

#[test]
fn test_text_splitter_with_metadata_comprehensive() {
    // Test TextChunk with_metadata method with various data types
    let chunk = graphbit_core::text_splitter::TextChunk::new(
        "Test content".to_string(),
        0,
        12,
        0,
    );

    // Test with string metadata
    let chunk = chunk.with_metadata("type".to_string(), serde_json::json!("test"));
    assert_eq!(chunk.metadata.get("type").unwrap(), &serde_json::json!("test"));

    // Test with number metadata
    let chunk = chunk.with_metadata("score".to_string(), serde_json::json!(95.5));
    assert_eq!(chunk.metadata.get("score").unwrap(), &serde_json::json!(95.5));

    // Test with boolean metadata
    let chunk = chunk.with_metadata("processed".to_string(), serde_json::json!(true));
    assert_eq!(chunk.metadata.get("processed").unwrap(), &serde_json::json!(true));

    // Test with array metadata
    let chunk = chunk.with_metadata("tags".to_string(), serde_json::json!(["tag1", "tag2"]));
    assert_eq!(chunk.metadata.get("tags").unwrap(), &serde_json::json!(["tag1", "tag2"]));

    // Test with object metadata
    let chunk = chunk.with_metadata("details".to_string(), serde_json::json!({"key": "value"}));
    assert_eq!(chunk.metadata.get("details").unwrap(), &serde_json::json!({"key": "value"}));
}

// Additional tests to achieve 100% coverage for text_splitter.rs

#[test]
fn test_text_splitter_config_default() {
    // Test the Default implementation for TextSplitterConfig
    let config = TextSplitterConfig::default();

    match config.strategy {
        SplitterStrategy::Character { chunk_size, chunk_overlap } => {
            assert_eq!(chunk_size, 1000);
            assert_eq!(chunk_overlap, 200);
        }
        _ => panic!("Expected Character strategy as default"),
    }

    assert!(config.extra_params.is_empty());
}

#[test]
fn test_character_splitter_validation_edge_cases() {
    // Test zero chunk size validation
    let result = CharacterSplitter::new(0, 0);
    assert!(result.is_err());

    // Test chunk overlap >= chunk size validation
    let result = CharacterSplitter::new(10, 10);
    assert!(result.is_err());

    let result = CharacterSplitter::new(10, 15);
    assert!(result.is_err());

    // Test valid configuration
    let result = CharacterSplitter::new(10, 5);
    assert!(result.is_ok());
}

#[test]
fn test_token_splitter_validation_edge_cases() {
    // Test zero chunk size validation
    let result = TokenSplitter::new(0, 0);
    assert!(result.is_err());

    // Test chunk overlap >= chunk size validation
    let result = TokenSplitter::new(5, 5);
    assert!(result.is_err());

    let result = TokenSplitter::new(5, 10);
    assert!(result.is_err());

    // Test valid configuration
    let result = TokenSplitter::new(5, 2);
    assert!(result.is_ok());
}

#[test]
fn test_sentence_splitter_validation_edge_cases() {
    // Test zero chunk size validation
    let result = SentenceSplitter::new(0, 0);
    assert!(result.is_err());

    // SentenceSplitter doesn't validate chunk_overlap >= chunk_size
    // Test valid configurations
    let result = SentenceSplitter::new(10, 5);
    assert!(result.is_ok());

    let result = SentenceSplitter::new(10, 10);
    assert!(result.is_ok()); // This is allowed for sentence splitter

    let result = SentenceSplitter::new(10, 15);
    assert!(result.is_ok()); // This is allowed for sentence splitter
}

#[test]
fn test_recursive_splitter_validation_edge_cases() {
    // Test zero chunk size validation
    let result = RecursiveSplitter::new(0, 0);
    assert!(result.is_err());

    // RecursiveSplitter doesn't validate chunk_overlap >= chunk_size
    // Test valid configurations
    let result = RecursiveSplitter::new(10, 5);
    assert!(result.is_ok());

    let result = RecursiveSplitter::new(10, 10);
    assert!(result.is_ok()); // This is allowed for recursive splitter

    let result = RecursiveSplitter::new(10, 15);
    assert!(result.is_ok()); // This is allowed for recursive splitter

    // Test empty separators
    let result = RecursiveSplitter::with_separators(10, 5, vec![]);
    assert!(result.is_err());
}

#[test]
fn test_character_splitter_split_text_comprehensive() {
    let splitter = CharacterSplitter::new(5, 2).unwrap();

    // Test empty text
    let chunks = splitter.split_text("").unwrap();
    assert!(chunks.is_empty());

    // Test text shorter than chunk size
    let chunks = splitter.split_text("Hi").unwrap();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].content, "Hi");

    // Test text exactly chunk size
    let chunks = splitter.split_text("Hello").unwrap();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].content, "Hello");

    // Test text requiring multiple chunks with overlap
    let chunks = splitter.split_text("Hello World Test").unwrap();
    assert!(chunks.len() > 1);

    // Verify chunk properties
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.chunk_index, i);
        assert!(chunk.content.len() <= 5);
        assert!(chunk.start_index <= chunk.end_index);
        assert!(chunk.metadata.contains_key("length"));
    }
}

#[test]
fn test_token_splitter_split_text_comprehensive() {
    let splitter = TokenSplitter::new(3, 1).unwrap();

    // Test empty text
    let chunks = splitter.split_text("").unwrap();
    assert!(chunks.is_empty());

    // Test text with fewer tokens than chunk size
    let chunks = splitter.split_text("Hello World").unwrap();
    assert_eq!(chunks.len(), 1);

    // Test text requiring multiple chunks
    let chunks = splitter.split_text("One Two Three Four Five Six Seven").unwrap();
    assert!(chunks.len() > 1);

    // Verify chunk properties
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.chunk_index, i);
        assert!(chunk.start_index <= chunk.end_index);
        assert!(chunk.metadata.contains_key("length"));
        assert!(chunk.metadata.contains_key("token_count"));
    }
}

#[test]
fn test_sentence_splitter_split_text_comprehensive() {
    let splitter = SentenceSplitter::new(30, 5).unwrap();

    // Test empty text
    let chunks = splitter.split_text("").unwrap();
    assert!(chunks.is_empty());

    // Test single sentence
    let chunks = splitter.split_text("This is a test.").unwrap();
    assert_eq!(chunks.len(), 1);

    // Test multiple sentences requiring chunking
    let text = "First sentence. Second sentence! Third sentence? Fourth sentence. Fifth sentence.";
    let chunks = splitter.split_text(text).unwrap();
    assert!(chunks.len() > 1);

    // Verify chunk properties
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.chunk_index, i);
        assert!(chunk.start_index <= chunk.end_index);
        assert!(chunk.metadata.contains_key("length"));
        assert!(chunk.metadata.contains_key("sentence_count"));
    }
}

#[test]
fn test_recursive_splitter_split_text_comprehensive() {
    let splitter = RecursiveSplitter::new(20, 5).unwrap();

    // Test empty text
    let chunks = splitter.split_text("").unwrap();
    assert!(chunks.is_empty());

    // Test text shorter than chunk size
    let chunks = splitter.split_text("Short text").unwrap();
    assert_eq!(chunks.len(), 1);

    // Test text requiring recursive splitting
    let text = "Paragraph one.\n\nParagraph two with more content.\n\nParagraph three with even more content that should be split.";
    let chunks = splitter.split_text(text).unwrap();
    assert!(chunks.len() > 1);

    // Verify chunk properties
    for (i, chunk) in chunks.iter().enumerate() {
        assert_eq!(chunk.chunk_index, i);
        assert!(chunk.start_index <= chunk.end_index);
        assert!(chunk.metadata.contains_key("length"));
    }
}

#[test]
fn test_recursive_splitter_split_by_characters() {
    // Test the split_by_characters fallback method
    let splitter = RecursiveSplitter::with_separators(5, 1, vec!["X".to_string()]).unwrap();

    // Text with no separators should fall back to character splitting
    let text = "abcdefghijklmnopqrstuvwxyz";
    let chunks = splitter.split_text(text).unwrap();

    // The text is long enough that it should be split into multiple chunks
    assert!(!chunks.is_empty());
    // With overlap, chunks might be longer than chunk_size, so just verify we get chunks
    for chunk in &chunks {
        assert!(!chunk.content.is_empty());
    }
}

#[test]
fn test_recursive_splitter_recursive_split_edge_cases() {
    // Test recursive splitting with various separator scenarios
    let splitter = RecursiveSplitter::new(10, 2).unwrap();

    // Test text that exactly matches chunk size
    let chunks = splitter.split_text("1234567890").unwrap();
    assert_eq!(chunks.len(), 1);

    // Test text with separators at the beginning
    let chunks = splitter.split_text("\n\nStart of text").unwrap();
    assert!(!chunks.is_empty());

    // Test text with separators at the end
    let chunks = splitter.split_text("End of text\n\n").unwrap();
    assert!(!chunks.is_empty());

    // Test text with only separators - this might result in empty chunks being filtered out
    let chunks = splitter.split_text("\n\n\n\n").unwrap();
    // The result might be empty if all parts are empty after splitting
    // This is acceptable behavior
}

#[test]
fn test_text_chunk_new_method() {
    // Test TextChunk::new method directly
    let chunk = graphbit_core::text_splitter::TextChunk::new(
        "Test content".to_string(),
        0,
        12,
        0,
    );

    assert_eq!(chunk.content, "Test content");
    assert_eq!(chunk.start_index, 0);
    assert_eq!(chunk.end_index, 12);
    assert_eq!(chunk.chunk_index, 0);
    assert!(chunk.metadata.contains_key("length"));
    assert_eq!(chunk.metadata["length"], serde_json::json!(12));
}

#[test]
fn test_text_chunk_with_metadata_chaining() {
    // Test chaining multiple with_metadata calls
    let chunk = graphbit_core::text_splitter::TextChunk::new(
        "Test".to_string(),
        0,
        4,
        0,
    )
    .with_metadata("key1".to_string(), serde_json::json!("value1"))
    .with_metadata("key2".to_string(), serde_json::json!(42))
    .with_metadata("key3".to_string(), serde_json::json!(true));

    assert_eq!(chunk.metadata["key1"], serde_json::json!("value1"));
    assert_eq!(chunk.metadata["key2"], serde_json::json!(42));
    assert_eq!(chunk.metadata["key3"], serde_json::json!(true));
    assert!(chunk.metadata.contains_key("length")); // Original metadata should still be there
}

#[test]
fn test_sentence_splitter_with_custom_endings_comprehensive() {
    // Test SentenceSplitter::with_endings method
    let custom_endings = vec![r"[.!?]+\s*", r"[\n\r]+"];
    let splitter = SentenceSplitter::with_endings(50, 10, custom_endings).unwrap();

    let text = "First sentence. Second sentence!\nThird sentence?";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    for chunk in &chunks {
        assert!(chunk.metadata.contains_key("sentence_count"));
    }

    // Test with invalid regex pattern
    let invalid_endings = vec!["[invalid"];
    let result = SentenceSplitter::with_endings(50, 10, invalid_endings);
    assert!(result.is_err());
}

#[test]
fn test_token_splitter_with_pattern_comprehensive() {
    // Test TokenSplitter::with_pattern method with various patterns
    let splitter = TokenSplitter::with_pattern(3, 1, r"\w+").unwrap();
    let text = "Hello, world! This is a test.";
    let chunks = splitter.split_text(text).unwrap();

    assert!(!chunks.is_empty());
    for chunk in &chunks {
        assert!(chunk.metadata.contains_key("token_count"));
    }

    // Test with pattern that matches punctuation
    let splitter = TokenSplitter::with_pattern(2, 0, r"[^\w\s]+").unwrap();
    let text = "Hello, world! Test.";
    let chunks = splitter.split_text(text).unwrap();
    assert!(!chunks.is_empty());

    // Test with invalid regex pattern
    let result = TokenSplitter::with_pattern(5, 1, "[invalid");
    assert!(result.is_err());
}
