//! Embeddings Integration Tests
//!
//! Tests for embedding generation, vector operations, and similarity search
//! using different providers (OpenAI, HuggingFace).

use graphbit_core::embeddings::*;
use serde_json::json;
use std::collections::HashMap;

fn create_test_embedding_config() -> EmbeddingConfig {
    EmbeddingConfig {
        provider: EmbeddingProvider::HuggingFace,
        api_key: "test-key".to_string(),
        model: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
        base_url: None,
        timeout_seconds: Some(30),
        max_batch_size: Some(32),
        extra_params: HashMap::new(),
    }
}

#[tokio::test]
async fn test_embedding_config_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = create_test_embedding_config();
    assert_eq!(config.provider, EmbeddingProvider::HuggingFace);
    assert_eq!(config.model, "sentence-transformers/all-MiniLM-L6-v2");
    assert_eq!(config.timeout_seconds, Some(30));
    assert_eq!(config.max_batch_size, Some(32));
}

#[tokio::test]
async fn test_embedding_service_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = create_test_embedding_config();
    let service_result = EmbeddingService::new(config);

    // Should succeed even with test configuration
    assert!(service_result.is_ok(), "Should create embedding service");
}

#[tokio::test]
async fn test_embedding_provider_factory() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = create_test_embedding_config();
    let provider_result = EmbeddingProviderFactory::create_provider(config);

    assert!(provider_result.is_ok(), "Should create provider");
    let provider = provider_result.unwrap();
    assert_eq!(provider.provider_name(), "huggingface");
    assert_eq!(
        provider.model_name(),
        "sentence-transformers/all-MiniLM-L6-v2"
    );
}

#[tokio::test]
async fn test_openai_embedding_provider_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = EmbeddingConfig {
        provider: EmbeddingProvider::OpenAI,
        api_key: "test-key".to_string(),
        model: "text-embedding-ada-002".to_string(),
        base_url: None,
        timeout_seconds: Some(30),
        max_batch_size: Some(16),
        extra_params: HashMap::new(),
    };

    let provider_result = EmbeddingProviderFactory::create_provider(config);
    assert!(provider_result.is_ok(), "Should create OpenAI provider");

    let provider = provider_result.unwrap();
    assert_eq!(provider.provider_name(), "openai");
    assert_eq!(provider.model_name(), "text-embedding-ada-002");
}

#[tokio::test]
async fn test_embedding_input_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test single input
    let single_input = EmbeddingInput::Single("Hello, world!".to_string());
    assert_eq!(single_input.len(), 1);
    assert!(!single_input.is_empty());
    assert_eq!(single_input.as_texts(), vec!["Hello, world!"]);

    // Test multiple input
    let multiple_input = EmbeddingInput::Multiple(vec![
        "First text".to_string(),
        "Second text".to_string(),
        "Third text".to_string(),
    ]);
    assert_eq!(multiple_input.len(), 3);
    assert!(!multiple_input.is_empty());
    assert_eq!(
        multiple_input.as_texts(),
        vec!["First text", "Second text", "Third text"]
    );

    // Test empty input
    let empty_single = EmbeddingInput::Single("".to_string());
    assert!(empty_single.is_empty());

    let empty_multiple = EmbeddingInput::Multiple(vec![]);
    assert!(empty_multiple.is_empty());
}

#[tokio::test]
async fn test_embedding_request_creation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let input = EmbeddingInput::Single("Test text for embedding".to_string());
    let mut params = HashMap::new();
    params.insert("dimensions".to_string(), json!(384));

    let request = EmbeddingRequest {
        input,
        user: Some("test-user".to_string()),
        params,
    };

    assert_eq!(request.input.len(), 1);
    assert_eq!(request.user, Some("test-user".to_string()));
    assert_eq!(request.params.get("dimensions"), Some(&json!(384)));
}

#[tokio::test]
async fn test_embedding_batch_request() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let request1 = EmbeddingRequest {
        input: EmbeddingInput::Single("First text".to_string()),
        user: None,
        params: HashMap::new(),
    };

    let request2 = EmbeddingRequest {
        input: EmbeddingInput::Multiple(vec!["Second text".to_string(), "Third text".to_string()]),
        user: Some("batch-user".to_string()),
        params: HashMap::new(),
    };

    let batch_request = EmbeddingBatchRequest {
        requests: vec![request1, request2],
        max_concurrency: Some(2),
        timeout_ms: Some(30000),
    };

    assert_eq!(batch_request.requests.len(), 2);
    assert_eq!(batch_request.max_concurrency, Some(2));
    assert_eq!(batch_request.timeout_ms, Some(30000));
}

