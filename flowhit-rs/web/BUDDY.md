# FlowHit Buddy (Aoi-chan) — state 18 Okt 2026

## Aset
- `web/buddy-sheet.png` (1312x1199 RGBA, dari IMG_9933) — SATU-SATUNYA sheet. `buddy-sheet.jpg` usang, abaikan.
- Tabel crop `FR` (23 frame) di `web/index.html` dalam `drawBuddy()`. Sudah QC per-frame via contact-sheet: bersih label, utuh.

## Behavior (semua verified via browser_exec, JS_OK)
- Idle kursor: 9 arah + hysteresis (masuk ±70/±60, lepas ±40). Blink idle DIMATIKAN (user request 18 Okt).
- Skala: SEMUA frame fit contain ke box 96x92 (`FIT = min(96/sw, 92/sh)`), bottom-anchor — tidak ada lagi frame kepotong (tengah dulu 128px) atau kekecilan (tidur dulu 82). Tidak ada lagi beda BUST/full.
- Fetch (`S.sending`): cycle `pikir,bingung,tangandagu,ide` @700ms + bounce.
- Sukses (status !== 500): seq 6 frame bahagia + bounce 2,5 dtk.
- Status 500: `tengah` 3 dtk, mood `aduh… 500!`. Network error: `dizzy`.
- Tidur: manual klik (persist localStorage) + auto (idle 90 dtk / tab hidden / window blur; wake saat pointer/key). Sleep cycle 4 frame bust @1,2 dtk + overlay Zzz ungu animasi. Skala: full H=92, bust H=82, bottom-anchor.
- Posisi kartu: quest di atas, buddy di bawah (buddy menempati slot quest).
- reduced-motion: crossfade off, frame statis, Z statis.

## Test hook
- Hash `#buddy=<nama>` memaksa satu frame (mis. `#buddy=tengah`).
- `drawBuddy._cur` = frame aktif. Tab background men-throttle rAF — uji via panggil `drawBuddy()` manual.

## Pending
- User HARUS `Ctrl+Shift+R` di tab Brave (cache lama bandel; synthetic key tidak tembus fokus).

## Update 19 Okt pagi (CSS v19, sesi TUI)
- Pill ENV vs auth disamakan: `select-control` gap 8px/padding 0 10px (border mint + shadow dihapus), `#env` ikut `font:inherit`, option popup `#env`/`#authsel2` di-style gelap (bg surface, 12px, padding 8px 10px).
- Verified via browser_exec: kedua pill identik 36px/padding/border/bg; reload biasa cukup (tidak perlu hard refresh).

## Update 19 Okt siang (CSS v20, sesi Telegram)
- Select ENV native DIBUANG (option list OS tidak bisa di-style) → dropdown custom: tombol `#envbtn` (ENV ● STAGING ▾) + popover `#envpop` (header ENVIRONMENT, opsi dot + nama + ✓ aktif, hover raised, aktif mint).
- Pilih opsi → `S.env` uppercase + render(); klik-di-luar menutup (guard envbtn, pola tokpop). Perilaku kirim (`/api/send`) tidak berubah.
- Verified via browser_exec + screenshot: pill rapi, popover utuh tidak clipped, klik opsi + outside-close OK.
- Server: instance lama (PID 10652, /api/tree empty-reply) di-kill, fresh `target/debug/flowhit-server.exe` jalan di :4174.

## Update 19 Okt siang 2 (CSS v21, sesi Telegram)
- Auth (`authsel2` native) DIBUANG → dropdown custom pola ENV: tombol `#authbtn` (shield + label + ▾) + popover `#authpop` (header AUTH MODE, 4 opsi AUTH_MODES + ✓ aktif).
- Pilih opsi → `cur().authMode` + render(); token button/popover/tab Auth ikut update; klik-di-luar menutup.
- Verified: 4 opsi tampil (none aktif), ganti bearer/basic/none pill + tokbtn ikut berubah, screenshot pill rapi tidak clipped.

## Update 19 Okt sore (panel kiri audit + fix, sesi Telegram)
- BUG disclosure collection: key `c:undefined` (markup `data-cn`, handler baca `dataset.col`) → toggle tak pernah jalan. Fix 1 baris (`dataset.cn`), verified 4>1>4.
- Auto-test panel kiri (live, non-destruktif) JALAN: collapse/reopen, filter+Esc, menu+, new request, pilih request, recent, rowmenu pop, rename+Esc batal, toggle folder.
- Data aman: tidak ada file workspace berubah (baris New request test menguap saat reload; `folder/new-request.yaml` itu bawaan 18 Okt).
- BELUM terintegrasi: tombol Save di titlebar (tanpa handler), tab Params/Pre-request/Post-response (placeholder), tab response Headers/Cookies/Test Results (ganti label saja, isi tetap body), quest-card teks statis, achievement display-only.

