use std::path::PathBuf;

#[cfg(any(not(feature = "builtin"), test))]
use std::collections::HashMap;

pub(crate) const REQUIRED_OCCT_VERSION: (u8, u8) = (7, 8);

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct OcctConfig {
    pub(crate) include_dir: PathBuf,
    pub(crate) library_dir: PathBuf,
    pub(crate) is_dynamic: bool,
}

#[cfg(any(not(feature = "builtin"), test))]
pub(crate) fn parse_detector_config(contents: &str) -> Result<(OcctConfig, (u8, u8)), String> {
    let values = parse_key_value_lines(contents);
    let version = parse_major_minor(&values, "VERSION_MAJOR", "VERSION_MINOR")?;
    let include_dir = required_path(&values, "INCLUDE_DIR")?;
    let library_dir = required_path(&values, "LIBRARY_DIR")?;
    let is_dynamic = match required_value(&values, "BUILD_SHARED_LIBS")? {
        "ON" => true,
        "OFF" => false,
        value => return Err(format!("BUILD_SHARED_LIBS must be ON or OFF, found {value:?}")),
    };

    Ok((OcctConfig { include_dir, library_dir, is_dynamic }, version))
}

#[cfg(any(feature = "builtin", test))]
pub(crate) fn parse_version(version: &str) -> Result<(u8, u8), String> {
    let mut components = version.split('.');
    let major = parse_version_component(components.next(), "major", version)?;
    let minor = parse_version_component(components.next(), "minor", version)?;
    Ok((major, minor))
}

pub(crate) fn validate_version(version: (u8, u8)) -> Result<(), String> {
    if version.0 == REQUIRED_OCCT_VERSION.0 && version.1 >= REQUIRED_OCCT_VERSION.1 {
        Ok(())
    } else {
        Err(format!(
            "found {}.{}, but {}.{} or a later compatible minor version is required",
            version.0, version.1, REQUIRED_OCCT_VERSION.0, REQUIRED_OCCT_VERSION.1
        ))
    }
}

#[cfg(any(not(feature = "builtin"), test))]
fn parse_key_value_lines(contents: &str) -> HashMap<&str, &str> {
    contents.lines().filter_map(|line| line.split_once('=')).collect()
}

#[cfg(any(not(feature = "builtin"), test))]
fn parse_major_minor(
    values: &HashMap<&str, &str>,
    major_key: &str,
    minor_key: &str,
) -> Result<(u8, u8), String> {
    let major = required_value(values, major_key)?
        .parse()
        .map_err(|_| format!("{major_key} must be an unsigned integer"))?;
    let minor = required_value(values, minor_key)?
        .parse()
        .map_err(|_| format!("{minor_key} must be an unsigned integer"))?;
    Ok((major, minor))
}

#[cfg(any(feature = "builtin", test))]
fn parse_version_component(
    component: Option<&str>,
    name: &str,
    full_version: &str,
) -> Result<u8, String> {
    component
        .ok_or_else(|| format!("OCCT version {full_version:?} has no {name} component"))?
        .parse()
        .map_err(|_| format!("OCCT version {full_version:?} has an invalid {name} component"))
}

#[cfg(any(not(feature = "builtin"), test))]
fn required_path(values: &HashMap<&str, &str>, key: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(required_value(values, key)?))
}

#[cfg(any(not(feature = "builtin"), test))]
fn required_value<'a>(values: &'a HashMap<&str, &str>, key: &str) -> Result<&'a str, String> {
    values
        .get(key)
        .copied()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("missing or empty {key}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_detector_output() {
        let input = "VERSION_MAJOR=7\nVERSION_MINOR=8\nINCLUDE_DIR=/occt/include\nLIBRARY_DIR=/occt/lib\nBUILD_SHARED_LIBS=OFF\n";

        let (config, version) = parse_detector_config(input).unwrap();

        assert_eq!(version, (7, 8));
        assert_eq!(config.include_dir, PathBuf::from("/occt/include"));
        assert_eq!(config.library_dir, PathBuf::from("/occt/lib"));
        assert!(!config.is_dynamic);
    }

    #[test]
    fn rejects_incomplete_detector_output() {
        let error = parse_detector_config("VERSION_MAJOR=7\nVERSION_MINOR=8\n").unwrap_err();
        assert_eq!(error, "missing or empty INCLUDE_DIR");
    }

    #[test]
    fn parses_metadata_version() {
        assert_eq!(parse_version("7.8.1").unwrap(), (7, 8));
        assert!(parse_version("7").is_err());
        assert!(parse_version("seven.8.1").is_err());
    }

    #[test]
    fn validates_compatible_versions() {
        assert!(validate_version((7, 8)).is_ok());
        assert!(validate_version((7, 9)).is_ok());
        assert!(validate_version((7, 7)).is_err());
        assert!(validate_version((8, 0)).is_err());
    }
}
