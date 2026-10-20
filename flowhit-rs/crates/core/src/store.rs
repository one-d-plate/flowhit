//! Baca workspace YAML (collections + envs).
use crate::model::{Env, Request};
use std::path::Path;
use thiserror::Error;
use tracing::{debug, warn};

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("gagal baca {0}: {1}")]
    Baca(String, String),
}

fn load_yaml<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, StoreError> {
    let b = std::fs::read(path).map_err(|e| StoreError::Baca(path.display().to_string(), e.to_string()))?;
    serde_yaml::from_slice(&b).map_err(|e| StoreError::Baca(path.display().to_string(), e.to_string()))
}

pub fn list_requests(dir: &Path) -> Vec<Request> {
    let mut out = Vec::new();
    let rd = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return out,
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.extension().map(|x| x == "yaml" || x == "yml").unwrap_or(false) {
            match load_yaml::<Request>(&p) {
                Ok(mut r) => {
                    if r.name.is_empty() {
                        r.name = p.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                    }
                    out.push(r);
                }
                Err(e) => warn!(file = %p.display(), error = %e, "request diskip"),
            }
        }
    }
    debug!(jumlah = out.len(), "requests dimuat");
    out
}

/// Hierarki: Collection (folder / General) → Folder (subfolder) → Request (file).
/// Mengembalikan (nama_collection, [(file, request)], [(nama_folder, ikon, [(file, request)])]).
/// `file` selalu path relatif dari collections/ (pakai /).
pub fn list_collections(dir: &Path) -> Vec<(String, Vec<(String, Request)>, Vec<(String, String, Vec<(String, Request)>)>)> {
    let mut out = Vec::new();
    let rd = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return out,
    };
    let mut subdirs = Vec::new();
    let mut root_files = Vec::new();
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            let n = p.file_name().unwrap_or_default().to_string_lossy().into_owned();
            if !n.starts_with('.') {
                subdirs.push((n, p));
            }
        } else if is_yaml(&p) {
            root_files.push(p);
        }
    }
    if !root_files.is_empty() {
        root_files.sort();
        out.push(("General".into(), rel_many(dir, &root_files), Vec::new()));
    }
    subdirs.sort_by(|a, b| a.0.cmp(&b.0));
    for (name, d) in subdirs {
        let mut loose = Vec::new();
        let mut folders: Vec<(String, String, Vec<(String, Request)>)> = Vec::new();
        if let Ok(rd) = std::fs::read_dir(&d) {
            let mut fsub = Vec::new();
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    let fn_ = p.file_name().unwrap_or_default().to_string_lossy().into_owned();
                    if !fn_.starts_with('.') {
                        fsub.push((fn_, p));
                    }
                } else if is_yaml(&p) {
                    loose.push(p);
                }
            }
            loose.sort();
            fsub.sort_by(|a, b| a.0.cmp(&b.0));
            for (fn_, fp) in fsub {
                let mut ff = Vec::new();
                if let Ok(rd) = std::fs::read_dir(&fp) {
                    for e in rd.flatten() {
                        let p = e.path();
                        if is_yaml(&p) {
                            ff.push(p);
                        }
                    }
                }
                ff.sort();
                folders.push((fn_.clone(), folder_icon(dir, &name, &fn_), rel_many(dir, &ff)));
            }
        }
        out.push((name, rel_many(dir, &loose), folders));
    }
    out
}

fn rel_many(dir: &Path, files: &[std::path::PathBuf]) -> Vec<(String, Request)> {
    let mut out = Vec::new();
    for p in files {
        match load_yaml::<Request>(p) {
            Ok(mut r) => {
                if r.name.is_empty() {
                    r.name = p.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                }
                let rel = p.strip_prefix(dir).unwrap_or(p).display().to_string().replace('\\', "/");
                out.push((rel, r));
            }
            Err(e) => warn!(file = %p.display(), error = %e, "request diskip"),
        }
    }
    out
}
fn is_yaml(p: &Path) -> bool {
    p.extension().map(|x| x == "yaml" || x == "yml").unwrap_or(false)
}

