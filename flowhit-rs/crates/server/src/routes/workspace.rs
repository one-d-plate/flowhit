use axum::{extract::State, Json};
use flowhit_core::{
    model::{Check, FormField, Param, Request},
    store,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tracing::{info, warn};

use crate::AppState;

#[derive(Deserialize)]
pub struct SaveIn {
    #[serde(default)]
    pub collection: String,
    #[serde(default)]
    pub folder: String,
    #[serde(default)]
    pub group: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub params: Vec<Param>,
    #[serde(default)]
    pub tests: Vec<Check>,
    #[serde(default)]
    pub prescript: String,
    #[serde(default = "default_form_mode")]
    pub body_mode: String,
    #[serde(default)]
    pub form: Vec<FormField>,
    #[serde(default = "default_auth_mode")]
    pub auth_mode: String,
    #[serde(default)]
    pub old_file: String,
}

fn default_auth_mode() -> String {
    "none".into()
}

fn default_form_mode() -> String {
    "json".into()
}

#[derive(Serialize)]
pub struct SaveOut {
    file: Option<String>,
    error: Option<String>,
}

#[derive(Deserialize)]
pub struct EnvSaveIn {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub vars: HashMap<String, String>,
}

#[derive(Serialize)]
pub struct EnvSaveOut {
    ok: bool,
    error: Option<String>,
}

pub async fn env_save(State(st): State<Arc<AppState>>, Json(input): Json<EnvSaveIn>) -> Json<EnvSaveOut> {
    info!(name = %input.name, "POST /api/env-save");
    match store::save_env_vars(&st.root.join("workspace/envs"), &input.name, &input.vars) {
        Ok(()) => Json(EnvSaveOut { ok: true, error: None }),
        Err(e) => {
            warn!(error = %e, "env save gagal");
            Json(EnvSaveOut { ok: false, error: Some(e.to_string()) })
        }
    }
}

#[derive(Deserialize)]
pub struct EnvNameIn {
    #[serde(default)]
    pub name: String,
}

#[derive(Deserialize)]
pub struct EnvRenameIn {
    #[serde(default)]
    pub old: String,
    #[serde(default)]
    pub new: String,
}

pub async fn env_create(State(st): State<Arc<AppState>>, Json(input): Json<EnvNameIn>) -> Json<EnvSaveOut> {
    info!(name = %input.name, "POST /api/env-create");
    match store::create_env(&st.root.join("workspace/envs"), &input.name) {
        Ok(()) => Json(EnvSaveOut { ok: true, error: None }),
        Err(e) => {
            warn!(error = %e, "env create gagal");
            Json(EnvSaveOut { ok: false, error: Some(e.to_string()) })
        }
    }
}

pub async fn env_rename(State(st): State<Arc<AppState>>, Json(input): Json<EnvRenameIn>) -> Json<EnvSaveOut> {
    info!(old = %input.old, new = %input.new, "POST /api/env-rename");
    match store::rename_env(&st.root.join("workspace/envs"), &input.old, &input.new) {
        Ok(()) => Json(EnvSaveOut { ok: true, error: None }),
        Err(e) => {
            warn!(error = %e, "env rename gagal");
            Json(EnvSaveOut { ok: false, error: Some(e.to_string()) })
        }
    }
}

pub async fn env_delete(State(st): State<Arc<AppState>>, Json(input): Json<EnvNameIn>) -> Json<EnvSaveOut> {
    info!(name = %input.name, "POST /api/env-delete");
    match store::delete_env(&st.root.join("workspace/envs"), &input.name) {
        Ok(()) => Json(EnvSaveOut { ok: true, error: None }),
        Err(e) => {
            warn!(error = %e, "env delete gagal");
            Json(EnvSaveOut { ok: false, error: Some(e.to_string()) })
        }
    }
}

pub async fn save(State(st): State<Arc<AppState>>, Json(input): Json<SaveIn>) -> Json<SaveOut> {
    info!(name = %input.name, "POST /api/save");
    let mut headers = input.headers.clone();
    if !headers.keys().any(|k| k.eq_ignore_ascii_case("content-type")) {
        headers.insert("Content-Type".into(), "application/json".into());
    }
    let req = Request {
        name: if input.name.is_empty() { "Untitled".into() } else { input.name },
        method: input.method,
        url: input.url,
        headers,
        body: input.body,
        auth: flowhit_core::model::AuthRef { mode: input.auth_mode },
        params: input.params,
        tests: input.tests,
        prescript: input.prescript,
        body_mode: input.body_mode,
        form: input.form,
    };
    let col = if input.collection.is_empty() { input.group.clone() } else { input.collection.clone() };
    match store::save_request(&st.root.join("workspace/collections"), &col, &input.folder, &req) {
        Ok(f) => {
            if !input.old_file.is_empty() && input.old_file != f {
                let _ = store::delete_request(&st.root.join("workspace/collections"), &input.old_file);
            }
            Json(SaveOut { file: Some(f), error: None })
        }
        Err(e) => {
            warn!(error = %e, "save gagal");
            Json(SaveOut { file: None, error: Some(e.to_string()) })
        }
    }
}

#[derive(Serialize)]
pub struct ExportOut {
    requests: Vec<flowhit_core::model::Request>,
    envs: Vec<flowhit_core::model::Env>,
}

pub async fn export(State(st): State<Arc<AppState>>) -> Json<ExportOut> {
    info!("GET /api/export");
    let dir = st.root.join("workspace/collections");
    let mut requests = Vec::new();
    for (_, reqs, folders) in store::list_collections(&dir) {
        for (_, r) in reqs {
            requests.push(r);
        }
        for (_, _, fr) in folders {
            for (_, r) in fr {
                requests.push(r);
            }
        }
    }
    Json(ExportOut { requests, envs: store::list_envs(&st.root.join("workspace/envs")) })
}

fn pm_esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n")
}

