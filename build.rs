use cc::Build;

fn main() {
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");
    Build::new()
        .cpp(true)
        .shared_flag(true)
        .pic(true)
        .define("TRACY_ENABLE", "1")
        .file("tracy/TracyClient.cpp")
        .compile("tracy");
}