## Update 19 Okt malam (integrasi bertahap, sesi Telegram, CSS v22)
- Save titlebar (`#titlesave`) di-wire via currentPayload → /api/save.
- Params: editor tabel (ON/KEY/VALUE, add/del) + ikut terkirim (?k=v) + persist yaml + import Postman query terbaca. Model Rust: `Param{k,v,on}`.
- Response tabs live: Headers (tabel + count), Cookies (parse Set-Cookie + count), Test Results (hasil assertions + badge lulus/total), Body raw beneran teks.
- Post-response: assertions status/body/time + tersimpan; dievaluasi tiap SEND → Test Results + badge VALIDATION inspector.
- Pre-request: perintah `header`, `param`, `delay` (max 5s) jalan sebelum SEND; tersimpan per-request.
- Quest-card teks dinamis (sisa hitung / complete).
- Fix ikut: SEND dari tab non-Body crash (textarea tak ada) → fallback q.body; save-as/dup/undo-restore bawa params/tests/prescript.
- Verified E2E via httpbin: ?foo=bar&via=script terkirim, echo ✓, save→yaml→reload pulih.
- SISA: tab Headers request masih statis (Add header mati) — kandidat berikutnya.

## Update 20 Okt dini hari (3 fitur besar, sesi Telegram, CSS v23)
- Headers request jadi editor (ON/KEY/VALUE, add/del/rename, count live) + persist yaml (map) + terkirim (default Content-Type + merge prescript). Rust: SaveIn.headers, TreeItem.headers.
- Multi-tab request ala Postman: strip tab (badge method + nama + ×), klik baris/recent/filter buka-atau-aktifkan, tutup tab aman (tetangga aktif, minimal 1), validTabs() selamatkan index basi tiap reload.
- JSON highlight response pretty (key mint, angka amber, bool/null merah) + raw kini tampil teks (dulu kosong) + pretty tampil penuh (dulu kepotong 1 baris).
- Verified: header custom X-Trace terkirim (httpbin echo YES), tab open/switch/close, span highlight computed mint; yaml bersih (header test dibuang tanpa save).

## Update 20 Okt pagi (dirty indicator + warna key JSON, sesi Telegram, CSS v24)
- JSON highlight response: key (:) lavender (`j-key`), value string mint, angka amber, bool/null merah. Indent rapi via pretty-print 2-spasi.
- Dirty tracking per-request (snapshot `_base` saat render pertama; reload dari server me-reset): tab × jadi • amber + tombol Save ikut amber saat ada perubahan.
- Klik •/× di tab kotor → modal: SIMPAN (save lalu tutup tab yang benar via cari file) / BUANG (restore dari snapshot, tutup) / BATAL.
- Fix ikut: wiring tc-* sempat masuk askDelete (tak pernah jalan) → pindah ke bind().
- Verified: dot muncul saat edit, BATAL diam, BUANG restore URL + dot hilang, SIMPAN tulis yaml + tutup; data user dikembalikan pristine.

## Update 20 Okt pagi 2 (editor body highlight, sesi Telegram, CSS v25)
- Editor Body JSON kini berwarna saat mengetik (teknik backdrop: textarea transparan + `code-backdrop` sinkron input/scroll).
- Verified computed: key lavender rgb(170,161,214), string mint rgb(88,214,181); no-ghosting (textarea transparent).

## Update 20 Okt pagi 3 (dirty live + editor upgrade, sesi Telegram, CSS v26)
- Akar dot tak muncul: indikator hanya update saat render, mengetik tidak render (jaga fokus). Fix: refreshDirty + listener dokumen sekali (_fhDirty) → dot/Save update tiap ketikan.
- Editor Body: Tab = 2 spasi (Shift+Tab dedent, seleksi multi-baris), auto-close pair (skip quote, backspace hapus pasangan), Enter auto-indent (smart brace).
- Verified: dot-live, Tab, pair, Enter indent; data test in-memory saja (tak tersimpan).

## Update 20 Okt pagi 4 (buang-tutup + modal UX, sesi Telegram, CSS v27)
- BUANG kini benar-benar tutup tab; tab terakhir boleh habis → editor kosong (S.sel=null + guard cur/curCol/curFol/rowReq/m-new/reload).
- Modal redesign: eyebrow UNSAVED CHANGES amber + nama file, Buang amber kiri / Batal ghost / Simpan primary kanan + fokus + Enter=Simpan, Esc=Batal.
- Verified: tutup semua tab → 0 tab + editor kosong + buka lagi via sidebar; modal hirarki jelas di screenshot; Esc tutup.

## Update 20 Okt pagi 5 (sash response-editor, sesi Telegram, CSS v28)
- Grip horizontal baru `#grip3` di atas panel RESPONSE (gaya ⋯, cursor row-resize, clamp 120–800px, var --resp-h).
- Verified: drag sintetis +100px → var 272px + panel membesar; sash terlihat di screenshot.

## Update 20 Okt pagi 6 (audit sisa + fix, sesi Telegram, v29)
- Export JSON (200 + ada params) dan Import Postman (query→params) verified via API; koleksi test dibersihkan.
- Fix: Ctrl+S kini simpan (dulu diiklankan tapi mati), r-open rowmenu kini buka tab (dulu cuma tutup menu).
- Verified browser: Ctrl+S dot→× + yaml tertulis + revert pristine; r-open buka tab Ser.