fn pm_export_item(r: &flowhit_core::model::Request) -> serde_json::Value {
    let headers: Vec<serde_json::Value> = r.headers.iter().map(|(k, v)| {
        serde_json::json!({ "key": k, "value": v })
    }).collect();
    let query: Vec<serde_json::Value> = r.params.iter().filter(|p| !p.k.trim().is_empty()).map(|p| {
        let mut o = serde_json::json!({ "key": p.k, "value": p.v });
        if !p.on { o["disabled"] = true.into(); }
        o
    }).collect();
    let body = if r.body_mode == "form" {
        let fields: Vec<serde_json::Value> = r.form.iter().filter(|f| f.t != "file" && !f.k.trim().is_empty()).map(|f| {
            let mut o = serde_json::json!({ "key": f.k, "value": f.v, "type": "text" });
            if !f.on { o["disabled"] = true.into(); }
            o
        }).collect();
        serde_json::json!({ "mode": "formdata", "formdata": fields })
    } else if r.body.trim().is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::json!({ "mode": "raw", "raw": r.body, "options": { "raw": { "language": "json" } } })
    };
    let mut req = serde_json::json!({
        "method": r.method,
        "header": headers,
        "url": { "raw": r.url.split('?').next().unwrap_or(""), "query": query },
    });
    if !body.is_null() { req["body"] = body; }
    let auth = match r.auth.mode.as_str() {
        "bearer" => serde_json::json!({ "type": "bearer", "bearer": [{ "key": "token", "value": "", "type": "string" }] }),
        "basic" => serde_json::json!({ "type": "basic", "basic": [{ "key": "username", "value": "" }, { "key": "password", "value": "" }] }),
        "apikey" => serde_json::json!({ "type": "apikey", "apikey": [] }),
        _ => serde_json::Value::Null,
    };
    if !auth.is_null() { req["auth"] = auth; }
    let mut item = serde_json::json!({ "name": r.name, "request": req });
    let mut events = Vec::new();
    if !r.prescript.trim().is_empty() {
        let mut exec: Vec<String> = vec!["// FlowHit pre-request (translated)".into()];
        for ln in r.prescript.lines() {
            let l = ln.trim();
            if l.is_empty() { continue; }
            if l.starts_with('#') { exec.push(format!("// {}", l)); continue; }
            let low = l.to_lowercase();
            if low.starts_with("header") {
                if let Some((k, v)) = l[6..].split_once(':') {
                    exec.push(format!("pm.request.headers.add({{key: \"{}\", value: \"{}\"}});", pm_esc(k.trim()), pm_esc(v.trim())));
                    continue;
                }
            }
            if low.starts_with("param") {
                if let Some((k, v)) = l[5..].split_once('=') {
                    exec.push(format!("pm.request.url.query.add({{key: \"{}\", value: \"{}\"}});", pm_esc(k.trim()), pm_esc(v.trim())));
                    continue;
                }
            }
            exec.push(format!("// {} (FlowHit only, no Postman equivalent)", l));
        }
        events.push(serde_json::json!({ "listen": "prerequest", "script": { "type": "text/javascript", "exec": exec } }));
    }
    let checks: Vec<String> = r.tests.iter().filter(|t| t.on).map(|t| match t.t.as_str() {
        "time" => format!("pm.test(\"time < {}ms\", function () {{ pm.expect(pm.response.responseTime).to.be.below({}); }});", pm_esc(&t.v), t.v.trim()),
        "body" => format!("pm.test(\"body contains {}\", function () {{ pm.expect(pm.response.text()).to.include(\"{}\"); }});", pm_esc(&t.v), pm_esc(&t.v)),
        _ => match t.v.trim().parse::<u16>() {
            Ok(n) => format!("pm.test(\"status == {}\", function () {{ pm.response.to.have.status({}); }});", n, n),
            Err(_) => format!("pm.test(\"status {}\", function () {{ pm.expect(pm.response.code).to.eql(\"{}\"); }});", pm_esc(&t.v), pm_esc(&t.v)),
        },
    }).collect();
    if !checks.is_empty() {
        events.push(serde_json::json!({ "listen": "test", "script": { "type": "text/javascript", "exec": checks } }));
    }
    if !events.is_empty() { item["event"] = events.into(); }
    item
}

