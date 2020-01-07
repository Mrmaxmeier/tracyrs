use cc::Build;

fn main() {
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");
    Build::new()
        .cpp(true)
        .shared_flag(true)
        .pic(true)
        .define("TRACY_ENABLE", "1")
        .flag("-Wno-implicit-fallthrough")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-enum-compare")
        .flag("-Wno-sign-compare")
        .file("tracy/TracyClient.cpp")
        .compile("tracy");
}
