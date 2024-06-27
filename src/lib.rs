#[macro_use]
extern crate anyhow;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

mod document;
mod metrics;
mod naive;

pub use document::*;
pub use metrics::*;
pub use naive::*;

mod import;

pub type Embeddings = Vec<f64>;

#[async_trait]
pub trait Embedder: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Embeddings>;
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Configuration {
    pub source_path: String,
    pub data_path: String,
    pub chunk_size: Option<usize>,
}