fn read_many(files: &[std::path::PathBuf]) -> Vec<(String, Request)> {
    let mut out = Vec::new();
    for p in files {
        match load_yaml::<Request>(p) {
            Ok(mut r) => {
                if r.name.is_empty() {
                    r.name = p.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                }
                out.push((p.file_name().unwrap_or_default().to_string_lossy().into_owned(), r));
            }
            Err(e) => warn!(file = %p.display(), error = %e, "request diskip"),
        }
    }
    out
}

pub fn list_envs(dir: &Path) -> Vec<Env> {
    let mut out = Vec::new();
    let rd = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return out,
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.extension().map(|x| x == "yaml" || x == "yml").unwrap_or(false) {
            match load_yaml::<Env>(&p) {
                Ok(mut v) => {
                    if v.name.is_empty() {
                        v.name = p.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                    }
                    out.push(v);
                }
                Err(e) => warn!(file = %p.display(), error = %e, "env diskip"),
            }
        }
    }
    debug!(jumlah = out.len(), "envs dimuat");
    out
}

/// Tulis ulang vars satu env (dikenali dari field name, case-insensitive).
/// Field lain (auth, secrets_id) dipertahankan.
pub fn save_env_vars(dir: &Path, name: &str, vars: &std::collections::HashMap<String, String>) -> Result<(), StoreError> {
    let rd = std::fs::read_dir(dir).map_err(|e| StoreError::Baca(dir.display().to_string(), e.to_string()))?;
    for e in rd.flatten() {
        let p = e.path();
        if !is_yaml(&p) {
            continue;
        }
        if let Ok(mut v) = load_yaml::<Env>(&p) {
            if v.name.eq_ignore_ascii_case(name) {
                v.vars = vars.clone();
                let text = serde_yaml::to_string(&v).map_err(|e| StoreError::Baca(p.display().to_string(), e.to_string()))?;
                std::fs::write(&p, text).map_err(|e| StoreError::Baca(p.display().to_string(), e.to_string()))?;
                return Ok(());
            }
        }
    }
    Err(StoreError::Baca(name.into(), "env tidak ketemu".into()))
}

/// Buat env baru: fail bila nama sudah ada (case-insensitive) atau file slug bentrok.
pub fn create_env(dir: &Path, name: &str) -> Result<(), StoreError> {
    let name = name.trim();
    if name.is_empty() {
        return Err(StoreError::Baca(name.into(), "nama env kosong".into()));
    }
    let rd = std::fs::read_dir(dir).map_err(|e| StoreError::Baca(dir.display().to_string(), e.to_string()))?;
    for e in rd.flatten() {
        let p = e.path();
        if !is_yaml(&p) {
            continue;
        }
        if let Ok(v) = load_yaml::<Env>(&p) {
            if v.name.eq_ignore_ascii_case(name) {
                return Err(StoreError::Baca(name.into(), "env sudah ada".into()));
            }
        }
    }
    let target = dir.join(slugify(name) + ".yaml");
    if target.exists() {
        return Err(StoreError::Baca(name.into(), "file env sudah ada".into()));
    }
    let env = Env { name: name.into(), vars: Default::default(), auth: Default::default(), secrets_id: "default".into() };
    let text = serde_yaml::to_string(&env).map_err(|e| StoreError::Baca(target.display().to_string(), e.to_string()))?;
    std::fs::write(&target, text).map_err(|e| StoreError::Baca(target.display().to_string(), e.to_string()))?;
    Ok(())
}

