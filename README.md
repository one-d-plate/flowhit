# FLOWHIT PIXEL — source

API client Windows/macOS gratis + maskot Aoi. Rust (axum) + WebView2/WKWebView + vanilla JS, UI bahasa Indonesia.

- `flowhit-rs/` — server + web UI + workspace contoh
- `flowhit-app/` — launcher desktop (tao/wry) + `installer.iss` (Inno Setup)

## Build lokal

```sh
cd flowhit-rs && cargo build --release --bin flowhit-server
cd ../flowhit-app && cargo build --release
```

## Rilis

Push tag `vX.Y.Z` → GitHub Actions bangun installer Windows + DMG macOS,
terbit otomatis di **Releases**.
