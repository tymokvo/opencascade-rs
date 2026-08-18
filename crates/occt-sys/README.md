# occt-sys

`occt-sys` owns the bundled Open CASCADE Technology (OCCT) native build. Its
Cargo build script installs a static Release build beneath this package's
`OUT_DIR`; consumers must not invoke another OCCT build or infer the location
from Cargo's target-directory layout.

## Cargo artifact metadata contract

The package declares `links = "occt"` and emits the following metadata for the
build scripts of its **immediate Cargo dependents**:

| Environment variable | Meaning |
| --- | --- |
| `DEP_OCCT_ROOT` | Root of the installed OCCT tree |
| `DEP_OCCT_INCLUDE` | Installed headers directory |
| `DEP_OCCT_LIB` | Installed native libraries directory |
| `DEP_OCCT_CMAKE` | Directory containing `OpenCASCADEConfig.cmake` |
| `DEP_OCCT_VERSION` | Bundled OCCT version, such as `7.8.1` |
| `DEP_OCCT_LINK_KIND` | Cargo link kind; currently `static` |

These variables are a build-script-only contract. Cargo does not propagate
`DEP_OCCT_*` metadata through transitive dependencies, and application/runtime
Rust code should not depend on the paths because `cargo clean` may remove them.
A dependent build script should fail fast if required metadata or artifacts are
missing rather than searching the host system when bundled OCCT was requested.
