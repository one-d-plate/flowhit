#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::net::TcpStream;
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use tao::event::{Event, WindowEvent};
use tao::event_loop::{ControlFlow, EventLoop};
use tao::window::WindowBuilder;
use wry::{NewWindowResponse, WebViewBuilder};

#[cfg(windows)]
const CREATE_NO_WINDOW: u32 = 0x0800_0000;
const PORT: u16 = 4174;

fn wait_port(timeout: Duration) -> bool {
    let t0 = Instant::now();
    while t0.elapsed() < timeout {
        if TcpStream::connect(("127.0.0.1", PORT)).is_ok() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    false
}

fn load_icon() -> Option<tao::window::Icon> {
    let img = image::load_from_memory(include_bytes!("../icon.png"))
        .ok()?
        .to_rgba8();
    let (w, h) = img.dimensions();
    tao::window::Icon::from_rgba(img.into_raw(), w, h).ok()
}

fn server_candidates(exe_dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let bin = if cfg!(windows) { "flowhit-server.exe" } else { "flowhit-server" };
    let mut v = vec![exe_dir.join(bin)];
    if let Some(res) = exe_dir.parent().map(|p| p.join("Resources")) {
        v.push(res.join(bin));
    }
    v
}

fn data_dir() -> std::path::PathBuf {
    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            let d = std::path::PathBuf::from(home)
                .join("Library/Application Support/FLOWHIT PIXEL");
            let _ = std::fs::create_dir_all(&d);
            return d;
        }
    }
    std::env::current_dir().unwrap_or_else(|_| ".".into())
}

fn main() -> wry::Result<()> {
    let dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| ".".into());
    let server = server_candidates(&dir)
        .into_iter()
        .find(|p| p.is_file())
        .expect("flowhit-server tidak ketemu di folder aplikasi");
    let data = data_dir();

    let mut cmd = Command::new(&server);
    cmd.current_dir(&dir)
        .env("FLOWHIT_DATA", &data)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);
    let mut child = cmd.spawn().expect("gagal menjalankan flowhit-server");

    let ok = wait_port(Duration::from_secs(20));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("FLOWHIT PIXEL")
        .with_window_icon(load_icon())
        .with_inner_size(tao::dpi::LogicalSize::new(1280.0, 860.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(900.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let url = if ok {
        format!("http://127.0.0.1:{PORT}/")
    } else {
        "about:blank".to_string()
    };
    let _webview = WebViewBuilder::new()
        .with_url(url)
        .with_new_window_req_handler(|_url, _feat| NewWindowResponse::Deny)
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            let _ = child.kill();
            *control_flow = ControlFlow::Exit;
        }
    });
}
