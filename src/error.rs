use strum::EnumIs;
use thiserror::Error;

// Custom error type using thiserror crate
#[derive(Error, Debug, EnumIs)]
#[non_exhaustive]
pub enum ApiError {
    #[error("Url parse error: {0}")]
    UrlError(#[from] url::ParseError),
    #[error("http request error: {0}")]
    HttpRequestError(#[from] http::Error),
    #[error("JSON serialization error: {0}")]
    JsonSerializationError(#[from] serde_json::Error),
}
