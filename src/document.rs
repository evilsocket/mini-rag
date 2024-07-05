use std::{io::Read, path::Path};

use anyhow::Result;

use rayon::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    path: String,
    ident: String,
    #[serde(skip_deserializing, skip_serializing)]
    data: Option<String>,
}

impl Document {
    pub fn from_reader<R>(path: &Path, mut reader: R) -> Result<Self>
    where
        R: Read,
    {
        let path = std::fs::canonicalize(path.display().to_string())?
            .display()
            .to_string();

        let mut data = String::new();

        reader.read_to_string(&mut data)?;

        let data = Some(data);
        let ident = sha256::digest(data.as_ref().unwrap());
        Ok(Self { path, data, ident })
    }

    pub fn get_ident(&self) -> &str {
        &self.ident
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_data(&mut self) -> Result<&str> {
        if self.data.is_none() {
            self.data = Some(std::fs::read_to_string(&self.path)?);
        }

        Ok(self.data.as_ref().unwrap())
    }

    pub fn drop_data(&mut self) {
        self.data = None;
    }

    pub fn get_byte_size(&mut self) -> Result<usize> {
        Ok(self.get_data()?.as_bytes().len())
    }

    pub fn chunks(mut self, chunk_size: usize) -> Result<Vec<Document>> {
        return Ok(self
            .get_data()?
            .chars()
            .collect::<Vec<char>>()
            .par_chunks(chunk_size)
            .enumerate()
            .map(|(idx, chunk)| Document {
                ident: format!("{}@{}", self.ident, idx),
                path: format!("{}", self.path),
                data: Some(chunk.iter().collect::<String>()),
            })
            .collect());
    }
}
