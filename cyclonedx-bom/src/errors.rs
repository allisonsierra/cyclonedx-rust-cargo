#[derive(Debug, thiserror::Error)]
pub enum BomError {
    #[error("Failed to serialize BOM to JSON: {0}")]
    JsonSerializationError(#[from] serde_json::Error),

    #[error("Failed to serialize BOM to XML: {0}")]
    XmlSerializationError(String),
}
