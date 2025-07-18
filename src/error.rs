#[derive(thiserror::Error, Debug)]
pub enum TohayaError {
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
pub struct ParseError(pub String);

impl From<biblib::CitationError> for ParseError {
    fn from(value: biblib::CitationError) -> Self {
        Self(value.to_string())
    }
}

#[cfg(feature = "wasm")]
impl From<TohayaError> for wasm_bindgen::JsValue {
    fn from(value: TohayaError) -> Self {
        Self::from_str(&value.to_string())
    }
}
