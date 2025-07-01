//! Embeddings module for GraphBit Python bindings

pub(crate) mod client;
pub(crate) mod config;

pub use client::EmbeddingClient;
pub use config::EmbeddingConfig;
