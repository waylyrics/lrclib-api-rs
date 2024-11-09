use std::error::Error;

use lrclib_api_rs::{types::GetLyricsResponse, LRCLibAPI};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[test]
fn query_by_match() -> Result<()> {
    let api = LRCLibAPI::new();
    let req = api.get_lyrics(
        "憂鬱,你好!",
        "Priscilla Chan",
        Some("永遠是你的朋友"),
        Some(219),
    )?;
    let url = req.uri().to_string();

    let resp = reqwest::blocking::get(url)?.text()?;
    let resp: GetLyricsResponse = serde_json::from_str(&resp)?;
    assert!(resp.is_success());

    Ok(())
}
