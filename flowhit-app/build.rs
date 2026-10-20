fn main() {
    let mut res = winresource::WindowsResource::new();
    res.set_icon("icon.ico");
    if let Err(e) = res.compile() {
        eprintln!("icon resource gagal: {e}");
    }
}
