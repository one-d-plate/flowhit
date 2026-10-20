//! FlowHit core: model data workspace (mirip versi Go, tanpa framework).
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Request {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub auth: AuthRef,
    #[serde(default)]
    pub params: Vec<Param>,
    #[serde(default)]
    pub tests: Vec<Check>,
    #[serde(default)]
    pub prescript: String,
    #[serde(default = "body_json")]
    pub body_mode: String,
    #[serde(default)]
    pub form: Vec<FormField>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Param {
    #[serde(default)]
    pub k: String,
    #[serde(default)]
    pub v: String,
    #[serde(default = "default_true")]
    pub on: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Check {
    #[serde(default = "check_status")]
    pub t: String,
    #[serde(default)]
    pub v: String,
    #[serde(default = "default_true")]
    pub on: bool,
}

fn check_status() -> String {
    "status".into()
}

fn body_json() -> String {
    "json".into()
}

fn field_text() -> String {
    "text".into()
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FormField {
    #[serde(default)]
    pub k: String,
    #[serde(default)]
    pub v: String,
    #[serde(default = "field_text")]
    pub t: String,
    #[serde(default = "default_true")]
    pub on: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthRef {
    #[serde(default = "default_inherit")]
    pub mode: String,
}

fn default_inherit() -> String {
    "inherit".into()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Env {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub vars: HashMap<String, String>,
    #[serde(default)]
    pub auth: EnvAuth,
    #[serde(default)]
    pub secrets_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvAuth {
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub token_ref: String,
}

/// Satu langkah dalam Flow (request berantai ala Postman Flows).
/// kind: start | request | decision | setvar | loop | end
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowNode {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
    /// request: path relatif file yaml dari collections/ (pakai /)
    #[serde(default)]
    pub ref_file: String,
    /// request: simpan hasil respons ke variabel flow [{var, src}]
    /// src: `status` | `body` | `header:Name` | `json:$.path.to.field`
    #[serde(default)]
    pub extract: Vec<FlowExtract>,
    /// decision + loop(while): kondisi simpel
    #[serde(default)]
    pub cond: FlowCond,
    /// setvar: variabel flow {nama: nilai} (nilai boleh pakai {{var}})
    #[serde(default)]
    pub vars: HashMap<String, String>,
    /// loop: `count` | `while`
    #[serde(default = "loop_count_mode")]
    pub loop_mode: String,
    #[serde(default = "loop_count_n")]
    pub loop_count: u32,
    /// loop: id node awal badan loop (dilompati balik)
    #[serde(default)]
    pub loop_target: String,
    /// kabel keluar: linier / true / false
    #[serde(default)]
    pub next: String,
    #[serde(default)]
    pub next_true: String,
    #[serde(default)]
    pub next_false: String,
    /// kabel 2-arah: id target yang responsnya ditunggu (call-return)
    #[serde(default)]
    pub await_ids: Vec<String>,
}

fn loop_count_mode() -> String {
    "count".into()
}

fn loop_count_n() -> u32 {
    3
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowExtract {
    #[serde(default)]
    pub var: String,
    #[serde(default)]
    pub src: String,
}

/// Kondisi simpel: left op right.
/// left: `status` | `body` | `json:$.a.b` | `var:NAMA`
/// op: == != > < >= <= contains not_contains
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlowCond {
    #[serde(default = "cond_left")]
    pub left: String,
    #[serde(default = "cond_op")]
    pub op: String,
    #[serde(default)]
    pub right: String,
}

fn cond_left() -> String {
    "status".into()
}

fn cond_op() -> String {
    "==".into()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Flow {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub nodes: Vec<FlowNode>,
}
