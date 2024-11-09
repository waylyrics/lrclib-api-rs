use serde::{Deserialize, Serialize};
use strum::EnumIs;

// Define response structs based on API sample responses.
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LyricsData {
    /// lyrics id in LRCLib database
    pub id: u64,
    /// maybe deprecated fallback of track_name
    pub name: String,
    /// Title of the track
    pub track_name: String,
    /// Track's artist name
    pub artist_name: String,
    /// Track's album name
    pub album_name: Option<String>,
    /// duration in seconds
    pub duration: Option<f64>,
    /// instrumental means both plain_lyrics and synced_lyrics are empty.
    pub instrumental: bool,
    /// plain lyrics without timestamp
    pub plain_lyrics: Option<String>,
    /// synced lyrics as standard LRC
    pub synced_lyrics: Option<String>,
}

/// check comments on LyricsData.
#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LyricsPublishData {
    pub track_name: String,
    pub artist_name: String,
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
