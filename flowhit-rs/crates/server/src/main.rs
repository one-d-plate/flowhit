mod routes;

use axum::{routing::{get, post}, Router};
use std::{path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;
use tracing::info;

pub struct AppState {
    pub root: PathBuf,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("flowhit=debug,tower_http=info")
        .with_target(false)
        .without_time()
        .compact()
        .init();

    let exe_dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()));
    let root: PathBuf = std::env::var("FLOWHIT_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().expect("cwd terbaca"));
    let web_dir = [&root, exe_dir.as_deref().unwrap_or(&root)]
        .iter()
        .map(|b| b.join("web"))
        .chain(exe_dir.iter().flat_map(|d| d.parent()).map(|p| p.join("Resources/web")))
        .find(|p| p.is_dir())
        .expect("folder web tidak ketemu");
    let st = Arc::new(AppState { root: root.clone() });
    let app = Router::new()
        .route("/api/tree", get(routes::tree::tree))
        .route("/api/send", post(routes::send::send))
        .route("/api/save", post(routes::workspace::save))
        .route("/api/export", get(routes::workspace::export))
        .route("/api/import", post(routes::workspace::import))
        .route("/api/delete", post(routes::workspace::delete))
        .route("/api/group-rename", post(routes::workspace::group_rename))
        .route("/api/group-delete", post(routes::workspace::group_delete))
        .route("/api/folder-rename", post(routes::workspace::folder_rename))
        .route("/api/folder-delete", post(routes::workspace::folder_delete))
        .route("/api/icon", post(routes::workspace::set_icon))
        .route("/api/folder-icon", post(routes::workspace::folder_icon))
        .route("/api/env-save", post(routes::workspace::env_save))
        .route("/api/env-create", post(routes::workspace::env_create))
        .route("/api/env-rename", post(routes::workspace::env_rename))
        .route("/api/env-delete", post(routes::workspace::env_delete))
        .route("/api/export-postman", get(routes::workspace::export_postman))
        .route("/api/flow-save", post(routes::workspace::flow_save))
        .route("/api/flow-delete", post(routes::workspace::flow_delete))
        .fallback_service(ServeDir::new(web_dir))
        .with_state(st);

    info!("FlowHit-RS :4174");
    let li = tokio::net::TcpListener::bind("127.0.0.1:4174").await.expect("bind :4174");
    axum::serve(li, app).await.expect("server jalan");
}
