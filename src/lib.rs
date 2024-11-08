use std::borrow::Cow;

use http::{Method, Request};

mod types;
pub use types::*;

const BASE_URL: &str = "https://lrclib.net";
const DEFAULT_USER_AGENT: &str = concat!("LRCAPIWrapper/", env!("CARGO_PKG_VERSION"));

#[derive(Clone, Debug)]
pub struct LRCLibAPI {
    base_url: String,
    user_agent: String,
}

impl Default for LRCLibAPI {
    fn default() -> Self {
        Self {
            base_url: BASE_URL.into(),
            user_agent: DEFAULT_USER_AGENT.into(),
        }
    }
}

impl LRCLibAPI {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_base_url(base_url: String) -> Self {
        Self {
            base_url,
            ..Default::default()
        }
    }

    pub fn with_user_agent(user_agent: String) -> Self {
        Self {
            user_agent,
            ..Default::default()
        }
    }

    pub fn with_parts(base_url: String, user_agent: String) -> Self {
        Self {
            base_url,
            user_agent,
        }
    }
}

impl LRCLibAPI {
    pub fn get_lyrics(
        &self,
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

        let uri = format!("{}/api/get?{}", &self.base_url, query_string);

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("User-Agent", &self.user_agent)
            .body(())
            .map_err(ApiError::from)
    }

    pub fn get_lyrics_by_id(&self, id: u64) -> Result<Request<()>, ApiError> {
        let uri = format!("{}/api/get/{}", &self.base_url, id);

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("User-Agent", &self.user_agent)
            .body(())
            .map_err(ApiError::from)
    }

    pub fn search_lyrics(
        &self,
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

        let uri = format!("{}/api/search?{}", &self.base_url, query_string);

        Request::builder()
            .method(Method::GET)
            .uri(uri)
            .header("User-Agent", &self.user_agent)
            .body(())
            .map_err(ApiError::from)
    }

    pub fn request_publish_challenge(&self) -> Result<Request<()>, ApiError> {
        let uri = format!("{}/api/request-challenge", &self.base_url);

        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("User-Agent", &self.user_agent)
            .body(())
            .map_err(ApiError::from)
    }

    pub fn publish_lyrics(
        &self,
        lyrics: &LyricsData,
        publish_token: &str,
    ) -> Result<Request<String>, ApiError> {
        let uri = format!("{}/api/publish", &self.base_url);
        if lyrics.album_name.is_none() || lyrics.duration.is_none() {
            Err(ApiError::MissingFieldExists)?
        }

        let body = serde_json::to_string(lyrics)?;

        Request::builder()
            .method(Method::POST)
            .uri(uri)
            .header("User-Agent", &self.user_agent)
            .header("X-Publish-Token", publish_token)
            .header("Content-Type", "application/json")
            .body(body)
            .map_err(ApiError::from)
    }
}
