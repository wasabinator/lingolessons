#[cfg(windows)]
fn main() {
    println!("Building Resources");
    let mut res = winresource::WindowsResource::new();
    res.set_icon("res/appicon.ico");
    res.compile().unwrap();
}