## Update 20 Okt siang (import rekursif, sesi Telegram, backend only)
- BUG import: folder Postman jadi request sampah + isi hilang. Fix `import_item` rekursif (folder→subfolder, dalam diratakan "a / b").
- Verified: Nested Req masuk foldera + header X-A + body ikut; Top Req di root; koleksi test dibersihkan.

## Update 20 Okt sore (5 fitur Postman, sesi Telegram, CSS v30–33)
- `⧉ curl`: composeSend tunggal (SEND + curl konsisten) + clipboard race 1500ms + fallback prompt + label ✓ copied.
- History: S.hist + localStorage fh-hist (50) + seksi HISTORY sidebar + klik = tab unsaved + histclear.
- Env editor: modal ✎ Manage variables dari popover ENV → CRUD key/value → POST /api/env-save (save_env_vars, name case-insensitive) → reloadTree. Fix ikut: secrets_id sempat hilang dari yaml (struct Env tak punya field) → tambah field + revert staging pristine.
- Form-data: body_mode json/form + tabel FORM (on/key/type text-file/value-file) + multipart Rust (reqwest multipart + base64 file via FileReader) + save/tree/history/curl ikut (-F). Verified API + UI echo httpbin.
- Runner: ▶ Run collection/folder di menu koleksi → modal RUNNER (progress per-request ✓/✗/…/○ + status•ms + ringkasan lulus + Stop/Tutup + Esc). Lulus = tests semua ok, atau status <400 bila tanpa tests.
- Verified: curl output + history reopen; env-save staging rt + revert; multipart echo YES; runner General 3/3 ✓ + modal tutup bersih.

## Update 20 Okt sore 2 (env jadi panel editor full, sesi Telegram, CSS v34)
- Modal env dibuang: ✎ Manage variables kini buka ENVIRONMENT FILE sebagai panel editor full (request-area + response-area disembunyikan, breadcrumb ENVIRONMENTS / nama).
- Nama env bisa diketik langsung di kolom ENV (besar, uppercase mint) → Save memanggil /api/env-rename dulu bila berubah (file yaml ikut ke-rename), lalu /api/env-save.
- ＋New: prompt nama → POST /api/env-create → langsung dibuka di panel. Switcher pill per env + dirty indicator live (• belum disimpan / ✓ tersimpan) + Esc/← Requests kembali (confirm bila kotor).
- Backend: store create_env + rename_env, route env-create/env-rename. Fix ikut: build error OsStr vs String di rename.
- Verified: panel full + response hidden; tambah var → dirty → save → yaml tertulis + secrets_id utuh; create Lab → rename Lab2 (file lab2.yaml) via API; bersih-bersih (lab2 dihapus, staging revert pristine); screenshot panel ✓.

## Update 20 Okt sore 3 (runner = SEND, sesi Telegram, tanpa bump CSS)
- Runner dulu kirim mentah (tanpa prescript/form). Refactor: logika prescript diekstrak jadi applyPrescript() dipakai bersama composeSend + runScope.
- Runner kini: params + header prescript + param prescript + delay + Content-Type otomatis + form text fields ikut terkirim (file diskip — File object tak bisa persist di tree).
- Verified: unit applyPrescript (header+param+delay ✓); runScope items bawa prescript/body_mode/form; regresi runner General 3/3; regresi SEND 200.

## Update 20 Okt sore 4 (env delete + Ctrl+S panel, sesi Telegram, tanpa bump CSS)
- CRUD env genap: tombol 🗑 di panel + DELETE /api/env-delete (tolak hapus env terakhir); habis hapus tutup panel + env aktif pindah ke sisa pertama.
- Ctrl+S di panel env kini simpan env (sebelumnya mati karena titlesave disembunyikan).
- Verified: create→delete API ok; hapus staging sendirian ditolak; Ctrl+S simpan var uji; staging revert pristine (1 file).

## Update 20 Okt sore 5 (export Postman beneran, sesi Telegram, tanpa bump CSS)
- Tombol export dulu unduh format internal (tak bisa dibuka Postman). Kini GET /api/export-postman → Collection v2.1 asli (folder bersarang, query+disabled, form-data, events prerequest/test dari prescript/checks) → file flowhit-postman.json.
- Import ikut diperkuat: formdata → form fields + body_mode form; raw URL di-strip query-nya (sebelumnya param kekirim dobel).
- Verified: schema collection.json, 4 item; events prerequest+test ter-generate; round-trip import ok (url bersih + params); TmpEv + Imported dibersihkan, tree pristine 3+1.

## Update 20 Okt sore 6 (verifikasi klik beneran via computer_use, sesi Telegram)
- Klik asli di Brave (bukan sintetik): buka Tes Export → tab kebuka; SEND → 200 + Headers 7 + Test Results 2/2 + history keisi + quest complete; ENV ▾ → popover → Manage → panel ENVIRONMENT FILE full; ← Requests → balik; menu workspace → Export JSON → dialog Save → file valid Postman v2.1 (4 item).
- Catatan: right-click koleksi via AX buka menu native (menu app via tombol Collection menu ✓); tombol dialog Save butuh UIA Invoke (klik background biasa tak tembus).
- Workspace pristine (staging bersih, tree 3+1).

