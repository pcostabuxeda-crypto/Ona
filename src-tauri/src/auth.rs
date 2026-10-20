//! OAuth de Spotify (codi d'autorització + PKCE) amb tornada a 127.0.0.1:5588.
//! Substitueix `librespot-oauth`, que es bloqueja per sempre esperant el navegador
//! i deixa el port ocupat si l'usuari no acaba.

use std::time::Duration;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::RngCore;
use sha2::{Digest, Sha256};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub const REDIRECT: &str = "http://127.0.0.1:5588/login";
const LISTEN_ADDR: &str = "127.0.0.1:5588";
const TIMEOUT: Duration = Duration::from_secs(300);

const DONE_HTML: &str = r#"<!doctype html><meta charset="utf-8"><title>Ona</title>
<body style="margin:0;height:100vh;display:grid;place-items:center;background:#0b0b12;color:#f2f0ff;font-family:system-ui">
<div style="text-align:center"><h1 style="font-weight:600">Fet!</h1><p style="opacity:.7">Ja pots tancar aquesta pestanya i tornar a Ona.</p></div>"#;

pub struct Tokens {
    pub access: String,
}

fn random_b64(len: usize) -> String {
    let mut bytes = vec![0u8; len];
    rand::thread_rng().fill_bytes(&mut bytes);
    URL_SAFE_NO_PAD.encode(bytes)
}

/// Obre el navegador i espera el codi. `open` rep l'URL d'autorització.
pub async fn authorize(
    http: &reqwest::Client,
    client_id: &str,
    scopes: &[&str],
    open: impl FnOnce(&str) -> Result<(), String>,
) -> Result<Tokens, String> {
    let verifier = random_b64(48);
    let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
    let state = random_b64(16);

    let url = reqwest::Url::parse_with_params(
        "https://accounts.spotify.com/authorize",
        &[
            ("client_id", client_id),
            ("response_type", "code"),
            ("redirect_uri", REDIRECT),
            ("code_challenge_method", "S256"),
            ("code_challenge", &challenge),
            ("scope", &scopes.join(" ")),
            ("state", &state),
        ],
    )
    .map_err(|e| e.to_string())?;

    // Primer s'obre el port, després el navegador: així la tornada sempre troba algú escoltant.
    let listener = TcpListener::bind(LISTEN_ADDR)
        .await
        .map_err(|e| format!("No s'ha pogut obrir el port 5588: {e}"))?;
    open(url.as_str())?;

    let code = tokio::time::timeout(TIMEOUT, wait_for_code(&listener, &state))
        .await
        .map_err(|_| "S'ha esgotat el temps d'espera del navegador".to_string())??;
    drop(listener);

    token_request(
        http,
        &[
            ("grant_type", "authorization_code"),
            ("code", &code),
            ("redirect_uri", REDIRECT),
            ("client_id", client_id),
            ("code_verifier", &verifier),
        ],
    )
    .await
}

async fn token_request(http: &reqwest::Client, form: &[(&str, &str)]) -> Result<Tokens, String> {
    let resp = http
        .post("https://accounts.spotify.com/api/token")
        .form(form)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        return Err(format!(
            "Spotify ha rebutjat l'accés: {}",
            body.get("error_description")
                .or(body.get("error"))
                .and_then(|v| v.as_str())
                .unwrap_or("error desconegut")
        ));
    }
    Ok(Tokens {
        access: body.get("access_token").and_then(|v| v.as_str()).unwrap_or("").to_string(),
    })
}

/// Atén peticions fins que n'arriba una a `/login` amb el codi.
async fn wait_for_code(listener: &TcpListener, state: &str) -> Result<String, String> {
    loop {
        let (mut sock, _) = listener.accept().await.map_err(|e| e.to_string())?;
        let mut buf = vec![0u8; 8192];
        let n = sock.read(&mut buf).await.unwrap_or(0);
        let req = String::from_utf8_lossy(&buf[..n]);
        let target = req.split_whitespace().nth(1).unwrap_or("");

        let Ok(url) = reqwest::Url::parse(&format!("http://127.0.0.1{target}")) else {
            continue;
        };
        if url.path() != "/login" {
            let _ = sock
                .write_all(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n")
                .await;
            continue;
        }
        let param = |k: &str| url.query_pairs().find(|(key, _)| key == k).map(|(_, v)| v.into_owned());

        let body = DONE_HTML.as_bytes();
        let head = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        );
        let _ = sock.write_all(head.as_bytes()).await;
        let _ = sock.write_all(body).await;

        if let Some(e) = param("error") {
            return Err(format!("Has denegat l'accés ({e})"));
        }
        if param("state").as_deref() != Some(state) {
            continue;
        }
        if let Some(code) = param("code") {
            return Ok(code);
        }
    }
}