pub async fn export_postman(State(st): State<Arc<AppState>>) -> Json<serde_json::Value> {
    info!("GET /api/export-postman");
    let dir = st.root.join("workspace/collections");
    let mut items = Vec::new();
    for (col, reqs, folders) in store::list_collections(&dir) {
        let mut sub: Vec<serde_json::Value> = reqs.iter().map(|(_, r)| pm_export_item(r)).collect();
        for (fname, _, fr) in &folders {
            sub.push(serde_json::json!({ "name": fname, "item": fr.iter().map(|(_, r)| pm_export_item(r)).collect::<Vec<_>>() }));
        }
        if col == "General" {
            items.extend(sub);
        } else {
            items.push(serde_json::json!({ "name": col, "item": sub }));
        }
    }
    Json(serde_json::json!({
        "info": { "name": "FlowHit Export", "schema": "https://schema.getpostman.com/json/collection/v2.1.0/collection.json" },
        "item": items,
    }))
}

#[derive(Deserialize)]
pub struct DeleteIn {
    #[serde(default)]
    pub file: String,
}

#[derive(Serialize)]
pub struct DeleteOut {
    ok: bool,
    error: Option<String>,
}

pub async fn delete(State(st): State<Arc<AppState>>, Json(input): Json<DeleteIn>) -> Json<DeleteOut> {
    info!(file = %input.file, "POST /api/delete");
    match store::delete_request(&st.root.join("workspace/collections"), &input.file) {
        Ok(()) => Json(DeleteOut { ok: true, error: None }),
        Err(e) => {
            warn!(error = %e, "delete gagal");
            Json(DeleteOut { ok: false, error: Some(e.to_string()) })
        }
    }
}

#[derive(Deserialize)]
pub struct GroupRenameIn {
    #[serde(default)]
    pub old: String,
    #[serde(default)]
    pub new: String,
}

#[derive(Serialize)]
pub struct GroupRenameOut {
    ok: bool,
    error: Option<String>,
}

pub async fn group_rename(State(st): State<Arc<AppState>>, Json(input): Json<GroupRenameIn>) -> Json<GroupRenameOut> {
    info!(old = %input.old, new = %input.new, "POST /api/group-rename");
    match store::rename_group(&st.root.join("workspace/collections"), &input.old, &input.new) {
        Ok(()) => Json(GroupRenameOut { ok: true, error: None }),
        Err(e) => {
            warn!(error = %e, "rename grup gagal");
            Json(GroupRenameOut { ok: false, error: Some(e.to_string()) })
        }
    }
}

#[derive(Deserialize)]
pub struct GroupDeleteIn {
    #[serde(default)]
    pub name: String,
}

#[derive(Serialize)]
pub struct GroupDeleteOut {
    ok: bool,
    count: usize,
    error: Option<String>,
}

pub async fn group_delete(State(st): State<Arc<AppState>>, Json(input): Json<GroupDeleteIn>) -> Json<GroupDeleteOut> {
    info!(name = %input.name, "POST /api/group-delete");
    match store::delete_group(&st.root.join("workspace/collections"), &input.name) {
        Ok(n) => Json(GroupDeleteOut { ok: true, count: n, error: None }),
        Err(e) => {
            warn!(error = %e, "hapus grup gagal");
            Json(GroupDeleteOut { ok: false, count: 0, error: Some(e.to_string()) })
        }
    }
}

#[derive(Deserialize)]
pub struct FolderIn {
    #[serde(default)]
    pub collection: String,
    #[serde(default)]
    pub old: String,
    #[serde(default)]
    pub new: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Serialize)]
pub struct FolderOut {
    ok: bool,
    count: usize,
    error: Option<String>,
}

