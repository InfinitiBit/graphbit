//! Embeddings module for GraphBit Node.js bindings

pub(crate) mod client;
pub(crate) mod config;

pub use client::EmbeddingClient;
pub use config::EmbeddingConfig;