/// Ganti nama env (field name; file ikut di-rename bila namanya slug lama).
pub fn rename_env(dir: &Path, old: &str, new: &str) -> Result<(), StoreError> {
    let new = new.trim();
    if new.is_empty() {
        return Err(StoreError::Baca(new.into(), "nama env kosong".into()));
    }
    if old.eq_ignore_ascii_case(new) {
        return Ok(());
    }
    let rd = std::fs::read_dir(dir).map_err(|e| StoreError::Baca(dir.display().to_string(), e.to_string()))?;
    let mut found: Option<std::path::PathBuf> = None;
    for e in rd.flatten() {
        let p = e.path();
        if !is_yaml(&p) {
            continue;
        }
        if let Ok(v) = load_yaml::<Env>(&p) {
            if v.name.eq_ignore_ascii_case(new) {
                return Err(StoreError::Baca(new.into(), "nama env sudah dipakai".into()));
            }
            if v.name.eq_ignore_ascii_case(old) {
                found = Some(p);
            }
        }
    }
    let p = found.ok_or_else(|| StoreError::Baca(old.into(), "env tidak ketemu".into()))?;
    let mut v = load_yaml::<Env>(&p).map_err(|e| StoreError::Baca(p.display().to_string(), e.to_string()))?;
    v.name = new.into();
    let text = serde_yaml::to_string(&v).map_err(|e| StoreError::Baca(p.display().to_string(), e.to_string()))?;
    let old_slug = slugify(old);
    if p.file_stem().map(|s| s.to_string_lossy() == old_slug).unwrap_or(false) {
        let target = dir.join(slugify(new) + ".yaml");
        if target.exists() {
            return Err(StoreError::Baca(new.into(), "file env sudah ada".into()));
        }
        std::fs::write(&target, &text).map_err(|e| StoreError::Baca(target.display().to_string(), e.to_string()))?;
        std::fs::remove_file(&p).map_err(|e| StoreError::Baca(p.display().to_string(), e.to_string()))?;
    } else {
        std::fs::write(&p, text).map_err(|e| StoreError::Baca(p.display().to_string(), e.to_string()))?;
    }
    Ok(())
}

/// Hapus env (file yaml-nya). Menolak bila tinggal 1 file.
pub fn delete_env(dir: &Path, name: &str) -> Result<(), StoreError> {
    let rd = std::fs::read_dir(dir).map_err(|e| StoreError::Baca(dir.display().to_string(), e.to_string()))?;
    let mut yamls = Vec::new();
    let mut found: Option<std::path::PathBuf> = None;
    for e in rd.flatten() {
        let p = e.path();
        if !is_yaml(&p) {
            continue;
        }
        yamls.push(p.clone());
        if let Ok(v) = load_yaml::<Env>(&p) {
            if v.name.eq_ignore_ascii_case(name) {
                found = Some(p);
            }
        }
    }
    let p = found.ok_or_else(|| StoreError::Baca(name.into(), "env tidak ketemu".into()))?;
    if yamls.len() <= 1 {
        return Err(StoreError::Baca(name.into(), "env terakhir tidak boleh dihapus".into()));
    }
    std::fs::remove_file(&p).map_err(|e| StoreError::Baca(p.display().to_string(), e.to_string()))?;
    Ok(())
}

/// Tulis 1 request ke collections/[collection]/[folder]/{slug}.yaml.
/// General tanpa folder = langsung di root. Mengembalikan path relatif (pakai /).
pub fn save_request(dir: &Path, collection: &str, folder: &str, req: &Request) -> Result<String, StoreError> {
    let mut target = dir.to_path_buf();
    if !collection.is_empty() && collection != "General" {
        target = target.join(slugify(collection));
    }
    if !folder.is_empty() {
        target = target.join(slugify(folder));
    }
    std::fs::create_dir_all(&target).map_err(|e| StoreError::Baca(target.display().to_string(), e.to_string()))?;
    let slug = slugify(&req.name);
    let path = target.join(format!("{}.yaml", slug));
    let text = serde_yaml::to_string(req).map_err(|e| StoreError::Baca(path.display().to_string(), e.to_string()))?;
    std::fs::write(&path, text).map_err(|e| StoreError::Baca(path.display().to_string(), e.to_string()))?;
    Ok(path.strip_prefix(dir).unwrap_or(&path).display().to_string().replace('\\', "/"))
}

fn slugify(name: &str) -> String {
    let s = name.to_lowercase().chars().map(|c| if c.is_alphanumeric() { c } else { '-' }).collect::<String>();
    let s = s.split('-').filter(|x| !x.is_empty()).collect::<Vec<_>>().join("-");
    if s.is_empty() { "request".into() } else { s }
}

/// Ikon collection tersimpan di {collection}/.meta.json {"icon":"..."}.
/// General (virtual, file lepas di root) memakai collections/.meta.json.
pub fn col_icon(dir: &Path, collection: &str) -> String {
    let p = if collection == "General" {
        dir.join(".meta.json")
    } else {
        dir.join(slugify(collection)).join(".meta.json")
    };
    std::fs::read_to_string(p)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v.get("icon").and_then(|x| x.as_str()).map(|s| s.to_string()))
        .unwrap_or_default()
}

