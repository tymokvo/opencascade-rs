use cmake;
use std::{env, path::PathBuf};

/// A type encapsulating a native build of OCCT.
///
/// This is used for structuring, validating, and forwarding native library info
/// on to other consumers of the underlying native package.
struct OcctSysBuild {}
impl OcctSysBuild {
    // Paths in the package's `install_dir` at which to add headers, binaries, and
    // cmake files
    const INCLUDE_DIR: &str = "include";
    const LIB_DIR: &str = "lib";
    const CMAKE_DIR: &str = "lib/cmake/opencascade";
}

fn main() {
    // Get this crate's path
    let manifest_dir = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not defined"),
    );

    // Get the path to the OCCT submodule
    let source_dir = manifest_dir.join("OCCT");

    // Get the path to the patches directory
    let patch_dir = manifest_dir.join("patch");

    // Get the output directory for this package (configured by cargo)
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not defined"));

    // Get a path at which to install native library files
    let install_dir = out_dir.join("occt");

    let config = {
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
}
