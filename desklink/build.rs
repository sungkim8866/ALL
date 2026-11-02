// Embeds the icon and version info into DeskLink.exe when building on Windows.
fn main() {
    println!("cargo:rerun-if-changed=assets/icon.ico");
    let target_windows = std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows");
    #[cfg(windows)]
    if target_windows {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "DeskLink");
        res.set("FileDescription", "DeskLink - Mac/Windows mouse, keyboard, clipboard and file sharing");
        if let Err(e) = res.compile() {
            println!("cargo:warning=could not embed Windows icon: {e}");
        }
    }
    let _ = target_windows;
}