pub fn set_icon(dir: &Path, collection: &str, icon: &str) -> Result<(), StoreError> {
    if dir_bad(collection, true) {
        return Err(StoreError::Baca(collection.into(), "collection ditolak".into()));
    }
    let d = if collection == "General" {
        dir.to_path_buf()
    } else {
        let d = dir.join(slugify(collection));
        if !d.is_dir() {
            return Err(StoreError::Baca(collection.into(), "collection tidak ada".into()));
        }
        d
    };
    let clean: String = icon.chars().take(8).collect();
    let text = format!("{{\"icon\":{}}}", serde_json::to_string(&clean).unwrap_or("\"\"".into()));
    std::fs::write(d.join(".meta.json"), text).map_err(|e| StoreError::Baca(collection.into(), e.to_string()))
}

/// Ikon folder tersimpan di {collection}/{folder}/.meta.json.
pub fn folder_icon(dir: &Path, collection: &str, folder: &str) -> String {
    let p = dir.join(slugify(collection)).join(slugify(folder)).join(".meta.json");
    std::fs::read_to_string(p)
        .ok()
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| v.get("icon").and_then(|x| x.as_str()).map(|s| s.to_string()))
        .unwrap_or_default()
}

pub fn set_folder_icon(dir: &Path, collection: &str, folder: &str, icon: &str) -> Result<(), StoreError> {
    if dir_bad(collection, false) || folder.is_empty() || folder.contains("..") || folder.contains('/') || folder.contains('\\') {
        return Err(StoreError::Baca(folder.into(), "folder ditolak".into()));
    }
    let d = dir.join(slugify(collection)).join(slugify(folder));
    if !d.is_dir() {
        return Err(StoreError::Baca(folder.into(), "folder tidak ada".into()));
    }
    let clean: String = icon.chars().take(8).collect();
    let text = format!("{{\"icon\":{}}}", serde_json::to_string(&clean).unwrap_or("\"\"".into()));
    std::fs::write(d.join(".meta.json"), text).map_err(|e| StoreError::Baca(folder.into(), e.to_string()))
}

fn dir_bad(s: &str, allow_general: bool) -> bool {
    s.is_empty() || (!allow_general && s == "General") || s.contains("..") || s.contains('/') || s.contains('\\')
}

/// Rename collection (folder level-1).
pub fn rename_group(dir: &Path, old: &str, new: &str) -> Result<(), StoreError> {
    if dir_bad(old, false) || dir_bad(new, false) {
        return Err(StoreError::Baca(new.into(), "nama collection ditolak".into()));
    }
    let from = dir.join(slugify(old));
    let to = dir.join(slugify(new));
    if !from.is_dir() {
        return Err(StoreError::Baca(old.into(), "collection tidak ada".into()));
    }
    if to.exists() {
        return Err(StoreError::Baca(new.into(), "nama sudah dipakai".into()));
    }
    std::fs::rename(&from, &to).map_err(|e| StoreError::Baca(new.into(), e.to_string()))
}

/// Rename folder di dalam collection.
pub fn rename_folder(dir: &Path, collection: &str, old: &str, new: &str) -> Result<(), StoreError> {
    if dir_bad(old, true) || dir_bad(new, true) {
        return Err(StoreError::Baca(new.into(), "nama folder ditolak".into()));
    }
    let base = if collection == "General" { dir.to_path_buf() } else { dir.join(slugify(collection)) };
    let from = base.join(slugify(old));
    let to = base.join(slugify(new));
    if !from.is_dir() {
        return Err(StoreError::Baca(old.into(), "folder tidak ada".into()));
    }
    if to.exists() {
        return Err(StoreError::Baca(new.into(), "nama sudah dipakai".into()));
    }
    std::fs::rename(&from, &to).map_err(|e| StoreError::Baca(new.into(), e.to_string()))
}
/// Hapus collection (folder + isinya). General tidak bisa dihapus.
pub fn delete_group(dir: &Path, name: &str) -> Result<usize, StoreError> {
    if dir_bad(name, false) {
        return Err(StoreError::Baca(name.into(), "collection ditolak".into()));
    }
    let target = dir.join(slugify(name));
    if !target.is_dir() {
        return Err(StoreError::Baca(name.into(), "collection tidak ada".into()));
    }
    let n = std::fs::read_dir(&target).map(|r| r.count()).unwrap_or(0);
    std::fs::remove_dir_all(&target).map_err(|e| StoreError::Baca(name.into(), e.to_string()))?;
    Ok(n)
}