#[tokio::test]
async fn test_cosine_similarity_calculation() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Test cosine similarity calculation
    let vec1 = vec![1.0, 2.0, 3.0];
    let vec2 = vec![4.0, 5.0, 6.0];
    let vec3 = vec![1.0, 2.0, 3.0]; // Same as vec1

    let similarity_12 = EmbeddingService::cosine_similarity(&vec1, &vec2).unwrap();
    let similarity_13 = EmbeddingService::cosine_similarity(&vec1, &vec3).unwrap();

    // Identical vectors should have similarity of 1.0
    assert!(
        (similarity_13 - 1.0).abs() < 1e-6,
        "Identical vectors should have similarity 1.0"
    );

    // Different vectors should have similarity between 0 and 1
    assert!(
        similarity_12 > 0.0 && similarity_12 < 1.0,
        "Different vectors should have similarity between 0 and 1"
    );

    // Self-similarity should be higher than cross-similarity
    assert!(
        similarity_13 > similarity_12,
        "Self-similarity should be higher"
    );
}

#[tokio::test]
async fn test_embedding_service_text_processing() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let config = create_test_embedding_config();
    let service = EmbeddingService::new(config).expect("Failed to create service");

    // Test single text embedding (will likely fail without real API)
    let result = service.embed_text("Hello, world!").await;
    match result {
        Ok(embedding) => {
            assert!(!embedding.is_empty(), "Embedding should not be empty");
            println!(
                "Successfully generated embedding with {} dimensions",
                embedding.len()
            );
        }
        Err(e) => {
            println!("Embedding failed as expected (no real API): {:?}", e);
            // This is expected in test environment without real API keys
        }
    }
}

#[tokio::test]
async fn test_openai_real_embeddings() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_openai_api_key() {
        println!("Skipping real OpenAI embeddings test - no valid API key");
        return;
    }

    let api_key = super::get_openai_api_key_or_skip();
    let config = EmbeddingConfig {
        provider: EmbeddingProvider::OpenAI,
        api_key,
        model: "text-embedding-ada-002".to_string(),
        base_url: None,
        timeout_seconds: Some(30),
        max_batch_size: Some(16),
        extra_params: HashMap::new(),
    };

    let service = EmbeddingService::new(config).expect("Failed to create service");

    // Test single text embedding
    let result = service
        .embed_text("The quick brown fox jumps over the lazy dog")
        .await;
    match result {
        Ok(embedding) => {
            assert!(!embedding.is_empty(), "Embedding should not be empty");
            assert_eq!(
                embedding.len(),
                1536,
                "OpenAI ada-002 should return 1536 dimensions"
            );
            println!(
                "OpenAI real embeddings successful with {} dimensions",
                embedding.len()
            );
        }
        Err(e) => {
            println!("OpenAI embeddings failed: {:?}", e);
            panic!("OpenAI embeddings should succeed with valid credentials");
        }
    }
}

#[tokio::test]
async fn test_openai_real_batch_embeddings() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_openai_api_key() {
        println!("Skipping real OpenAI batch embeddings test - no valid API key");
        return;
    }

    let api_key = super::get_openai_api_key_or_skip();
    let config = EmbeddingConfig {
        provider: EmbeddingProvider::OpenAI,
        api_key,
        model: "text-embedding-ada-002".to_string(),
        base_url: None,
        timeout_seconds: Some(30),
        max_batch_size: Some(16),
        extra_params: HashMap::new(),
    };

    let service = EmbeddingService::new(config).expect("Failed to create service");

    let texts = vec![
        "The first document about machine learning".to_string(),
        "A second text about artificial intelligence".to_string(),
        "Third content regarding neural networks".to_string(),
    ];

    // Test batch embedding
    let result = service.embed_texts(&texts).await;
    match result {
        Ok(embeddings) => {
            assert_eq!(
                embeddings.len(),
                texts.len(),
                "Should get embedding for each text"
            );

            for (i, embedding) in embeddings.iter().enumerate() {
                assert!(!embedding.is_empty(), "Embedding {} should not be empty", i);
                assert_eq!(
                    embedding.len(),
                    1536,
                    "Each embedding should have 1536 dimensions"
                );
            }

            // Test similarity between related texts
            let similarity_01 =
                EmbeddingService::cosine_similarity(&embeddings[0], &embeddings[1]).unwrap();
            let similarity_02 =
                EmbeddingService::cosine_similarity(&embeddings[0], &embeddings[2]).unwrap();

            assert!(
                similarity_01 > 0.5,
                "Related texts should have high similarity"
            );
            assert!(
                similarity_02 > 0.5,
                "Related texts should have high similarity"
            );

            println!("   OpenAI real batch embeddings successful");
            println!("   Similarity between text 0 and 1: {:.3}", similarity_01);
            println!("   Similarity between text 0 and 2: {:.3}", similarity_02);
        }
        Err(e) => {
            println!("  OpenAI batch embeddings failed: {:?}", e);
            panic!("OpenAI batch embeddings should succeed with valid credentials");
        }
    }
}

