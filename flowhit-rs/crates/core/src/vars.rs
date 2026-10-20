//! Substitusi {{var}} + error rapi.
use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VarsError {
    #[error("variable hilang: {0:?}")]
    Missing(Vec<String>),
}

fn re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"\{\{\s*([a-zA-Z0-9_.]+)\s*\}\}").expect("regex valid"))
}

/// Mengembalikan (teks jadi, daftar var hilang).
pub fn resolve(s: &str, vars: &HashMap<String, String>) -> (String, Vec<String>) {
    let mut missing = Vec::new();
    let out = re()
        .replace_all(s, |caps: &regex::Captures| {
            let k = &caps[1];
            match vars.get(k) {
                Some(v) => v.clone(),
                None => {
                    missing.push(k.to_string());
                    caps[0].to_string()
                }
            }
        })
        .into_owned();
    (out, missing)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ganti_var_dasar() {
        let mut v = HashMap::new();
        v.insert("base_url".into(), "https://x.id".into());
        let (s, miss) = resolve("{{base_url}}/post", &v);
        assert_eq!(s, "https://x.id/post");
        assert!(miss.is_empty());
    }
    #[test]
    fn var_hilang_dilaporkan() {
        let (s, miss) = resolve("{{a}}/{{b}}", &HashMap::new());
        assert_eq!(s, "{{a}}/{{b}}");
        assert_eq!(miss, vec!["a", "b"]);
    }
}
