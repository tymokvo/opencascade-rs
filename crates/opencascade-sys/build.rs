mod build_support;

use std::env;

#[cfg(not(feature = "builtin"))]
use std::fs;
#[cfg(feature = "builtin")]
use std::path::PathBuf;

#[cfg(not(feature = "builtin"))]
use build_support::parse_detector_config;
#[cfg(feature = "builtin")]
use build_support::parse_version;
use build_support::{validate_version, OcctConfig};

/// The list of used OpenCASCADE libraries which needs to be linked with.
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

fn main() {
    let target = env::var("TARGET").expect("No TARGET environment variable defined");
    let is_windows = target.to_lowercase().contains("windows");
    let is_windows_gnu = target.to_lowercase().contains("windows-gnu");

    let occt_config = OcctConfig::detect();

    println!("cargo:rustc-link-search=native={}", occt_config.library_dir.to_str().unwrap());

    let lib_type = if occt_config.is_dynamic { "dylib" } else { "static" };
    for lib in OCCT_LIBS {
        println!("cargo:rustc-link-lib={lib_type}={lib}");
    }

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

    if let "windows" = env::consts::OS {
        let current = env::current_dir().unwrap();
        build.include(current.parent().unwrap());
    }

    build
        .cpp(true)
        .flag_if_supported("-std=c++11")
        .define("_USE_MATH_DEFINES", "TRUE")
        .include(occt_config.include_dir)
        .include("include")
        .compile("wrapper");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/wrapper.hxx");
}

impl OcctConfig {
    fn detect() -> Self {
        #[cfg(feature = "builtin")]
        {
            Self::from_builtin_metadata()
        }

        #[cfg(not(feature = "builtin"))]
        {
            Self::from_preinstalled_package()
        }
    }

    #[cfg(feature = "builtin")]
    fn from_builtin_metadata() -> Self {
        for key in ["ROOT", "INCLUDE", "LIB", "CMAKE", "VERSION", "LINK_KIND"] {
            println!("cargo:rerun-if-env-changed=DEP_OCCT_{key}");
        }

        let root = required_metadata_path("ROOT");
        let include_dir = required_metadata_path("INCLUDE");
        let library_dir = required_metadata_path("LIB");
        let cmake_dir = required_metadata_path("CMAKE");
        let version_text = required_metadata("VERSION");
        let link_kind = required_metadata("LINK_KIND");

        require_directory(&root, "DEP_OCCT_ROOT");
        require_directory(&include_dir, "DEP_OCCT_INCLUDE");
        require_directory(&library_dir, "DEP_OCCT_LIB");
        require_directory(&cmake_dir, "DEP_OCCT_CMAKE");
        require_file(&include_dir.join("Standard.hxx"), "bundled OCCT header");
        require_file(&cmake_dir.join("OpenCASCADEConfig.cmake"), "bundled OCCT CMake config");

        for (name, path) in [("DEP_OCCT_INCLUDE", &include_dir), ("DEP_OCCT_LIB", &library_dir)] {
            assert!(
                path.starts_with(&root),
                "{name} ({}) must be inside DEP_OCCT_ROOT ({})",
                path.display(),
                root.display()
            );
        }

        let version = parse_version(&version_text)
            .unwrap_or_else(|error| panic!("invalid DEP_OCCT_VERSION: {error}"));
        validate_version(version)
            .unwrap_or_else(|error| panic!("Builtin OpenCASCADE version is incompatible: {error}. Please fix the OCCT version requirement in `opencascade-sys` or the bundled OCCT revision in `occt-sys`."));

        let is_dynamic = match link_kind.as_str() {
            "static" => false,
            "dylib" => true,
            _ => panic!("DEP_OCCT_LINK_KIND must be `static` or `dylib`, found {link_kind:?}"),
        };

        Self { include_dir, library_dir, is_dynamic }
    }

    #[cfg(not(feature = "builtin"))]
    fn from_preinstalled_package() -> Self {
        println!("cargo:rerun-if-env-changed=DEP_OCCT_ROOT");

        let dst = std::panic::catch_unwind(|| cmake::Config::new("OCCT").register_dep("occt").build())
            .expect("Pre-installed OpenCASCADE library not found. You can use `builtin` feature if you do not want to install OCCT libraries system-wide.");

        let config_path = dst.join("share").join("occ_info.txt");
        let contents = fs::read_to_string(&config_path).unwrap_or_else(|error| {
            panic!("failed to read OpenCASCADE detector output {}: {error}", config_path.display())
        });
        let (config, version) = parse_detector_config(&contents).unwrap_or_else(|error| {
            panic!("invalid OpenCASCADE detector output {}: {error}", config_path.display())
        });
        validate_version(version).unwrap_or_else(|error| {
            panic!("Pre-installed OpenCASCADE version is incompatible: {error}. Please provide the required version or use the `builtin` feature.")
        });
        config
    }
}

#[cfg(feature = "builtin")]
fn required_metadata(key: &str) -> String {
    let variable = format!("DEP_OCCT_{key}");
    env::var(&variable).unwrap_or_else(|_| {
        panic!("missing {variable}; the `occt-sys` artifact contract is incomplete")
    })
}

#[cfg(feature = "builtin")]
fn required_metadata_path(key: &str) -> PathBuf {
    PathBuf::from(required_metadata(key))
}

#[cfg(feature = "builtin")]
fn require_directory(path: &std::path::Path, name: &str) {
    assert!(path.is_dir(), "{name} is not a directory: {}", path.display());
}

#[cfg(feature = "builtin")]
fn require_file(path: &std::path::Path, name: &str) {
    assert!(path.is_file(), "{name} is missing: {}", path.display());
}
