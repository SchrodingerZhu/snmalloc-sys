use cmake::Config;
fn main() {
    let mut cfg = &mut Config::new("snmalloc");
    if cfg!(all(windows, target_env = "msvc")) {
        cfg = cfg.generator("Visual Studio 15 2017 Win64")
            .build_arg("--config")
            .build_arg("Release")
    } else {
        cfg = cfg.generator("Ninja")
            .define("CMAKE_BUILD_TYPE", "Release")
    }
    let target = if cfg!(feature = "1mib") {
        "snmallocshim-1mib"
    } else {
        "snmallocshim"
    };
    let mut dst = cfg.build_target(target).build();
    dst.push("./build");
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib={}", target);
    if cfg!(unix) {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=atomic");
    }
}