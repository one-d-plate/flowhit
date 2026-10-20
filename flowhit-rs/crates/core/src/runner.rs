//! Eksekusi 1 HTTP request (setara runner Go).
use serde::Serialize;
use std::collections::HashMap;
use std::time::Instant;
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Error)]
pub enum RunError {
    #[error("URL rusak: {0}")]
    UrlRusak(String),
    #[error("gagal kirim: {0}")]
    Kirim(String),
}

/// Gagal + daftar variable yang hilang (biar UI bisa nunjukin).
#[derive(Debug, Error)]
#[error("{err}")]
pub struct Fail {
    pub err: RunError,
    pub missing: Vec<String>,
}

impl From<reqwest::Error> for Fail {
    fn from(e: reqwest::Error) -> Self {
        Fail { err: RunError::Kirim(e.to_string()), missing: vec![] }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Outcome {
    pub status: u16,
    pub status_text: String,
    pub ms: u128,
    pub size: usize,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub body_large: bool,
}

fn form_active(form: &[(String, String)], files: &[(String, String, Vec<u8>)]) -> bool {
    !form.is_empty() || !files.is_empty()
}

fn build_multipart(
    form: &[(String, String)],
    files: &[(String, String, Vec<u8>)],
    vars: &HashMap<String, String>,
    missing: &mut Vec<String>,
) -> reqwest::multipart::Form {
    let mut f = reqwest::multipart::Form::new();
    for (k, v) in form {
        let (rv, m) = crate::vars::resolve(v, vars);
        missing.extend(m);
        f = f.text(k.clone(), rv);
    }
    for (k, name, data) in files {
        let part = reqwest::multipart::Part::bytes(data.clone()).file_name(name.clone());
        f = f.part(k.clone(), part);
    }
    f
}

pub async fn send(
    method: &str,
    raw_url: &str,
    headers: &HashMap<String, String>,
    body: &str,
    vars: &HashMap<String, String>,
    auth_headers: &HashMap<String, String>,
    form: &[(String, String)],
    files: &[(String, String, Vec<u8>)],
) -> Result<(Outcome, Vec<String>), Fail> {
    let (url, mut missing) = crate::vars::resolve(raw_url, vars);
    let mut hdrs = HashMap::new();
    for (k, v) in headers {
        let (rv, m) = crate::vars::resolve(v, vars);
        missing.extend(m);
        hdrs.insert(k.clone(), rv);
    }
    for (k, v) in auth_headers {
        hdrs.insert(k.clone(), v.clone());
    }
    let (rbody, m) = crate::vars::resolve(body, vars);
    missing.extend(m);

    let method_parsed = method
        .parse()
        .map_err(|_| Fail { err: RunError::UrlRusak(url.clone()), missing: missing.clone() })?;
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let mut rb = client.request(method_parsed, &url);
    for (k, v) in &hdrs {
        if form_active(form, files) && k.eq_ignore_ascii_case("content-type") {
            continue;
        }
        rb = rb.header(k, v);
    }
    if form_active(form, files) {
        rb = rb.multipart(build_multipart(form, files, vars, &mut missing));
    } else if !rbody.is_empty() {
        rb = rb.body(rbody);
    }
    info!(%method, %url, "kirim request");
    let t0 = Instant::now();
    let resp = rb.send().await.map_err(|e| {
        warn!(error = %e, "gagal kirim");
        Fail { err: RunError::Kirim(e.to_string()), missing: missing.clone() }
    })?;
    let status = resp.status();
    let mut rh = HashMap::new();
    for (k, v) in resp.headers() {
        rh.insert(k.to_string(), v.to_str().unwrap_or("").to_string());
    }
    let bytes = resp.bytes().await.unwrap_or_default();
    let ms = t0.elapsed().as_millis();
    info!(status = %status, ms, size = bytes.len(), "respons masuk");
    let (body_str, large) = if bytes.len() >= 2 * 1024 * 1024 {
        (String::from_utf8_lossy(&bytes[..5000]).into_owned(), true)
    } else {
        (String::from_utf8_lossy(&bytes).into_owned(), false)
    };
    Ok((
        Outcome {
            status: status.as_u16(),
            status_text: status.to_string(),
            ms,
            size: bytes.len(),
            headers: rh,
            body: body_str,
            body_large: large,
        },
        missing,
    ))
}
