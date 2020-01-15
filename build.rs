use cc::Build;

fn main() {
    if cfg!(not(feature = "tracy_enable")) {
        println!("cargo:warning=tracyrs: Tracy instrumentation is not enabled.");
        return;
    }

    if cfg!(feature = "tracy_enable") {
        println!("cargo:warning=tracyrs: Tracy instrumentation is enabled.");
    }

    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");

    let mut build = Build::new();
    build.cpp(true);
    build.shared_flag(true);
    build.pic(true);

    if cfg!(feature = "tracy_enable") {
        build.define("TRACY_ENABLE", "1");
    }

    #[cfg(feature = "tracy_on_demand")]
    build.define("TRACY_ON_DEMAND", "1");

    #[cfg(feature = "tracy_no_exit")]
    build.define("TRACY_NO_EXIT", "1");

    #[cfg(feature = "tracy_no_broadcast")]
    build.define("TRACY_NO_BROADCAST", "1");

    build
        .flag("-Wno-implicit-fallthrough")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-enum-compare")
        .flag("-Wno-sign-compare")
        .file("tracy/TracyClient.cpp")
        .compile("tracy");
}