pub async fn folder_rename(State(st): State<Arc<AppState>>, Json(input): Json<FolderIn>) -> Json<FolderOut> {
    info!(col = %input.collection, old = %input.old, new = %input.new, "POST /api/folder-rename");
    match store::rename_folder(&st.root.join("workspace/collections"), &input.collection, &input.old, &input.new) {
        Ok(()) => Json(FolderOut { ok: true, count: 0, error: None }),
        Err(e) => {
            warn!(error = %e, "rename folder gagal");
            Json(FolderOut { ok: false, count: 0, error: Some(e.to_string()) })
        }
    }
}

pub async fn folder_delete(State(st): State<Arc<AppState>>, Json(input): Json<FolderIn>) -> Json<FolderOut> {
    let target = if input.name.is_empty() { input.old.clone() } else { input.name.clone() };
    info!(col = %input.collection, name = %target, "POST /api/folder-delete");
    match store::delete_folder(&st.root.join("workspace/collections"), &input.collection, &target) {
        Ok(n) => Json(FolderOut { ok: true, count: n, error: None }),
        Err(e) => {
            warn!(error = %e, "hapus folder gagal");
            Json(FolderOut { ok: false, count: 0, error: Some(e.to_string()) })
        }
    }
}

#[derive(Deserialize)]
pub struct IconIn {
    #[serde(default)]
    pub collection: String,
    #[serde(default)]
    pub icon: String,
}

#[derive(Serialize)]
pub struct IconOut {
    ok: bool,
    error: Option<String>,
}

pub async fn set_icon(State(st): State<Arc<AppState>>, Json(input): Json<IconIn>) -> Json<IconOut> {
    info!(col = %input.collection, "POST /api/icon");
    match store::set_icon(&st.root.join("workspace/collections"), &input.collection, &input.icon) {
        Ok(_) => Json(IconOut { ok: true, error: None }),
        Err(e) => {
            warn!(error = %e, "set ikon gagal");
            Json(IconOut { ok: false, error: Some(e.to_string()) })
        }
    }
}

#[derive(Deserialize)]
pub struct FolderIconIn {
    #[serde(default)]
    pub collection: String,
    #[serde(default)]
    pub folder: String,
    #[serde(default)]
    pub icon: String,
}

pub async fn folder_icon(State(st): State<Arc<AppState>>, Json(input): Json<FolderIconIn>) -> Json<IconOut> {
    info!(col = %input.collection, folder = %input.folder, "POST /api/folder-icon");
    match store::set_folder_icon(&st.root.join("workspace/collections"), &input.collection, &input.folder, &input.icon) {
        Ok(_) => Json(IconOut { ok: true, error: None }),
        Err(e) => {
            warn!(error = %e, "set ikon folder gagal");
            Json(IconOut { ok: false, error: Some(e.to_string()) })
        }
    }
}

// Import koleksi Postman v2.1 (item: [{name, request:{method,url,body,header}}]).
#[derive(Deserialize)]
pub struct ImportIn {
    pub postman: serde_json::Value,
}

#[derive(Serialize)]
pub struct ImportOut {
    count: usize,
    error: Option<String>,
}

pub async fn import(State(st): State<Arc<AppState>>, Json(input): Json<ImportIn>) -> Json<ImportOut> {
    info!("POST /api/import");
    let items = input.postman.get("item").and_then(|v| v.as_array()).cloned().unwrap_or_default();
    let mut count = 0;
    let dir = st.root.join("workspace/collections");
    for it in &items {
        import_item(&dir, "Imported", "", it, &mut count);
    }
    Json(ImportOut { count, error: if count == 0 { Some("tidak ada request terbaca (butuh Postman v2.1)".into()) } else { None } })
}

fn import_item(dir: &std::path::Path, collection: &str, folder: &str, it: &serde_json::Value, count: &mut usize) {
    if let Some(sub) = it.get("item").and_then(|v| v.as_array()) {
        let name = it.get("name").and_then(|v| v.as_str()).unwrap_or("Folder");
        let sub_folder = if folder.is_empty() { name.into() } else { format!("{folder} / {name}") };
        for s in sub {
            import_item(dir, collection, &sub_folder, s, count);
        }
        return;
    }
    let req = pm_item(it);
    if req.name.is_empty() {
        return;
    }
    match store::save_request(dir, collection, folder, &req) {
        Ok(_) => *count += 1,
        Err(e) => warn!(error = %e, "import item gagal"),
    }
}

