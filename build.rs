use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=pathkit/pathkit.h");
    println!("cargo:rerun-if-changed=pathkit/include");
    println!("cargo:rerun-if-changed=pathkit/src");
    println!("cargo:rerun-if-changed=pathkit-bridge/include/pathkit_cxx_decl.h");
    println!("cargo:rerun-if-changed=pathkit-bridge/src/cxx_bridge.cpp");
    println!("cargo:rerun-if-changed=src/bridge.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let pathkit = manifest_dir.join("pathkit");
    let pathkit_bridge = manifest_dir.join("pathkit-bridge");

    let pathkit_src = pathkit.join("src");
    let mut cpp_files: Vec<PathBuf> = glob::glob(pathkit_src.join("**/*.cpp").to_str().unwrap())
        .unwrap()
        .filter_map(|p| p.ok())
        .collect();
    cpp_files.retain(|p| p.exists() && p.file_name().unwrap() != "main.cpp");
    cpp_files.sort();

    let mut cxx = cxx_build::bridge("src/bridge.rs");
    cxx.cpp(true)
        .std("c++17")
        .include(&pathkit)
        .include(&pathkit_bridge)
        .define("PK_RELEASE", "1")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        .flag("-Wno-unused-but-set-variable")
        .flag("-Wno-unused-private-field")
        .flag("-Wno-array-parameter")
        .flag("-Wno-unused-function")
        .flag("-Wno-bitwise-instead-of-logical")
        .file(pathkit_bridge.join("src/cxx_bridge.cpp"));
    for f in &cpp_files {
        cxx.file(f);
    }
    cxx.compile("pathkit_cxx_bridge");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    match (target_os.as_str(), target_env.as_str()) {
        ("linux", _) | ("windows", "gnu") | ("android", _) => {
            println!("cargo:rustc-link-lib=dylib=stdc++");
        }
        ("macos", _) | ("ios", _) => println!("cargo:rustc-link-lib=dylib=c++"),
        ("windows", "msvc") => {}
        _ => unimplemented!(
            "target_os: {}, target_env: {}",
            target_os.as_str(),
            target_env.as_str()
        ),
    }
}
