#[cfg(windows)]
fn main() {
    println!("Building Resources");
    embed_resource::compile("resource.rc", embed_resource::NONE);
}
