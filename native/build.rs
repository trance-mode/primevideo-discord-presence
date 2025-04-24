// build.rs
fn main() {
    embed_resource::compile("native/icon.rc", embed_resource::NONE);
}