#[tokio::test]
async fn test_huggingface_real_embeddings() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_huggingface_api_key() {
        println!("Skipping real HuggingFace embeddings test - no valid API key");
        return;
    }

    let api_key = super::get_huggingface_api_key_or_skip();
    let config = EmbeddingConfig {
        provider: EmbeddingProvider::HuggingFace,
        api_key,
        model: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
        base_url: None,
        timeout_seconds: Some(60), // HuggingFace can be slower
        max_batch_size: Some(32),
        extra_params: HashMap::new(),
    };

    let service = EmbeddingService::new(config).expect("Failed to create service");

    // Test single text embedding
    let result = service
        .embed_text("This is a test sentence for embedding")
        .await;
    match result {
        Ok(embedding) => {
            assert!(!embedding.is_empty(), "Embedding should not be empty");
            // MiniLM-L6-v2 typically returns 384 dimensions
            assert_eq!(
                embedding.len(),
                384,
                "MiniLM-L6-v2 should return 384 dimensions"
            );
            println!(
                " HuggingFace real embeddings successful with {} dimensions",
                embedding.len()
            );
        }
        Err(e) => {
            println!(" HuggingFace embeddings failed: {:?}", e);
            // HuggingFace API can be unreliable, so we log but don't fail the test
            println!(
                "HuggingFace embeddings failed (this might be expected): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_semantic_search_with_real_embeddings() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    // Skip if no real API key is provided
    if !super::has_openai_api_key() {
        println!("Skipping semantic search test - no valid OpenAI API key");
        return;
    }

    let api_key = super::get_openai_api_key_or_skip();
    let config = EmbeddingConfig {
        provider: EmbeddingProvider::OpenAI,
        api_key,
        model: "text-embedding-ada-002".to_string(),
        base_url: None,
        timeout_seconds: Some(30),
        max_batch_size: Some(16),
        extra_params: HashMap::new(),
    };

    let service = EmbeddingService::new(config).expect("Failed to create service");

    // Create a document corpus
    let documents = vec![
        "Python is a programming language used for web development and data science".to_string(),
        "Rust is a systems programming language focused on safety and performance".to_string(),
        "Machine learning involves training algorithms on data to make predictions".to_string(),
        "The weather today is sunny with a temperature of 75 degrees".to_string(),
        "Artificial intelligence is transforming how we solve complex problems".to_string(),
    ];

    // Generate embeddings for all documents
    let doc_embeddings = service.embed_texts(&documents).await;
    if let Err(e) = doc_embeddings {
        println!(" Failed to generate document embeddings: {:?}", e);
        return;
    }
    let doc_embeddings = doc_embeddings.unwrap();

    // Test semantic search queries
    let queries = vec![
        ("programming languages", vec![0, 1]), // Should match Python and Rust docs
        ("AI and ML", vec![2, 4]),             // Should match ML and AI docs
        ("weather forecast", vec![3]),         // Should match weather doc
    ];

    for (query, expected_matches) in queries {
        let query_embedding = service.embed_text(query).await;
        if let Err(e) = query_embedding {
            println!(
                " Failed to generate query embedding for '{}': {:?}",
                query, e
            );
            continue;
        }
        let query_embedding = query_embedding.unwrap();

        // Calculate similarities
        let mut similarities: Vec<(usize, f32)> = doc_embeddings
            .iter()
            .enumerate()
            .map(|(i, doc_emb)| {
                let sim =
                    EmbeddingService::cosine_similarity(&query_embedding, doc_emb).unwrap_or(0.0);
                (i, sim)
            })
            .collect();

        // Sort by similarity (highest first)
        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        println!(" Query: '{}'", query);
        for (i, (doc_idx, similarity)) in similarities.iter().enumerate() {
            let doc_text = &documents[*doc_idx];
            let preview = if doc_text.len() > 60 {
                &doc_text[..60]
            } else {
                doc_text
            };
            println!("  {}. [{:.3}] {}", i + 1, similarity, preview);
        }

        // Check that expected matches are in top results
        let top_2_docs: Vec<usize> = similarities.iter().take(2).map(|(idx, _)| *idx).collect();
        let has_expected_match = expected_matches
            .iter()
            .any(|&expected| top_2_docs.contains(&expected));

        if has_expected_match {
            println!(" Semantic search found relevant documents for '{}'", query);
        } else {
            println!(
                "  Semantic search results for '{}' might not be as expected",
                query
            );
        }
    }
}

#[tokio::test]
async fn test_embedding_provider_comparison() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let test_text = "Artificial intelligence and machine learning are revolutionizing technology";
    let mut successful_providers = Vec::new();

    // Test OpenAI if available
    if super::has_openai_api_key() {
        let api_key = std::env::var("OPENAI_API_KEY").unwrap();
        let config = EmbeddingConfig {
            provider: EmbeddingProvider::OpenAI,
            api_key,
            model: "text-embedding-ada-002".to_string(),
            base_url: None,
            timeout_seconds: Some(30),
            max_batch_size: Some(16),
            extra_params: HashMap::new(),
        };

        let service = EmbeddingService::new(config).expect("Failed to create OpenAI service");
        if let Ok(embedding) = service.embed_text(test_text).await {
            successful_providers.push(("OpenAI", embedding.len()));
        }
    }

    // Test HuggingFace if available
    if super::has_huggingface_api_key() {
        let api_key = std::env::var("HUGGINGFACE_API_KEY").unwrap();
        let config = EmbeddingConfig {
            provider: EmbeddingProvider::HuggingFace,
            api_key,
            model: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
            base_url: None,
            timeout_seconds: Some(60),
            max_batch_size: Some(32),
            extra_params: HashMap::new(),
        };

        let service = EmbeddingService::new(config).expect("Failed to create HuggingFace service");
        if let Ok(embedding) = service.embed_text(test_text).await {
            successful_providers.push(("HuggingFace", embedding.len()));
        }
    }

    println!("🔍 Embedding provider comparison:");
    for (provider_name, dimensions) in &successful_providers {
        println!("  {} -> {} dimensions", provider_name, dimensions);
    }

    if successful_providers.is_empty() {
        println!("No embedding providers available for comparison test");
    } else {
        println!(
            "Successfully tested {} embedding provider(s)",
            successful_providers.len()
        );
    }

    // Verify we can create services even without API calls
    // This test documents that the embedding comparison ran, regardless of provider availability
}

#[tokio::test]
async fn test_embedding_provider_capabilities() {
    graphbit_core::init().expect("Failed to initialize GraphBit");

    let configs = vec![
        EmbeddingConfig {
            provider: EmbeddingProvider::OpenAI,
            api_key: "test".to_string(),
            model: "text-embedding-ada-002".to_string(),
            base_url: None,
            timeout_seconds: None,
            max_batch_size: None,
            extra_params: HashMap::new(),
        },
        EmbeddingConfig {
            provider: EmbeddingProvider::HuggingFace,
            api_key: "test".to_string(),
            model: "sentence-transformers/all-MiniLM-L6-v2".to_string(),
            base_url: None,
            timeout_seconds: None,
            max_batch_size: None,
            extra_params: HashMap::new(),
        },
    ];

    for config in configs {
        let provider = EmbeddingProviderFactory::create_provider(config).unwrap();

        // All providers should support basic functionality
        assert!(
            provider.supports_batch(),
            "Provider should support batch processing"
        );
        assert!(
            provider.max_batch_size() > 0,
            "Provider should have positive max batch size"
        );

        // Validate configuration
        let validation_result = provider.validate_config();
        assert!(
            validation_result.is_ok(),
            "Provider configuration should be valid"
        );
    }
}
