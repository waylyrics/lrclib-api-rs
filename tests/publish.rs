use std::error::Error;

use lrclib_api_rs::{
    types::{LyricsPublishData, PublishResponse},
    LRCLibAPI,
};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[test]
fn publish_lyric() -> Result<()> {
    let api = LRCLibAPI::new();
    let req = api.publish_lyrics(
        &LyricsPublishData {
            id: 0,
            name: "你好".into(),
            track_name: "你好压".into(),
            artist_name: "初音未来".into(),
            instrumental: false,
            plain_lyrics: "你好".into(),
            synced_lyrics: "[00:00.00]你好".into(),
            album_name: "专辑".into(),
            duration: 0.0,
        },
        "INVALID_TOKEN",
    )?;
    let reqwest_req = req.try_into()?;

    let client = reqwest::blocking::Client::new();
    let resp = client.execute(reqwest_req)?.text()?;
    let pub_resp: PublishResponse = serde_json::from_str(&resp)?;
    assert!(pub_resp.is_error());

    Ok(())
}
