fn main() {
    let target = std::env::var("TARGET").expect("No TARGET environment variable defined");
    let is_windows = target.to_lowercase().contains("windows");
    let is_windows_gnu = target.to_lowercase().contains("windows-gnu");

    if is_windows {
        println!("cargo:rustc-link-lib=dylib=user32");
    }

    let mut build = cxx_build::bridge("src/lib.rs");

    if is_windows_gnu {
        build.define("OCC_CONVERT_SIGNALS", "TRUE");
    }

    if target.to_lowercase().contains("msvc") {
        build.flag("/EHsc");
    }

    if let "windows" = std::env::consts::OS {
        let current = std::env::current_dir().unwrap();
        build.include(current.parent().unwrap());
    }

    // HACK: Ultra-hack. This re-emits the link lines for cxx to link properly.
    // Can pass through cargo metadata?
    const OCCT_LIBS: &[&str] = &[
        "TKMath",
        "TKernel",
        "TKDE",
        "TKFeat",
        "TKGeomBase",
        "TKG2d",
        "TKG3d",
        "TKTopAlgo",
        "TKGeomAlgo",
        "TKBRep",
        "TKPrim",
        "TKDESTEP",
        "TKDEIGES",
        "TKDESTL",
        "TKMesh",
        "TKHLR",
        "TKShHealing",
        "TKFillet",
        "TKBool",
        "TKBO",
        "TKOffset",
        "TKXSBase",
        "TKCAF",
        "TKLCAF",
        "TKXCAF",
    ];

    for lib in OCCT_LIBS {
        println!("cargo::rustc-link-lib=static={}", lib);
    }

    build
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .define("_USE_MATH_DEFINES", "TRUE")
        .include(std::env::var("DEP_OCCT_INCLUDE").unwrap())
        .include("include")
        .compile("wrapper");

    println!("cargo:rustc-link-lib=static=wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/wrapper.hxx");
}
