use std::error::Error;

use lrclib_api_rs::{LRCLibAPI, LyricsData, PublishResponse};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[test]
fn publish_lyric() -> Result<()> {
    let req = LRCLibAPI::publish_lyrics_request(
        &LyricsData {
            id: 0,
            name: "你好".into(),
            track_name: "你好压".into(),
            artist_name: "初音未来".into(),
            instrumental: false,
            plain_lyrics: "你好".into(),
            synced_lyrics: "[00:00.00]你好".into(),
            album_name: Some("专辑".into()),
            duration: Some(0.0),
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

#[test]
fn publish_lyric_field_missing() -> Result<()> {
    let req = LRCLibAPI::publish_lyrics_request(
        &LyricsData {
            id: 0,
            name: "你好".into(),
            track_name: "你好压".into(),
            artist_name: "初音未来".into(),
            instrumental: false,
            plain_lyrics: "你好".into(),
            synced_lyrics: "[00:00.00]你好".into(),
            album_name: Some("专辑".into()),
            duration: None,
        },
        "INVALID_TOKEN",
    );

    assert!(req.unwrap_err().is_missing_field_exists());

    Ok(())
}

#[test]
fn publish_lyric_field_missing_2() -> Result<()> {
    let req = LRCLibAPI::publish_lyrics_request(
        &LyricsData {
            id: 0,
            name: "你好".into(),
            track_name: "你好压".into(),
            artist_name: "初音未来".into(),
            instrumental: false,
            plain_lyrics: "你好".into(),
            synced_lyrics: "[00:00.00]你好".into(),
            album_name: None,
            duration: Some(0.),
        },
        "INVALID_TOKEN",
    );

    assert!(req.unwrap_err().is_missing_field_exists());

    Ok(())
}
