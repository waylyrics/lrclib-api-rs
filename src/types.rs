use serde::{Deserialize, Serialize};
use strum::EnumIs;
use thiserror::Error;

// Define response structs based on API sample responses.
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LyricsData {
    pub id: u64,
    pub name: String,
    pub track_name: String,
    pub artist_name: String,
    pub instrumental: bool,
    pub album_name: Option<String>,
    pub duration: Option<f64>,
    pub plain_lyrics: Option<String>,
    pub synced_lyrics: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LyricsPublishData {
    pub id: u64,
    pub name: String,
    pub track_name: String,
    pub artist_name: String,
    pub instrumental: bool,
    pub album_name: String,
    pub duration: f64,
    pub plain_lyrics: String,
    pub synced_lyrics: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    pub status_code: u32,
    pub name: String,
    pub message: String,
}

// Enum for wrapping successful and error responses for the /api/get endpoint.
#[derive(Debug, Deserialize, EnumIs)]
#[serde(untagged)]
pub enum GetLyricsResponse {
    Success(LyricsData),
    Error(ErrorResponse),
}

// Enum for wrapping success and error responses for the /api/publish endpoint.
#[derive(Debug, Deserialize, EnumIs)]
#[serde(untagged)]
pub enum PublishResponse {
    Created,              // Represents a successful publish (201 Created).
    Error(ErrorResponse), // Error response
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishChallenge {
    pub prefix: String,
    pub target: String,
}

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