## Update 20 Okt sore 7 (auth mode persist per request, sesi Telegram, tanpa bump CSS)
- Dulu tiap buka ulang request balik No auth. Kini mode-nya tersimpan di yaml (auth.mode) — kredensial tetap session-only, tak pernah tersimpan.
- Server: SaveIn.auth_mode → Request.auth.mode; TreeItem.auth_mode; export → blok auth Postman; import → mode dari Postman auth (secret dibuang).
- Client: load mapping auth_mode; semua jalur save (payload/rename/save-as/duplicate/undo) bawa auth_mode; runner pakai kredensial sesi sesuai mode tersimpan.
- Verified: save bearer → yaml + tree; reload → pill Bearer + kirim tanpa token ditolak; import bearer (secret dibuang); Tes Export revert none + tests utuh; Imported dibersihkan.

## Update 20 Okt malam (part 6 x5, sesi Telegram, tanpa bump CSS)
- ⤓ Save response: unduh body (nama dari path URL + ekstensi dari content-type).
- Filter highlight: ternyata sudah ada (hi + mark styled) — tanpa kerja.
- ➕ Import curl: modal tempel curl → parse (-X/URL/-H/--data*/-F/-b/-A) → tab unsaved General → kirim 200 ✓.
- 🍪 Cookie jar: set-cookie respons final tersimpan per-domain (localStorage) → otomatis terkirim (kecuali ada header Cookie manual) → tampil di tab Cookies + tombol hapus. Batas: redirect 302 tak tertangkap (reqwest follow otomatis).
- Ikon folder: 🎨 di menu folder → .meta.json per folder → tree bawakan icon. Verified buat-set-tampil-hapus.
- Semua temp dibersihkan (tree 3+1, staging pristine).

## Update 20 Okt malam 2 (runner via menu, sesi Telegram)
- Jalur UI penuh: ⋯ koleksi → Run collection → modal → 3/3 lulus → Tutup bersih ✓.
- Catatan: menu ⋯ baru muncul saat hover (right-click murni buka menu native — by design, bukan bug).

## HANDOFF sesi baru (20 Okt malam, CSS v50) — baca ini dulu
- Server: `target/debug/flowhit-server.exe` (build 20 Okt: fix ikon General) di `http://127.0.0.1:4174/`. Buka `?v=50` (tab fresh; JANGAN reuse ?v= lama — HTML ke-cache). Restart bila `/api/tree` empty-reply: taskkill via `cmd /c 'taskkill /F /IM flowhit-server.exe'` + jalankan ulang.
- State: cols General(4: pm-test/ser/tes-export/Demo Flow Echo) + folder(1: new-request), envs [staging] + Demo Simple + Demo Complex (11 node, layout mirror tersimpan). History/quest apa adanya. Temp bersih (test icon/flow dibuang).
- Selesai 20 Okt: rail pembatas buku + tab kontekstual + tanpa title (v45-48), folder/request ramping (v47/v49), ikon 📦 + fix ganti ikon koleksi/folder incl. General (backend col_icon/set_icon), Demo Complex + fix setvar fixpoint, 🧹 auto-layout, kabel output-kiri/input-kanan + mirror, inspector/log mengambang, follow kamera + 🎯.
- Aturan: stabilitas > fitur; tanpa prompt()/confirm() bawaan; workspace revert setelah test; tiap blok update BUDDY.md; `node --check` tiap edit JS index.html; verifikasi headless: scroll sintetis tak DIF event (pakai wheel), node pakai pointerdown (klik sintetis tak mempan).
- Ide berikutnya (belum dikerjakan): flow edge-drag manual, file-upload di flow-run, OAuth2/digest, cookie redirect-302, search Ctrl+K, footer editor.

## Update 20 Okt malam 3 (env ✕ + modal new + fix cache API, sesi Telegram)
- Panel env kini ada tombol ✕ kanan atas (selain ← Requests/Esc).
- ＋New tak lagi prompt() jadul → modal NEW ENVIRONMENT (input + Enter=Buat + Esc=Batal).
- Bug ketemu saat verify: GET /api/tree di-cache browser → env baru tak muncul. Fix: fetch cache:'no-store' di api().
- Verified: × tutup; modal buat FixCheck2 → switcher 2 pill; FixCheck+FixCheck2 dihapus, staging saja.

## Update 20 Okt malam 4 (env +add inline, sesi Telegram, CSS v36)
- Tombol tambah kini baris dashed ＋ Tambah variable nempel 0px di bawah variable terakhir (dulu button jauh di bawah).
- Verified: gap 0px, klik → baris baru + fokus; tanpa save (memori saja).

## malam5 — env file-tabs + zero native dialog (19 Okt)
- Switcher pills STAGING → file-tabs ala request tabs (`staging.yaml` + ikon ≣ + dot dirty •), tombol ＋ dashed.
- Tombol `← Requests` → `‹ Kembali`; breadcrumb `ENVIRONMENTS / staging.yaml`.
- Dirty dot live di tab aktif via refreshEnvStatus tanpa re-render (fokus ketik aman).
- Zero native dialog: confirm/prompt tinggal 1 (fallback copy-curl). New collection/folder inline rename, save-as auto `copy/copy 2`.
- Local-only col/folder: `_serverCols/_localFol` + keep di reloadTree; rename lokal tanpa backend.
- Verified fh-verify ?v=40: tabs `≣staging.yaml|＋`, dot `•`, folder `Folder Baru` survive reload, save-as `PM Test copy` lalu delete; workspace pristine.

