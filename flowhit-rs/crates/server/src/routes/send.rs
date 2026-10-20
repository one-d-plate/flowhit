use axum::{extract::State, Json};
use base64::Engine;
use flowhit_core::{runner, store};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::{info, warn};

use crate::AppState;

#[derive(Deserialize)]
pub struct SendIn {
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub env: String,
    #[serde(default)]
    pub auth: AuthIn,
    #[serde(default)]
    pub form: Vec<FormIn>,
    #[serde(default)]
    pub files: Vec<FileIn>,
}

#[derive(Deserialize, Default)]
pub struct AuthIn {
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub pass: String,
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub val: String,
}

#[derive(Deserialize, Default)]
pub struct FormIn {
    #[serde(default)]
    pub k: String,
    #[serde(default)]
    pub v: String,
}

#[derive(Deserialize, Default)]
pub struct FileIn {
    #[serde(default)]
    pub k: String,
    #[serde(default)]
    pub filename: String,
    #[serde(default)]
    pub b64: String,
}

fn multi_parts(input: &SendIn) -> (Vec<(String, String)>, Vec<(String, String, Vec<u8>)>) {
    let form = input.form.iter().filter(|p| !p.k.trim().is_empty()).map(|p| (p.k.clone(), p.v.clone())).collect();
    let mut files = Vec::new();
    for f in &input.files {
        if f.k.trim().is_empty() {
            continue;
        }
        let name = if f.filename.is_empty() { "file".into() } else { f.filename.clone() };
        match base64::engine::general_purpose::STANDARD.decode(&f.b64) {
            Ok(d) => files.push((f.k.clone(), name, d)),
            Err(e) => warn!(field = %f.k, error = %e, "file base64 rusak, diskip"),
        }
    }
    (form, files)
}

#[derive(Serialize)]
pub struct SendOut {
    result: Option<runner::Outcome>,
    missing: Vec<String>,
    error: Option<String>,
}

pub async fn send(State(st): State<Arc<AppState>>, Json(input): Json<SendIn>) -> Json<SendOut> {
    info!(method = %input.method, url = %input.url, env = %input.env, "POST /api/send");
    let mut vars = HashMap::new();
    for e in store::list_envs(&st.root.join("workspace/envs")) {
        if e.name.eq_ignore_ascii_case(&input.env) {
            vars = e.vars;
            break;
        }
    }
    let mut auth_h = HashMap::new();
    match input.auth.mode.as_str() {
        "bearer" if !input.auth.token.is_empty() => {
            auth_h.insert("Authorization".into(), format!("Bearer {}", input.auth.token));
        }
        "basic" => {
            let raw = format!("{}:{}", input.auth.user, input.auth.pass);
            auth_h.insert(
                "Authorization".into(),
                format!("Basic {}", base64::engine::general_purpose::STANDARD.encode(raw)),
            );
        }
        "apikey" if !input.auth.val.is_empty() => {
            let name = if input.auth.key.trim().is_empty() { "X-API-Key".to_string() } else { input.auth.key.clone() };
            auth_h.insert(name, input.auth.val.clone());
        }
        _ => {}
    }
    let (form, files) = multi_parts(&input);
    match runner::send(&input.method, &input.url, &input.headers, &input.body, &vars, &auth_h, &form, &files).await {
        Ok((res, missing)) => Json(SendOut { result: Some(res), missing, error: None }),
        Err(f) => {
            warn!(error = %f, "request gagal");
            Json(SendOut { result: None, missing: f.missing, error: Some(f.err.to_string()) })
        }
    }
}
