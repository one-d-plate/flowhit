use axum::{extract::State, Json};
use flowhit_core::store;
use serde::Serialize;
use std::{collections::HashMap, sync::Arc};
use tracing::info;

use crate::AppState;

#[derive(Serialize)]
pub struct TreeItem {
    pub file: String,
    pub name: String,
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: HashMap<String, String>,
    pub params: Vec<flowhit_core::model::Param>,
    pub tests: Vec<flowhit_core::model::Check>,
    pub prescript: String,
    pub body_mode: String,
    pub form: Vec<flowhit_core::model::FormField>,
    pub auth_mode: String,
}

#[derive(Serialize)]
pub struct TreeFolder {
    pub name: String,
    pub icon: String,
    pub requests: Vec<TreeItem>,
}

#[derive(Serialize)]
pub struct TreeCol {
    pub name: String,
    pub icon: String,
    pub requests: Vec<TreeItem>,
    pub folders: Vec<TreeFolder>,
}

#[derive(Serialize)]
pub struct TreeFlow {
    pub file: String,
    pub name: String,
    pub nodes: Vec<flowhit_core::model::FlowNode>,
}

#[derive(Serialize)]
pub struct Tree {
    pub cols: Vec<TreeCol>,
    pub envs: Vec<flowhit_core::model::Env>,
    pub flows: Vec<TreeFlow>,
}

fn items(v: Vec<(String, flowhit_core::model::Request)>) -> Vec<TreeItem> {
    v.into_iter()
        .map(|(file, r)| TreeItem { file, name: r.name, method: r.method, url: r.url, body: r.body, headers: r.headers, params: r.params, tests: r.tests, prescript: r.prescript, body_mode: r.body_mode, form: r.form, auth_mode: r.auth.mode })
        .collect()
}

pub async fn tree(State(st): State<Arc<AppState>>) -> Json<Tree> {
    info!("GET /api/tree");
    let dir = st.root.join("workspace/collections");
    let cols = store::list_collections(&dir)
        .into_iter()
        .map(|(name, reqs, folders)| {
            let icon = store::col_icon(&dir, &name);
            TreeCol {
                name,
                icon,
                requests: items(reqs),
                folders: folders.into_iter().map(|(n, ic, r)| TreeFolder { name: n, icon: ic, requests: items(r) }).collect(),
            }
        })
        .collect();
    Json(Tree {
        cols,
        envs: store::list_envs(&st.root.join("workspace/envs")),
        flows: store::list_flows(&st.root.join("workspace/flows"))
            .into_iter()
            .map(|(file, f)| TreeFlow { file, name: f.name, nodes: f.nodes })
            .collect(),
    })
}