## malam6 — curl bawa auth (19 Okt)
- `composeSend` return +`authMode`; `toCurl` append header auth sesuai mode (bearer/basic btoa/apikey + default X-API-Key).
- Verified ?v=41: bearer/basic/apikey masuk header, mode none bersih.

## malam7 — warna method konsisten (19 Okt)
- `tone()`: PUT→sky(biru), PATCH→lav, DELETE→red, GET→mint, POST→amber. Badge tree+tabs+response ikut.
- `.method-text.put/.patch/.delete` di recent/history (sebelumnya abu-abu).
- Verified ?v=42 computed: mint rgb(88,214,181), sky rgb(131,183,255), red rgb(237,119,111).

## malam8 — grip drag persist + HANDOFF SESI BARU (19 Okt)
- Bug: grip1/2/3 tulis CSS var inline di `.app-shell`/`.workspace`, tapi tiap render() rebuild DOM → balik default. Fix: simpan `S.sideW/S.inspW/S.respH` tiap drag + template render style inline.
- Verified ?v=43: `--side-w:320px; --insp-w:350px | --resp-h:420px` persist across render.

## malam9 — FLOW v1 (request berantai ala Postman Flows, 19 Okt, CSS v37)
- Backend: model Flow/FlowNode/FlowCond/FlowExtract + store list/save/delete (`workspace/flows/*.yaml`) + routes flow-save/flow-delete + tree bawakan `flows`. Build OK.
- Editor: seksi FLOWS di sidebar + panel full (toolbar: Kembali/nama/dirty/palet ＋▶＋◆＋ϟ＋⟳＋■/Save/Run/Stop/🗑) + kanvas drag-drop node + kabel bezier (T mint/F merah) + inspector per kind.
- Node: start → request (ref file + extract status/body/header/json:$.path → {{VAR}}) → decision (status/body/json/var/header vs ==,!=,>,<,>=,<=,contains,not_contains → TRUE/FALSE) → setvar ({{var}} templating) → loop (count max 25 / while + target badan loop) → end. Kabel via dropdown inspector.
- Runner client-side (pakai jalur /api/send: prescript/params/auth/jar ikut): guard 60 langkah, log live + ringkasan ✓/✗, Stop, Esc/Kembali confirm bila kotor, Ctrl+S simpan flow, zero native dialog.
- Verified ?v=37: save→yaml→tree; run setvar→request→extract FOO=bar→decision TRUE→end (200 ✓); loop ×2 + cabang FALSE ✓; klik-select + drag persist + dirty; screenshot kanvas rapi; FlowUji dihapus, workspace pristine.
- Batas v1: tanpa edge-drag manual (via inspector), tanpa file-upload di flow-run, tanpa OAuth/digest, while-loop pakai respons terakhir.

## malam10 — motion flow run (19 Okt, CSS v38)
- Kabel menuju node aktif: putus-putus mint jalan (`flow-dash` 0.5s, T/F ikut warna cabang).
- Node aktif: gemetar gaming (`flow-shake` 0.35s) + glow amber.
- Jejak full start→end: kabel terlewati tetap mint solid + node terlewati ring mint (`path` per run, bertahan setelah selesai).
- `prefers-reduced-motion`: animasi mati. Verified ?v=38: trail s,r,d,e + 4 kabel + 4 node ✓.

## malam11 — sidebar breathing + highlight tambah (19 Okt, CSS v39)
- Sidebar: row 36px + radius + margin antar baris, grup dipisah divider dashed, section-label lega. Tombol ＋ (Collections/FLOWS) + palet flow + Add param/header: tint mint halus (hover naik dikit).
- Verified ?v=39 + screenshot: lega, ＋ gampang ketemu, tidak norak.

## malam12 — pacing run + sidebar collapse/scroll (19 Okt, CSS v40)
- Run pacing: request 900ms, non-request 500ms — getar + aliran data terlihat dari start sampai end. Stop tetap responsif.
- FLOWS/RECENT/HISTORY: klik header collapse/expand (▸/▾), isi max-height + scroll sendiri.
- Sidebar flex: collections+sections scroll di tengah (`side-scroll`), quest + Aoi-chan pin bawah — buddy tak pernah tertutup. Row 32px + radius + divider.
- Verified ?v=40: collapse ✓; buddy+quest visible ✓; screenshot rapi.

## malam13 — klik request keluar dari flow (19 Okt, tanpa bump CSS)
- Bug: klik request/RECENT/HISTORY saat flow terbuka tak ke mana-mana (panel kirim tak muncul). Fix: openReq keluar dari flowView otomatis; flow kotor → modal Buang/Batal, Buang lanjut navigasi (`flowAfter`).
- Verified ?v=40: bersih langsung pindah + SEND ada; kotor → confirm → Buang → pindah ✓.

