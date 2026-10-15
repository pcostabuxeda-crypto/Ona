//! Fitxers d'àudio de la carpeta Música de l'usuari (l'únic lloc que el protocol d'assets deixa llegir).

use std::path::{Path, PathBuf};

use serde_json::{json, Value};

const EXTS: &[&str] = &["mp3", "flac", "m4a", "aac", "ogg", "opus", "wav"];
// shortcut: 5.000 fitxers i 6 nivells de carpetes, prou per a una col·lecció personal.
const MAX_FILES: usize = 5000;
const MAX_DEPTH: usize = 6;

fn walk(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        if out.len() >= MAX_FILES {
            return;
        }
        let p = e.path();
        if p.is_dir() {
            if depth < MAX_DEPTH {
                walk(&p, depth + 1, out);
            }
        } else if p
            .extension()
            .and_then(|x| x.to_str())
            .is_some_and(|x| EXTS.contains(&x.to_ascii_lowercase().as_str()))
        {
            out.push(p);
        }
    }
}

/// "Artista - Títol" (el format més habitual) o només el títol.
fn split_name(stem: &str) -> (&str, &str) {
    match stem.split_once(" - ") {
        Some((a, t)) if !a.trim().is_empty() && !t.trim().is_empty() => (a.trim(), t.trim()),
        _ => ("", stem.trim()),
    }
}

/// [{path, title, artist, folder}] ordenats per carpeta i nom.
pub fn tracks(root: &Path) -> Value {
    let mut files = Vec::new();
    walk(root, 0, &mut files);
    files.sort();
    files
        .iter()
        .map(|p| {
            let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
            let (artist, title) = split_name(stem);
            let folder = p
                .parent()
                .filter(|d| *d != root)
                .and_then(|d| d.file_name())
                .and_then(|s| s.to_str())
                .unwrap_or("");
            json!({ "path": p.to_string_lossy(), "title": title, "artist": artist, "folder": folder })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_artist_and_title() {
        assert_eq!(split_name("Oques Grasses - Elefants"), ("Oques Grasses", "Elefants"));
        assert_eq!(split_name("Només títol"), ("", "Només títol"));
        assert_eq!(split_name(" - raro"), ("", "- raro"));
    }
}
