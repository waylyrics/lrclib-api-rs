use std::error::Error;

use lrclib_api_rs::{LRCLibAPI, LyricsData};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[test]
fn search_keyword() -> Result<()> {
    let api = LRCLibAPI::new();
    let req = api.search_lyrics_query("周杰伦")?;
    let url = req.uri().to_string();

    let resp = reqwest::blocking::get(url)?.text()?;
    println!("{resp}");
    let _: Vec<LyricsData> = serde_json::from_str(&resp)?;

    Ok(())
}

#[test]
fn search_detailed() -> Result<()> {
    let api = LRCLibAPI::new();
    let req =
        api.search_lyrics_detailed("原神", Some("ChiliChill"), Some("Let the Wind Tell You"))?;
    let url = req.uri().to_string();

    let resp = reqwest::blocking::get(url)?.text()?;
    println!("{resp}");
    let _: Vec<LyricsData> = serde_json::from_str(&resp)?;

    Ok(())
}
