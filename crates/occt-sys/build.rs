use cmake;
use std::{env, path::PathBuf};

/// A type encapsulating a native build of OCCT.
///
/// This is used for structuring, validating, and forwarding native library info
/// on to other consumers of the underlying native package.
struct OcctSysBuild {}
impl OcctSysBuild {
    // The name of the link owned by this rust package. Should match the
    // `package.links` property of Cargo.toml
    const LINK_NAME: &str = "occt";
    const OCCT_DIR: &str = "OCCT";
    const PATCH_DIR: &str = "patch";
    // Paths in the package's `install_dir` at which to add headers, binaries, and
    // cmake files
    const INCLUDE_DIR: &str = "include";
    const LIB_DIR: &str = "lib";
    const CMAKE_DIR: &str = "cmake";
}

/// Emit metadata for this package to trigger re-runs on native, patch, and
/// build source changes.
fn emit_rerun_meta() {
    println!("cargo::rerun-if-changed={}", OcctSysBuild::OCCT_DIR);
    println!("cargo::rerun-if-changed={}", OcctSysBuild::PATCH_DIR);
    println!("cargo::rerun-if-changed=build.rs");

    // Re-run if the host c/cpp build vars change.
    for variable in [
        "CC",
        "CXX",
        "CFLAGS",
        "CXXFLAGS",
        "AR",
        "CMAKE",
        "CMAKE_GENERATOR",
        "CMAKE_GENERATOR_PLATFORM",
        "CMAKE_GENERATOR_TOOLSET",
        "CMAKE_PREFIX_PATH",
        "CMAKE_TOOLCHAIN_FILE",
        "HOST",
        "TARGET",
        "PROFILE",
    ] {
        println!("cargo::rerun-if-env-changed={variable}");
    }
}

fn main() {
    emit_rerun_meta();

    // Get this crate's path
    let manifest_dir = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not defined"),
    );

    // Get the path to the OCCT submodule
    let source_dir = manifest_dir.join(OcctSysBuild::OCCT_DIR);

    // Get the path to the patches directory
    let patch_dir = manifest_dir.join(OcctSysBuild::PATCH_DIR);

    // Get the output directory for this package (configured by cargo)
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not defined"));

    // Get a path at which to install native library files
    let install_dir = out_dir.join(OcctSysBuild::LINK_NAME);

    // Config::build returns the path at which the downstream links should resolve
    let cmake_dst = {
        // Configure cmake for building the native library
        cmake::Config::new(&source_dir)
            .define("BUILD_PATCH", &patch_dir)
            .define("BUILD_LIBRARY_TYPE", "Static")
            .define("BUILD_MODULE_ApplicationFramework", "FALSE")
            .define("BUILD_MODULE_Draw", "FALSE")
            .define("USE_D3D", "FALSE")
            .define("USE_DRACO", "FALSE")
            .define("USE_EIGEN", "FALSE")
            .define("USE_FFMPEG", "FALSE")
            .define("USE_FREEIMAGE", "FALSE")
            .define("USE_FREETYPE", "FALSE")
            .define("USE_GLES2", "FALSE")
            .define("USE_OPENGL", "FALSE")
            .define("USE_OPENVR", "FALSE")
            .define("USE_RAPIDJSON", "FALSE")
            .define("USE_TBB", "FALSE")
            .define("USE_TCL", "FALSE")
            .define("USE_TK", "FALSE")
            .define("USE_VTK", "FALSE")
            .define("USE_XLIB", "FALSE")
            .define("INSTALL_DIR_LIB", OcctSysBuild::LIB_DIR)
            .define("INSTALL_DIR_INCLUDE", OcctSysBuild::INCLUDE_DIR)
            .define("INSTALL_DIR_CMAKE", OcctSysBuild::CMAKE_DIR)
            // OCCT 7.8 declares CMake 3.1 compatibility, which CMake 4 no
            // longer enables unless a policy floor is supplied explicitly.
            .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
            .profile("Release")
            .out_dir(&install_dir)
            .build()
    };

    // Use the resulting path from cmake
    let out_include = cmake_dst.join(OcctSysBuild::INCLUDE_DIR);
    let out_lib = cmake_dst.join(OcctSysBuild::LIB_DIR);
    let out_cmake = cmake_dst.join(OcctSysBuild::CMAKE_DIR);

    // This metadata will be automatically turned into `DEP_OCCT_{LIB, INCLUDE,
    // CMAKE}` environment variables for any direct dependent which cmake can
    // pick up in the dependent's build script.
    println!("cargo::metadata=lib={}", out_lib.display());
    println!("cargo::metadata=include={}", out_include.display());
    println!("cargo::metadata=cmake={}", out_cmake.display());

    // Tell rust-lld where to link to the native libraries that we built.
    println!("cargo::rustc-link-search=native={}", out_lib.display());

    // HACK: This is stolen from the existing build scripts.
    // Really, we need to use the system archiving tools to link these properly

    // Every line here corresponds to a `{lib}.a` file that is built by cmake.
    // Linking these in the wrong order can cause linker errors down the line.
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
}
