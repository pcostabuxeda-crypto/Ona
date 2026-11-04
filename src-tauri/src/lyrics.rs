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
    let exact: Option<Value> = async {
        http.get("https://lrclib.net/api/get")
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
        .ok()
    }
    .await;
    // La coincidència exacta (títol, artista, àlbum i durada) falla sovint: si no és sincronitzada,
    // es busca per títol i artista i es tria la versió amb la mateixa durada.
    let data = match exact {
        Some(d) if d.get("syncedLyrics").and_then(Value::as_str).is_some_and(|l| !l.is_empty()) => d,
        exact => search(http, title, artist, duration_ms).await.or(exact)?,
    };

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

async fn search(http: &reqwest::Client, title: &str, artist: &str, duration_ms: u32) -> Option<Value> {
    let results: Vec<Value> = http
        .get("https://lrclib.net/api/search")
        .query(&[("track_name", clean_title(title).as_str()), ("artist_name", artist)])
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?
        .json()
        .await
        .ok()?;
    pick(&results, duration_ms as f64 / 1000.0).cloned()
}

/// Treu del títol el que no forma part de la cançó: «- Remastered 2011», «(feat. X)», «[Live]»...
fn clean_title(title: &str) -> String {
    let mut t = title.split(" - ").next().unwrap_or(title).to_string();
    for (open, close) in [('(', ')'), ('[', ']')] {
        while let (Some(a), Some(b)) = (t.find(open), t.find(close)) {
            if b < a {
                break;
            }
            t.replace_range(a..=b, "");
        }
    }
    let t = t.split_whitespace().collect::<Vec<_>>().join(" ");
    if t.is_empty() {
        title.to_string()
    } else {
        t
    }
}

/// La versió amb la durada més semblant (com a molt 4 s de diferència), millor si és sincronitzada.
fn pick(results: &[Value], duration_s: f64) -> Option<&Value> {
    let has = |r: &Value, k: &str| r.get(k).and_then(Value::as_str).is_some_and(|l| !l.trim().is_empty());
    results
        .iter()
        .filter_map(|r| {
            let d = (r.get("duration").and_then(Value::as_f64)? - duration_s).abs();
            (d <= 4.0 && (has(r, "syncedLyrics") || has(r, "plainLyrics"))).then_some((r, d))
        })
        // Primer les sincronitzades; entre iguals, la durada més propera.
        .min_by(|(a, da), (b, db)| has(b, "syncedLyrics").cmp(&has(a, "syncedLyrics")).then(da.total_cmp(db)))
        .map(|(r, _)| r)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neteja_el_titol() {
        assert_eq!(clean_title("Yesterday - Remastered 2009"), "Yesterday");
        assert_eq!(clean_title("Mood (feat. iann dior)"), "Mood");
        assert_eq!(clean_title("Tinc Fam de Tu [Live]"), "Tinc Fam de Tu");
        assert_eq!(clean_title("(Intro)"), "(Intro)");
    }

    #[test]
    fn tria_la_sincronitzada_amb_la_durada_bona() {
        let r = vec![
            json!({ "id": 1, "duration": 200.0, "syncedLyrics": "[00:01.00] a", "plainLyrics": "a" }),
            json!({ "id": 2, "duration": 241.0, "plainLyrics": "b" }),
            json!({ "id": 3, "duration": 243.0, "syncedLyrics": "[00:01.00] c", "plainLyrics": "c" }),
        ];
        // La sincronitzada guanya encara que estigui 3 s lluny; si totes són massa lluny, cap.
        assert_eq!(pick(&r, 240.0).unwrap()["id"], 3);
        assert!(pick(&r, 210.0).is_none());
    }
}