---
# HANDOFF — lanjut sesi baru dari sini
- Server: `target/debug/flowhit-server.exe` di `http://127.0.0.1:4174/` (restart bila `/api/tree` empty-reply: taskkill + `cargo run`). Verifikasi UI di `?v=40`.
- Malam ini (malam8→malam9): grip persist (?v=43) + FLOW v1 full (lihat entri malam9 di atas).
- Workspace pristine: cols General(3)+folder(1), envs staging.yaml saja, flows kosong.
- Aturan: stabilitas > fitur; tanpa prompt()/confirm() bawaan; workspace revert setelah test; tiap blok update BUDDY.md.
- Ide berikutnya (belum dikerjakan): pola mockup Figma — tombol Import + Paste cURL di sidebar, footer editor, search Ctrl+K, Prettify/Wrap; flow: edge-drag manual, file-upload di flow-run.

## malam14 — sidebar rapi v41 (20 Okt, sesi Telegram, CSS v41)
- Nama request satu baris + ellipsis (dulu `New request` wrap 2 baris, badge GET tidak sejajar): `.request-row [data-reqname]` nowrap/ellipsis/min-width 0 + badge & tombol flex none + row overflow hidden.
- Gutter scrollbar (`scrollbar-gutter: stable`) + padding: angka count tak terhimpit scrollbar; filter kbd tak nempel border.
- Buddy lega: margin-top 12px + teks min-width 0 + mood ellipsis; sidebar 224→240px di layar ≤1450px.
- Verified ?v=41: 4 row uniform 32px + nowrap computed; screenshot sidebar rapi, buddy utuh + napas.

## malam15 — quest in-flow + compact sidebar (20 Okt, sesi Telegram, CSS v42)
- Akar terhimpit: `.quest-card` absolute (bottom 38px, width 228px) numpuk buddy di window pendek. Fix: static in-flow + margin (flex order: scroll→quest→buddy, tak bisa tertutup lagi).
- Compact: heading 13px, search/collection row 28px, request row 28px/11px, recent 26px, buddy canvas 84→64px + padding 8px.
- Catatan: `@media (max-height:760px)` sembunyikan quest by design (buddy tetap tampil).
- Verified ?v=42: questPos static + overlap false + buddy dalam sidebar + rows 28px; screenshot rapi.
- Urutan (request user): buddy di atas quest → `.sidebar .quest-card{order:1}` (CSS v43). Verified computed order quest 1 > buddy 0.

## malam16 — sidebar lega + ramping (20 Okt, sesi Telegram, CSS v44)
- Panel terlalu sempit (±180px, grip ketarik). Fix: floor `minmax(248px, var(--side-w, 272px))` di ≤1450px (default 272, tak bisa kejepit lagi).
- Isi ramping: heading 12px, search 26px, collection/request row 26px, recent 24px, buddy 56px + padding 6px, quest padding 6px.
- Verified ?v=44: sideW 272 + rows 26px + buddy dalam sidebar; screenshot lega, buddy utuh di atas quest.

## malam17 — sidebar jadi tab (20 Okt, sesi Telegram, CSS v45)
- Recreate panel kiri: tab COLLECTIONS (4) / FLOWS (1) / HISTORY (0) gantikan tumpukan semua-seksi (request user).
- State `S.sideTab` persist localStorage `fh-sidetab`; search filter hanya tampil di tab Collections; konten digate via class `stab-*` + `:has` (nav vs section flows/recent/history).
- Verified ?v=45: ketiga tab tampilkan konten yang benar + badge count + screenshot; binding lama (collapse/filter/buka request) tak tersentuh.

## malam18 — rail pembatas buku (20 Okt, sesi Telegram, CSS v46)
- Tab horizontal dibuang (dinilai jelek) → rail vertikal 36px di tepi kiri: ikon + label vertikal + count, aktif mint (request user).
- Struktur: aside jadi row (rail + side-body column); judul h2 ikut tab aktif; binding data-stab tak berubah.
- Verified ?v=46: rail 3 tab + klik Flows/History ganti konten + h2 ikut; screenshot mirip pembatas buku, layout utuh.

## malam19 — folder minimalis (20 Okt, sesi Telegram, CSS v47)
- Baris folder ramping (request user): row 22px, nama 11px, ikon 12px/16px, count 8px, grup spacing rapat.
- Verified ?v=47: computed pas + screenshot minimalis rapi tanpa kepotong.

## malam20 — ＋ kontekstual + tanpa title (20 Okt, sesi Telegram, tanpa bump CSS)
- Tombol ＋ header ikut tab: flows → New flow langsung (flownew-h), hist → hapus history (histclear-h), col → menu koleksi. Title section FLOWS/RECENT/HISTORY + h2 Flows/History dibuang (redundant, rail sudah bicara).
- Verified ?v=48: label hide + flownew-h buka panel + Buang bersih (tanpa file); hist ⌫ + col menu; screenshot bersih.

## malam21 — ikon default 📦 + fix ganti ikon (20 Okt, sesi Telegram, tanpa bump CSS)
- Default collection 📁 → 📦 (request user) + label menu ikut.
- BUG ganti ikon tak jalan: (1) tombol 🎨 Folder icon tanpa binding → tambah r-ficon → iconTarget{col,fol}; (2) backend tolak General (virtual, bukan dir): set_icon/col_icon kini pakai collections/.meta.json untuk General.
- Verified API + UI E2E: set 🦊 tampil + tree bawa ikon; reset "" + meta dihapus (pristine).

