use std::{collections::VecDeque, fs::File, path::PathBuf};

use anyhow::Result;

use crate::Document;

pub(crate) fn import_document_from(path: &PathBuf) -> Result<Document> {
    let ext = if let Some(ext) = path.extension() {
        ext.to_str().unwrap()
    } else {
        "<unknown>"
    }
    .to_lowercase();

    // create a buffered reader depending on file type
    let reader: Box<dyn std::io::Read> = match ext.as_str() {
        "txt" => {
            // read as it is
            Box::new(File::open(path)?)
        }
        #[cfg(feature = "pdf")]
        "pdf" => {
            // extract text from pdf
            let pdf = lopdf::Document::load(path)?;
            let pages = pdf.get_pages();
            let mut parts = vec![];

            for (i, _) in pages.iter().enumerate() {
                let page_number = (i + 1) as u32;
                let page_text = pdf.extract_text(&[page_number]).map_err(|e| {
                    anyhow!(
                        "can't parse page {} of {}: {:?}",
                        page_number,
                        path.display(),
                        e
                    )
                })?;
                parts.push(page_text);
            }
            // https://stackoverflow.com/questions/32674905/pass-string-to-function-taking-read-trait
            Box::new(VecDeque::from(parts.join("\n\n").into_bytes()))
        }

        _ => return Err(anyhow!("file extension '{ext}' not handled")),
    };

    Document::from_reader(path, reader)
}
