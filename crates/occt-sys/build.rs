use std::{
    env, fs,
    path::{Path, PathBuf},
};

const INCLUDE_DIR: &str = "include";
const LIB_DIR: &str = "lib";
const CMAKE_DIR: &str = "lib/cmake/opencascade";
const OCCT_VERSION: &str = "7.8.1";

fn main() {
    emit_rerun_directives();

    let manifest_dir = PathBuf::from(
        env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not defined"),
    );
    let source_dir = manifest_dir.join("OCCT");
    let patch_dir = manifest_dir.join("patch");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is not defined"));
    let install_dir = out_dir.join("occt");

    let root = cmake::Config::new(&source_dir)
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
        .define("INSTALL_DIR_LIB", LIB_DIR)
        .define("INSTALL_DIR_INCLUDE", INCLUDE_DIR)
        .define("INSTALL_DIR_CMAKE", CMAKE_DIR)
        // OCCT 7.8 declares CMake 3.1 compatibility, which CMake 4 no
        // longer enables unless a policy floor is supplied explicitly.
        .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5")
        .profile("Release")
        .out_dir(&install_dir)
        .build();

    // `cmake::Config::build` emits `cargo:root` for compatibility with its
    // `register_dep` facility. The explicit contract below emits the same root
    // plus the additional paths and build properties consumers need.
    let artifact = OcctArtifact::validate(root);
    artifact.emit_metadata();
}

struct OcctArtifact {
    root: PathBuf,
    include: PathBuf,
    lib: PathBuf,
    cmake: PathBuf,
    version: String,
}

impl OcctArtifact {
    fn validate(root: PathBuf) -> Self {
        let include = root.join(INCLUDE_DIR);
        let header = include.join("Standard.hxx");
        assert_path_is_file(&header, "installed OCCT header");

        let lib = root.join(LIB_DIR);
        assert_path_is_dir(&lib, "installed OCCT library directory");
        let has_libraries = fs::read_dir(&lib)
            .unwrap_or_else(|error| {
                panic!("failed to read OCCT library directory {}: {error}", lib.display())
            })
            .filter_map(Result::ok)
            .any(|entry| entry.file_type().map(|kind| kind.is_file()).unwrap_or(false));
        assert!(has_libraries, "OCCT library directory {} is empty", lib.display());

        let cmake = root.join(CMAKE_DIR);
        assert_path_is_dir(&cmake, "installed OCCT CMake package directory");
        let version_file = cmake.join("OpenCASCADEConfigVersion.cmake");
        assert_path_is_file(
            &cmake.join("OpenCASCADEConfig.cmake"),
            "installed OCCT CMake package config",
        );
        assert_path_is_file(&version_file, "installed OCCT CMake package version config");

        let version = read_package_version(&version_file);
        assert_eq!(
            version, OCCT_VERSION,
            "bundled OCCT version mismatch: expected {OCCT_VERSION}, found {version}"
        );

        Self { root, include, lib, cmake, version }
    }

    fn emit_metadata(&self) {
        // Because this package declares `links = "occt"`, Cargo exposes these
        // values to immediate dependent build scripts as DEP_OCCT_<KEY>.
        emit_metadata("root", &self.root.display().to_string());
        emit_metadata("include", &self.include.display().to_string());
        emit_metadata("lib", &self.lib.display().to_string());
        emit_metadata("cmake", &self.cmake.display().to_string());
        emit_metadata("version", &self.version);
        emit_metadata("link_kind", "static");
    }
}

fn read_package_version(path: &Path) -> String {
    let contents = fs::read_to_string(path).unwrap_or_else(|error| {
        panic!("failed to read OCCT version config {}: {error}", path.display())
    });

    contents
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix("set(PACKAGE_VERSION \"")
                .and_then(|version| version.strip_suffix("\")"))
        })
        .map(str::to_owned)
        .unwrap_or_else(|| panic!("could not determine OCCT version from {}", path.display()))
}

fn assert_path_is_file(path: &Path, description: &str) {
    assert!(path.is_file(), "{description} is missing: {}", path.display());
}

fn assert_path_is_dir(path: &Path, description: &str) {
    assert!(path.is_dir(), "{description} is missing: {}", path.display());
}

fn emit_metadata(key: &str, value: &str) {
    // The single-colon form supports Cargo versions older than 1.77 while still
    // being interpreted as `links` metadata.
    println!("cargo:{key}={value}");
}

fn emit_rerun_directives() {
    println!("cargo:rerun-if-changed=OCCT");
    println!("cargo:rerun-if-changed=patch");
    println!("cargo:rerun-if-changed=build.rs");

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
        println!("cargo:rerun-if-env-changed={variable}");
    }
}
