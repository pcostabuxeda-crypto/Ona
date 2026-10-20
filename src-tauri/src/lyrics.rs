//! Lletres en un format únic per a la interfície:
//! `{ synced: bool, source: String, lines: [{ t: ms, text }], ttml?: String }`
//! (`ttml` porta temps per síl·laba i el parseja la interfície amb @braccato/parsers).

use serde_json::{json, Value};

/// API pública i gratuïta de Better Lyrics (https://docs.betterlyrics.org): TTML amb temps per síl·laba.
pub async fn from_betterlyrics(
    http: &reqwest::Client,
    title: &str,
    artist: &str,
    album: &str,
    duration_ms: u32,
) -> Option<Value> {
    let duration = (duration_ms / 1000).to_string();
    let data: Value = http
        .get("https://api.betterlyrics.org/getLyrics")
        .query(&[("s", title), ("a", artist), ("al", album), ("d", duration.as_str())])
        .timeout(std::time::Duration::from_secs(6))
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json()
        .await
        .ok()?;
    let ttml = data.get("ttml")?.as_str()?.trim();
    if !ttml.contains("<p") {
        return None;
    }
    Some(json!({ "synced": true, "source": "Better Lyrics", "lines": [], "ttml": ttml }))
}

/// Resposta de `color-lyrics/v2` de Spotify.
pub fn from_spotify(bytes: &[u8]) -> Option<Value> {
    let data: Value = serde_json::from_slice(bytes).ok()?;
    let lyrics = data.get("lyrics")?;
    let synced = lyrics.get("syncType")?.as_str()? == "LINE_SYNCED";
    let lines: Vec<Value> = lyrics
        .get("lines")?
        .as_array()?
        .iter()
        .filter_map(|line| {
            let t: u64 = line.get("startTimeMs")?.as_str()?.parse().ok()?;
            let text = line.get("words")?.as_str()?.trim();
            // Spotify marca els silencis instrumentals amb "♪".
            let text = if text == "♪" { "" } else { text };
            Some(json!({ "t": t, "text": text }))
        })
        .collect();
    if lines.is_empty() {
        return None;
    }
    Some(json!({ "synced": synced, "source": "Spotify", "lines": lines }))
}

pub async fn from_lrclib(
    http: &reqwest::Client,
    title: &str,
    artist: &str,
    album: &str,
    duration_ms: u32,
) -> Option<Value> {
    let duration = (duration_ms / 1000).to_string();
    let data: Value = http
        .get("https://lrclib.net/api/get")
        .query(&[
            ("track_name", title),
            ("artist_name", artist),
            ("album_name", album),
            ("duration", duration.as_str()),
        ])
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json()
        .await
        .ok()?;

    if let Some(lrc) = data.get("syncedLyrics").and_then(Value::as_str) {
        let lines = parse_lrc(lrc);
        if !lines.is_empty() {
            return Some(json!({ "synced": true, "source": "LRCLIB", "lines": lines }));
        }
    }
    let plain = data.get("plainLyrics").and_then(Value::as_str)?;
    let lines: Vec<Value> = plain
        .lines()
        .map(|l| json!({ "t": 0, "text": l.trim() }))
        .collect();
    Some(json!({ "synced": false, "source": "LRCLIB", "lines": lines }))
}

/// Línies `[mm:ss.xx] text` (n'hi pot haver més d'una marca per línia).
fn parse_lrc(lrc: &str) -> Vec<Value> {
    let mut out: Vec<(u64, String)> = Vec::new();
    for raw in lrc.lines() {
        let mut rest = raw.trim();
        let mut stamps = Vec::new();
        while let Some(stripped) = rest.strip_prefix('[') {
            let Some(end) = stripped.find(']') else { break };
            if let Some(ms) = parse_stamp(&stripped[..end]) {
                stamps.push(ms);
            }
            rest = &stripped[end + 1..];
        }
        for ms in stamps {
            out.push((ms, rest.trim().to_string()));
        }
    }
    out.sort_by_key(|(t, _)| *t);
    out.into_iter()
        .map(|(t, text)| json!({ "t": t, "text": text }))
        .collect()
}

fn parse_stamp(s: &str) -> Option<u64> {
    let (min, sec) = s.split_once(':')?;
    let min: u64 = min.parse().ok()?;
    let sec: f64 = sec.parse().ok()?;
    Some(min * 60_000 + (sec * 1000.0).round() as u64)
}