/// Hapus folder di dalam collection (+ isinya).
pub fn delete_folder(dir: &Path, collection: &str, name: &str) -> Result<usize, StoreError> {
    if dir_bad(name, true) {
        return Err(StoreError::Baca(name.into(), "folder ditolak".into()));
    }
    let base = if collection == "General" { dir.to_path_buf() } else { dir.join(slugify(collection)) };
    let target = base.join(slugify(name));
    if !target.is_dir() {
        return Err(StoreError::Baca(name.into(), "folder tidak ada".into()));
    }
    let n = std::fs::read_dir(&target).map(|r| r.count()).unwrap_or(0);
    std::fs::remove_dir_all(&target).map_err(|e| StoreError::Baca(name.into(), e.to_string()))?;
    Ok(n)
}

/// Hapus 1 file request (path relatif dari collections). Tolak path keluar dir.
pub fn delete_request(dir: &Path, rel: &str) -> Result<(), StoreError> {
    if rel.contains("..") || rel.starts_with('/') || rel.starts_with('\\') {
        return Err(StoreError::Baca(rel.into(), "path ditolak".into()));
    }
    let p = dir.join(rel);
    if p.extension().map(|x| x == "yaml" || x == "yml").unwrap_or(false) && p.is_file() {
        std::fs::remove_file(&p).map_err(|e| StoreError::Baca(p.display().to_string(), e.to_string()))
    } else {
        Err(StoreError::Baca(rel.into(), "bukan file request".into()))
    }
}

/// Daftar flow: (file relatif dari flows/, Flow).
pub fn list_flows(dir: &Path) -> Vec<(String, crate::model::Flow)> {
    let mut out = Vec::new();
    let rd = match std::fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return out,
    };
    let mut files: Vec<std::path::PathBuf> = rd
        .flatten()
        .map(|e| e.path())
        .filter(|p| is_yaml(p))
        .collect();
    files.sort();
    for p in files {
        match load_yaml::<crate::model::Flow>(&p) {
            Ok(mut f) => {
                if f.name.is_empty() {
                    f.name = p.file_stem().unwrap_or_default().to_string_lossy().into_owned();
                }
                let rel = p.strip_prefix(dir).unwrap_or(&p).display().to_string().replace('\\', "/");
                out.push((rel, f));
            }
            Err(e) => warn!(file = %p.display(), error = %e, "flow diskip"),
        }
    }
    out
}

/// Tulis 1 flow ke flows/{slug}.yaml. Mengembalikan path relatif.
pub fn save_flow(dir: &Path, flow: &crate::model::Flow) -> Result<String, StoreError> {
    std::fs::create_dir_all(dir).map_err(|e| StoreError::Baca(dir.display().to_string(), e.to_string()))?;
    let name = if flow.name.trim().is_empty() { "Untitled flow".into() } else { flow.name.clone() };
    let mut f = flow.clone();
    f.name = name.clone();
    let path = dir.join(slugify(&name) + ".yaml");
    let text = serde_yaml::to_string(&f).map_err(|e| StoreError::Baca(path.display().to_string(), e.to_string()))?;
    std::fs::write(&path, text).map_err(|e| StoreError::Baca(path.display().to_string(), e.to_string()))?;
    Ok(path.strip_prefix(dir).unwrap_or(&path).display().to_string().replace('\\', "/"))
}

/// Hapus 1 file flow (path relatif dari flows/). Tolak path keluar dir.
pub fn delete_flow(dir: &Path, rel: &str) -> Result<(), StoreError> {
    if rel.contains("..") || rel.starts_with('/') || rel.starts_with('\\') {
        return Err(StoreError::Baca(rel.into(), "path ditolak".into()));
    }
    let p = dir.join(rel);
    if p.extension().map(|x| x == "yaml" || x == "yml").unwrap_or(false) && p.is_file() {
        std::fs::remove_file(&p).map_err(|e| StoreError::Baca(p.display().to_string(), e.to_string()))
    } else {
        Err(StoreError::Baca(rel.into(), "bukan file flow".into()))
    }
}
