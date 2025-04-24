fn main() {
    println!("cargo:rerun-if-changed=icon.ico");
    println!("cargo:rerun-if-changed=icon.rc");
    embed_resource::compile("icon.rc", embed_resource::NONE);
}