## malam22 — nama + baris request ramping (20 Okt, sesi Telegram, CSS v49)
- Nama request 13→11px + baris 24px + badge 15px (request user, terlalu besar).
- Temuan: `div.request-row{font:inherit}` kalahkan aturan class → pakai selektor `div.request-row` + target span langsung.
- Verified ?v=49: computed 11px/24px + screenshot rapi sejajar.

## malam23 — Demo Complex + fix setvar berantai (20 Okt, sesi Telegram, tanpa bump CSS)
- Demo flow baru "Demo Complex" (11 node) + request "Demo Flow Echo": setvar + templating berantai → params & body JSON {{VAR}} → extract status/json/header → decision ==, contains, var: → loop count 2x → end ganda.
- BUG ketemu saat verify: key vars diacak backend (HashMap) → `halo-{{CITY}}` tak resolve. Fix: substitusi fixpoint di setvar (pass hingga stabil, anti-siklus).
- Verified run E2E: path s,v,g,d1,p,d2,d3,l,t,l,t,l,e2 + pass 4/4 + semua VAR resolve (GREETING=halo-jakarta, ORDER_ID=ORD-2).

## malam24 — tombol 🧹 Rapikan layout flow (20 Okt, sesi Telegram, tanpa bump CSS)
- Jawaban kabel semrawut: auto-layout berlapis (kolom per kedalaman BFS, loop-target sekolom loop di bawahnya, end bersama di kolom akhir, node yatim diurut akhir).
- Verified: acak total → Rapikan → overlap NIHIL + loop sekolom + e1 kolom akhir; Save → yaml persist (chain 40→1960 step 240).

## malam25 — kabel kanan-kiri + panel mengambang + follow kamera (20 Okt, sesi Telegram, CSS v50)
- Kabel bezier tetap, port dibalik (output kiri, input kanan) + auto-layout mirror (start paling kanan); Demo Complex dirapikan ulang + save.
- Inspector flow + log run jadi overlay mengambang di kanvas (inspector hanya saat node dipilih + tombol ×; log pojok kiri-bawah + Tutup).
- Kamera: default ikuti node berjalan (is-cur), scroll/wheel manual melepas + tombol 🎯 muncul untuk fokus lagi; suppress 800ms + instant jump (smooth tak jalan di headless).
- Pelajaran: sintaks `let` ganda sempat bunuh render (root kosong) → cek `node --check` tiap edit JS; scroll sintetis headless tak DIF event → uji wheel sintetis.
- Verified: edge M=x1=kiri + overlap NIHIL + yaml mirror + insp/log absolute + jump 984/1224 + wheel lepas + 🎯 refokus.

## malam26 — flow LTR + log terkunci panel + kanvas penuh (20 Okt, sesi Telegram, CSS v51)
- Balik arah (request user, lawan malam25): autoLayout `x = 40 + depth*DX` (start paling kiri); kabel keluar sisi kanan → masuk sisi kiri (`M x1 y1 C x1+dx y1, x2-dx y2, x2 y2`), pola app node standar.
- BUG log keluar panel: `.flow-log` absolute tapi `.flow-panel` tanpa position → nempel ancestor jauh. Fix: `.flow-panel{position:relative}` → log tetap mengambang tapi di dalam panel tengah.
- Kanvas penuh: `.flow-main` grid 1fr permanen (kolom inspector 264px walau hidden dihapus), `.flow-panel`+`.flow-main` flex:1 + canvas height:100%.
- Verified: CSS v51 + edge x1+dx tersaji server. Demo Complex di-tidy + save via browser (s:40→e1:2200 LTR, yaml persist).

## malam27 — port kabel drag-and-drop (20 Okt, sesi Telegram, CSS v52)
- Tiap node: port input kiri biru + port output kanan (hijau; decision TRUE/FALSE, loop badan-loop/keluar-loop). Node height fix 58px agar port sejajar dengan endpoint kabel (PMID 29 / HI 16 / LO 42).
- Drag output→input = sambung (ganti koneksi field terkait); drag input→area kosong = putus kabel masuk. Drop invalid (diri sendiri/start) batal. Selama drag: garis live + semua input valid highlight.
- Bonus: kabel loop_target dulu tak digambar → kini ada (hijau putus-putus + tag ⟳); kabel ikut nempel live saat node di-drag.
- Verified sintetis: d1.TRUE→e1 tersambung (edge M 928 76 C … 2200 89) lalu putus lagi; yaml tak tersentuh (tanpa save).

## malam28 — scroll kanvas anti-reset (20 Okt, sesi Telegram, tanpa bump CSS)
- BUG serius: render() rebuild #root → kanvas baru scroll 0 → tiap drop node / sambung kabel / ganti dropdown inspector, pandangan mental ke kiri. Fix: stash scrollLeft/Top di awal render(), restore setelah bind() (dikecualikan saat flow-run follow aktif agar kamera tetap jalan).
- Verified sintetis: scroll 1500 → klik node (render) → tetap 1500 + seleksi jalan.

