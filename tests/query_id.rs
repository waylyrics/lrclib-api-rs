use std::error::Error;

use lrclib_api_rs::{GetLyricsResponse, LRCLibAPI};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[test]
fn query_by_id() -> Result<()> {
    let req = LRCLibAPI::get_lyrics_by_id_request(1)?;
    let url = req.uri().to_string();

    let resp = reqwest::blocking::get(url)?.text()?;
    let resp: GetLyricsResponse = serde_json::from_str(&resp)?;
    assert!(resp.is_success());

    Ok(())
}

#[test]
fn query_by_id_error() -> Result<()> {
    let req = LRCLibAPI::get_lyrics_by_id_request(0)?;
    let url = req.uri().to_string();

    let resp = reqwest::blocking::get(url)?.text()?;
    let resp: GetLyricsResponse = serde_json::from_str(&resp)?;
    assert!(resp.is_error());

    Ok(())
}
