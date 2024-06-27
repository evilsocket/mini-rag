extern crate mini_rag;

use anyhow::Result;
use async_trait::async_trait;
use ollama_rs::Ollama;

use mini_rag::VectorStore;

struct OllamaEmbedder {
    client: Ollama,
    model: String,
}

impl OllamaEmbedder {
    fn new() -> Self {
        let client = Ollama::new("http://localhost", 11434);
        let model = "all-minilm".to_string();
        Self { model, client }
    }
}

#[async_trait]
impl mini_rag::Embedder for OllamaEmbedder {
    async fn embed(&self, text: &str) -> Result<mini_rag::Embeddings> {
        let resp = self
            .client
            .generate_embeddings(self.model.to_string(), text.to_string(), None)
            .await?;

        Ok(mini_rag::Embeddings::from(resp.embeddings))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // db configuration
    let config = mini_rag::Configuration {
        // folder containing documents to import
        source_path: "test_documents".to_string(),
        // folder used to persist the vector database
        data_path: "/tmp".to_string(),
        // disable chunking, process whole documents
        chunk_size: None,
    };

    // the object creating the embeddings
    let embedder = Box::new(OllamaEmbedder::new());

    let mut store = VectorStore::new(embedder, config)?;

    // this will import any new documents
    store.import_new_documents().await?;

    let mut docs = store.retrieve("darme pinter", 1).await?;

    println!("\n{}", docs[0].0.get_data()?);

    Ok(())
}