## malam29 — routing kabel rapi: bump + siku bawah (20 Okt, sesi Telegram, CSS v53)
- Aturan 3 kasus (request user): node sama → SATU kabel bump di atas node + panah dua arah + dash jalan (arus bolak-balik); target di kiri → siku orthogonal lewat lajur bawah (tiap kabel lajur sendiri +16px, sudut r=10); target di kanan → bezier lama.
- flowEdgeD kini return {d,tx,ty} + slot lajur disimpan di data-slot (live-drag ikut aturan sama); marker panah #flarr (mint) di defs SVG.
- Verified sintetis: self-loop g:g → is-bidir bump; mundur e1→g → siku L/Q lajur 184; restore tanpa save.

## malam30 — tipe kabel 1-arah / 2-arah (20 Okt, sesi Telegram, CSS v54 + rebuild server)
- Drop kabel kini buka chooser: → 1 arah (default) atau ⇄ 2 arah (tunggu respons). Klik kabel juga bisa (hit-path tak terlihat) → ganti tipe via inspector. Esc/klik-luar batalkan.
- Model: FlowNode.await_ids (Rust + serde default, yaml persist, tree bawa). 2-arah = visual panah ganda + runner log ⇄ menunggu respons / respons ✓. Jujur: eksekusi memang sudah sekuensial, jadi efek runtime hari ini = kontrak visual + log; jadi fondasi fan-out nanti.
- Verified: save→yaml→tree bawa await_ids:["b"] (temp dibersihkan); sintetis d3.FALSE→l 2-arah → cls is-false is-bidir + inspector; yaml demo tak tersentuh.

## malam31 — klik kabel + Delete untuk putus (20 Okt, sesi Telegram, tanpa bump CSS)
- Delete/Backspace (di luar input teks) + wireSel → field dikosongkan + await dibersihkan + render. BUG saat verify: klik kabel decision simpan port `true` bukan field `next_true` → seleksi langsung dibuang; fix helper flowWireField() dipakai klik-kabel + drop + chooser.
- Verified sintetis: klik hit-path d1:true → wireSel next_true → Delete → field kosong + await bersih + edge hilang.

## malam32 — error log run + klik-luar tutup chooser (20 Okt, sesi Telegram, tanpa bump CSS)
- Error `✗ POST Echo: request tak ketemu (demo-flow-echo.yaml)`: file request demo kehapus, ref flow yatim. Fix: bikin ulang collections/demo-flow-echo.yaml (POST {{base_url}}/post + body/QS city-qty agar extract ECHO_QTY/ECHO_CITY/CT cocok).
- Klik-luar tak nutup: veil + Esc sudah ada & verified jalan; tambah jaring pengaman pointerdown-capture (klik di luar .wire-pop pasti tutup, veil tak lagi satu-satunya jalan).
- Verified: Demo Complex full hijau (POST Echo 200 + 3 extract + loop 2× + Sukses); Demo Simple tetap hijau.

## malam33 — opsi kabel auto-tutup (20 Okt, sesi Telegram, tanpa bump CSS)
- Klik kabel → panel TIPE KABEL di inspector dulu nempel terus. Kini batal otomatis saat klik node lain (node-select clear wireSel) maupun klik area kosong kanvas (capture pointerdown, kecuali klik kabel/node/inspector/pop).
- Verified sintetis: cable_ui tampil → klik kanvas kosong clear → set lagi → klik node clear + seleksi node jalan.

## malam34 — kabel 2-arah di Demo Complex (20 Okt, sesi Telegram, tanpa bump CSS)
- Kabel loop l→t (Ulangi → GET Root) jadi 2-arah: l.await_ids=[t] di yaml → edge hijau is-bidir + run log ⇄ menunggu respons / respons ✓ tiap iterasi.
- Temuan: demo-flow-echo.yaml HILANG LAGI dari disk (kemungkinan kehapus saat testing) → flow error lagi. Bikin ulang + verified full hijau. Jangan hapus request itu selama flow masih mereferensikannya.
- Verified: run Complex 14 baris — POST Echo 200 + 3 extract + 2× (menunggu respons → 200 → respons ✓) + Sukses.

## malam35 — satu kabel bolak-balik (20 Okt, sesi Telegram, tanpa bump CSS)
- Benar kata user: dua kabel l↔t mubazir. Runner kini return-stack: susur kabel 2-arah → push pemanggil; node tanpa next + stack ada → ↩ kembali ke pemanggil (cap 25, guard 60 tetap). Kabel balik t→l dihapus dari yaml → tinggal SATU kabel hijau panah ganda.
- Aturan main: pulang = jalankan ulang node pemanggil (pas untuk loop; request pemanggil = kirim ulang = polling).
- Verified: 1 kabel l:target is-bidir; run 17 baris — 2× (menunggu → 200 → respons ✓ → ↩ kembali) + selesai + Sukses.
- Catatan: demo-flow-echo.yaml hilang 2× saat sesi ini (pm-test.yaml ikut berubah 23:20 — ada yang utak-atik workspace); bikin ulang tiap kali. Jangan hapus request yang dipakai flow.