fn pm_item(it: &serde_json::Value) -> Request {
    let name = it.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let r = it.get("request");
    let method = r.and_then(|x| x.get("method")).and_then(|v| v.as_str()).unwrap_or("GET").to_string();
    let mut url = r
        .and_then(|x| x.get("url"))
        .map(|u| {
            if let Some(s) = u.as_str() {
                s.to_string()
            } else {
                u.get("raw").and_then(|v| v.as_str()).unwrap_or("").to_string()
            }
        })
        .unwrap_or_default();
    if let Some(ix) = url.find('?') {
        url.truncate(ix);
    }
    let mut headers = HashMap::new();
    if let Some(hs) = r.and_then(|x| x.get("header")).and_then(|v| v.as_array()) {
        for h in hs {
            let k = h.get("key").and_then(|v| v.as_str()).unwrap_or("");
            let v = h.get("value").and_then(|v| v.as_str()).unwrap_or("");
            if !k.is_empty() {
                headers.insert(k.into(), v.into());
            }
        }
    }
    if !headers.contains_key("Content-Type") {
        headers.insert("Content-Type".into(), "application/json".into());
    }
    let body_obj = r.and_then(|x| x.get("body"));
    let body = body_obj
        .and_then(|b| b.get("raw"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let (body_mode, form) = match body_obj.and_then(|b| b.get("mode")).and_then(|v| v.as_str()) {
        Some("formdata") => {
            let mut form = Vec::new();
            if let Some(fs) = body_obj.and_then(|b| b.get("formdata")).and_then(|v| v.as_array()) {
                for f in fs {
                    let k = f.get("key").and_then(|v| v.as_str()).unwrap_or("");
                    if k.is_empty() || f.get("type").and_then(|v| v.as_str()).unwrap_or("text") == "file" {
                        continue;
                    }
                    let v = f.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let on = f.get("disabled").and_then(|v| v.as_bool()).map(|d| !d).unwrap_or(true);
                    form.push(flowhit_core::model::FormField { k: k.into(), v, t: "text".into(), on });
                }
            }
            ("form".to_string(), form)
        }
        _ => ("json".to_string(), Vec::new()),
    };
    let mut params = Vec::new();
    if let Some(qs) = r
        .and_then(|x| x.get("url"))
        .and_then(|u| u.get("query"))
        .and_then(|v| v.as_array())
    {
        for q in qs {
            let k = q.get("key").and_then(|v| v.as_str()).unwrap_or("");
            if k.is_empty() {
                continue;
            }
            let v = q.get("value").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let on = q.get("disabled").and_then(|v| v.as_bool()).map(|d| !d).unwrap_or(true);
            params.push(Param { k: k.into(), v, on });
        }
    }
    let auth_type = r.and_then(|x| x.get("auth")).and_then(|a| a.get("type")).and_then(|v| v.as_str()).unwrap_or("noauth");
    let auth_mode = if ["bearer", "basic", "apikey"].contains(&auth_type) { auth_type.to_string() } else { "none".to_string() };
    Request { name, method, url, headers, body, auth: flowhit_core::model::AuthRef { mode: auth_mode }, params, tests: Vec::new(), prescript: String::new(), body_mode, form }
}

#[derive(Deserialize)]
pub struct FlowSaveIn {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nodes: Vec<flowhit_core::model::FlowNode>,
    #[serde(default)]
    pub old_file: String,
}

pub async fn flow_save(State(st): State<Arc<AppState>>, Json(input): Json<FlowSaveIn>) -> Json<SaveOut> {
    info!(name = %input.name, "POST /api/flow-save");
    let flow = flowhit_core::model::Flow { name: if input.name.is_empty() { "Untitled flow".into() } else { input.name }, nodes: input.nodes };
    match store::save_flow(&st.root.join("workspace/flows"), &flow) {
        Ok(f) => {
            if !input.old_file.is_empty() && input.old_file != f {
                let _ = store::delete_flow(&st.root.join("workspace/flows"), &input.old_file);
            }
            Json(SaveOut { file: Some(f), error: None })
        }
        Err(e) => {
            warn!(error = %e, "flow save gagal");
            Json(SaveOut { file: None, error: Some(e.to_string()) })
        }
    }
}

#[derive(Deserialize)]
pub struct FlowDelIn {
    #[serde(default)]
    pub file: String,
}

pub async fn flow_delete(State(st): State<Arc<AppState>>, Json(input): Json<FlowDelIn>) -> Json<EnvSaveOut> {
    info!(file = %input.file, "POST /api/flow-delete");
    match store::delete_flow(&st.root.join("workspace/flows"), &input.file) {
        Ok(()) => Json(EnvSaveOut { ok: true, error: None }),
        Err(e) => {
            warn!(error = %e, "flow delete gagal");
            Json(EnvSaveOut { ok: false, error: Some(e.to_string()) })
        }
    }
}
