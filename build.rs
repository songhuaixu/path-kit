use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=pathkit/pathkit.h");
    println!("cargo:rerun-if-changed=pathkit/include");
    println!("cargo:rerun-if-changed=pathkit/src");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let pathkit = manifest_dir.join("pathkit");

    // 编译 pathkit C++ 库
    let pathkit_src = pathkit.join("src");
    let mut cpp_files: Vec<PathBuf> = glob::glob(pathkit_src.join("**/*.cpp").to_str().unwrap())
        .unwrap()
        .filter_map(|p| p.ok())
        .collect();
    cpp_files.retain(|p| p.exists() && p.file_name().unwrap() != "main.cpp");
    cpp_files.sort();

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .include(&pathkit)
        .define("PK_RELEASE", "1")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-unused-variable")
        // 抑制 pathkit C++ 库的其它 warning
        .flag("-Wno-unused-but-set-variable")   // SkCubicClipper.cpp: iters
        .flag("-Wno-unused-private-field")       // SkCubicClipper.h: fClip
        .flag("-Wno-array-parameter")           // SkGeometry/SkMatrix/SkPathOpsCubic: 数组形参不匹配
        .flag("-Wno-unused-function")            // SkPathBuilder.cpp: arc_is_lone_point 等
        .flag("-Wno-bitwise-instead-of-logical"); // SkPathEffect.cpp: | vs ||
    for f in &cpp_files {
        build.file(f);
    }
    build.compile("pathkit");

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

    // bindgen 直接从 C++ 头文件生成绑定
    let bindings = bindgen::Builder::default()
        .header(pathkit.join("pathkit.h").to_str().unwrap())
        .clang_args(["-x", "c++", "-std=c++17"])
        .clang_arg(format!("-I{}", pathkit.display()))
        .allowlist_type("pk::SkPath")
        .allowlist_type("pk::SkPath_Iter")
        .allowlist_type("pk::SkRect")
        .allowlist_type("pk::SkPathOp")
        .allowlist_type("pk::SkOpBuilder")
        .allowlist_type("pk::SkPathDirection")
        .allowlist_type("pk::SkScalar")
        .allowlist_type("pk::SkPathMeasure")
        .allowlist_type("pk::SkDashPathEffect")
        .allowlist_type("pk::SkCornerPathEffect")
        .allowlist_type("pk::SkRRect")
        .allowlist_function("Op")
        .allowlist_function("Simplify")
        .allowlist_function("SkPathEffect_filterPath")
        .allowlist_function("SkDashPathEffect_Make")
        .allowlist_function("SkCornerPathEffect_Make")
        .allowlist_type("pk::SkPathEffect")
        .allowlist_type("pk::SkPaint")
        .allowlist_type("pk::SkStrokeRec")
        .allowlist_file(".*pathops.*")
        .blocklist_type("std::.*")
        .opaque_type("std::.*")
        .constified_enum_module("pk::SkPathOp")
        .constified_enum_module("pk::SkPathFillType")
        .constified_enum_module("pk::SkPathDirection")
        .constified_enum_module("pk::SkPath_Verb")
        .constified_enum_module("pk::SkPath_SegmentMask")
        .constified_enum_module("pk::SkPath_AddPathMode")
        .constified_enum_module("pk::SkMatrix_TypeMask")
        .constified_enum_module("pk::SkMatrix_ScaleToFit")
        .constified_enum_module("pk::SkRRect_Type")
        .constified_enum_module("pk::SkRRect_Corner")
        .constified_enum_module("pk::SkStrokeRec_InitStyle")
        .constified_enum_module("pk::SkStrokeRec_Style")
        .constified_enum_module("pk::SkPathEffect_DashType")
        .constified_enum_module("pk::SkPaint_Cap")
        .constified_enum_module("pk::SkPaint_Join")
        .constified_enum_module("pk::SkPaint_Style")
        .constified_enum_module("pk::SkContourMeasure_MatrixFlags")
        .constified_enum_module("pk::SkPathMeasure_MatrixFlags")
        .enable_cxx_namespaces()
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs"))
        .expect("Couldn't write bindings");
}
