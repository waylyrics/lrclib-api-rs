use std::borrow::Cow;

use http::{Method, Request};

mod types;
pub use types::*;

const BASE_URL: &str = "https://lrclib.net";

pub struct LRCLibAPI;

impl LRCLibAPI {
    pub fn get_lyrics_request(
        track_name: &str,
        artist_name: &str,
        album_name: Option<&str>,
        duration: Option<u64>,
    ) -> Result<Request<()>, ApiError> {
        let mut query_params: Vec<(&str, Cow<'_, str>)> = vec![
            ("track_name", track_name.into()),
            ("artist_name", artist_name.into()),
        ];

        if let Some(album) = album_name {
            query_params.push(("album_name", album.into()));
        }
        if let Some(dur) = duration {
            let dur_s = dur.to_string();
            query_params.push(("duration", Cow::Owned(dur_s)));
        }

        let query_string: String = query_params
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&");

        let uri = format!("{}/api/get?{}", BASE_URL, query_string);

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("User-Agent", "LRCAPIWrapper/0.1.0")
            .body(())
            .map_err(ApiError::from)
    }

    pub fn get_lyrics_by_id_request(id: u64) -> Result<Request<()>, ApiError> {
        let uri = format!("{}/api/get/{}", BASE_URL, id);

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("User-Agent", "LRCAPIWrapper/0.1.0")
            .body(())
            .map_err(ApiError::from)
    }

    pub fn search_lyrics_request(
        query: Option<&str>,
        track_name: Option<&str>,
        artist_name: Option<&str>,
        album_name: Option<&str>,
    ) -> Result<Request<()>, ApiError> {
        let mut query_params = vec![];

        if let Some(q) = query {
            query_params.push(("q", q));
        }
        if let Some(track) = track_name {
            query_params.push(("track_name", track));
        }
        if let Some(artist) = artist_name {
            query_params.push(("artist_name", artist));
        }
        if let Some(album) = album_name {
            query_params.push(("album_name", album));
        }

        let query_string: String = query_params
            .into_iter()
            .map(|(key, value)| format!("{}={}", key, value))
            .collect::<Vec<String>>()
            .join("&");

        let uri = format!("{}/api/search?{}", BASE_URL, query_string);

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("User-Agent", "LRCAPIWrapper/0.1.0")
            .body(())
            .map_err(ApiError::from)
    }

    pub fn request_publish_challenge_request() -> Result<Request<()>, ApiError> {
        let uri = format!("{}/api/request-challenge", BASE_URL);

        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("User-Agent", "LRCAPIWrapper/0.1.0")
            .body(())
            .map_err(ApiError::from)
    }

    pub fn publish_lyrics_request(
        lyrics: &LyricsData,
        publish_token: &str,
    ) -> Result<Request<String>, ApiError> {
        let uri = format!("{}/api/publish", BASE_URL);

        let body = serde_json::to_string(lyrics)?;

        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("User-Agent", "LRCAPIWrapper/0.1.0")
            .header("X-Publish-Token", publish_token)
            .header("Content-Type", "application/json")
            .body(body)
            .map_err(ApiError::from)
    }
}
