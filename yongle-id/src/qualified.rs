//! Qualified identifier support.

use core::fmt;
use core::str::FromStr;

use semver::Version;
use thiserror::Error;
use unicode_ident::{is_xid_continue, is_xid_start};

/// A fully-qualified identifier composed of domain, version, and path.
///
/// The textual representation is `<domain>@<version>//<path>`, where domain
/// segments are dot-separated, path segments are slash-separated, and every
/// non-version segment is validated as a Unicode XID identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QualifiedName {
    domain_segments: Vec<String>,
    version: Version,
    path_segments: Vec<String>,
}

impl QualifiedName {
    /// Returns the validated domain segments of this identifier.
    pub fn domain_segments(&self) -> &[String] {
        &self.domain_segments
    }

    /// Returns the semantic version associated with this identifier.
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Returns the validated path segments of this identifier.
    pub fn path_segments(&self) -> &[String] {
        &self.path_segments
    }
}

impl fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}@{}//{}",
            self.domain_segments.join("."),
            self.version,
            self.path_segments.join("/")
        )
    }
}

impl FromStr for QualifiedName {
    type Err = QualifiedNameParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (domain, version_and_path) = value
            .split_once('@')
            .ok_or(QualifiedNameParseError::MissingVersionSeparator)?;
        let (version, path) = version_and_path
            .split_once("//")
            .ok_or(QualifiedNameParseError::MissingPathSeparator)?;

        let domain_segments = parse_domain_segments(domain)?;
        let version = Version::parse(version)
            .map_err(|_| QualifiedNameParseError::InvalidVersion(version.to_owned()))?;
        let path_segments = parse_path_segments(path)?;

        Ok(Self {
            domain_segments,
            version,
            path_segments,
        })
    }
}

/// Errors returned when parsing a [`QualifiedName`] from text.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum QualifiedNameParseError {
    /// The input does not contain the required `@` separator.
    #[error("qualified name must contain `@` between domain and version")]
    MissingVersionSeparator,
    /// The input does not contain the required `//` separator.
    #[error("qualified name must contain `//` between version and path")]
    MissingPathSeparator,
    /// The domain portion is empty.
    #[error("qualified name domain must not be empty")]
    EmptyDomain,
    /// The domain contains an empty segment.
    #[error("qualified name domain segments must not be empty")]
    EmptyDomainSegment,
    /// The version portion is not valid semantic version text.
    #[error("qualified name version `{0}` is not valid semver")]
    InvalidVersion(String),
    /// The path portion is empty.
    #[error("qualified name path must contain at least one segment")]
    EmptyPath,
    /// The path contains an empty segment, including trailing slash forms.
    #[error("qualified name path segments must not be empty")]
    EmptyPathSegment,
    /// A domain segment contains characters that are not valid XID identifier text.
    #[error("qualified name domain segment `{0}` is not a valid XID identifier")]
    InvalidDomainSegment(String),
    /// A path segment contains characters that are not valid XID identifier text.
    #[error("qualified name path segment `{0}` is not a valid XID identifier")]
    InvalidPathSegment(String),
}

fn parse_domain_segments(domain: &str) -> Result<Vec<String>, QualifiedNameParseError> {
    if domain.is_empty() {
        return Err(QualifiedNameParseError::EmptyDomain);
    }

    domain
        .split('.')
        .map(|segment| {
            if segment.is_empty() {
                return Err(QualifiedNameParseError::EmptyDomainSegment);
            }

            if !is_valid_identifier_part(segment) {
                return Err(QualifiedNameParseError::InvalidDomainSegment(
                    segment.to_owned(),
                ));
            }

            Ok(segment.to_owned())
        })
        .collect()
}

fn parse_path_segments(path: &str) -> Result<Vec<String>, QualifiedNameParseError> {
    if path.is_empty() {
        return Err(QualifiedNameParseError::EmptyPath);
    }

    path.split('/')
        .map(|segment| {
            if segment.is_empty() {
                return Err(QualifiedNameParseError::EmptyPathSegment);
            }

            if !is_valid_identifier_part(segment) {
                return Err(QualifiedNameParseError::InvalidPathSegment(
                    segment.to_owned(),
                ));
            }

            Ok(segment.to_owned())
        })
        .collect()
}

fn is_valid_identifier_part(value: &str) -> bool {
    let mut characters = value.chars();
    let Some(first) = characters.next() else {
        return false;
    };

    is_xid_start(first) && characters.all(is_xid_continue)
}

#[cfg(test)]
mod tests {
    use super::{QualifiedName, QualifiedNameParseError};
    use core::str::FromStr;

    #[test]
    fn parses_single_path_segment() {
        let name = QualifiedName::from_str("com.example@1.0.0//path").unwrap();

        assert_eq!(
            name.domain_segments(),
            ["com".to_owned(), "example".to_owned()]
        );
        assert_eq!(name.version().to_string(), "1.0.0");
        assert_eq!(name.path_segments(), ["path".to_owned()]);
        assert_eq!(name.to_string(), "com.example@1.0.0//path");
    }

    #[test]
    fn parses_multiple_path_segments() {
        let name = QualifiedName::from_str("dev.yongle@1.0.0//path/to/name").unwrap();

        assert_eq!(
            name.path_segments(),
            ["path".to_owned(), "to".to_owned(), "name".to_owned()]
        );
    }

    #[test]
    fn parses_unicode_xid_segments() {
        let name = QualifiedName::from_str("例子.项目@1.2.3//路径/名称").unwrap();

        assert_eq!(name.to_string(), "例子.项目@1.2.3//路径/名称");
    }

    #[test]
    fn round_trips_canonical_representation() {
        let source = "dev.yongle@2.3.4//path/to/name";
        let parsed = QualifiedName::from_str(source).unwrap();
        let reparsed = QualifiedName::from_str(&parsed.to_string()).unwrap();

        assert_eq!(parsed, reparsed);
    }

    #[test]
    fn rejects_non_semver_version() {
        let error = QualifiedName::from_str("dev.yongle@1//path").unwrap_err();

        assert_eq!(
            error,
            QualifiedNameParseError::InvalidVersion("1".to_owned())
        );
    }

    #[test]
    fn rejects_missing_domain() {
        let error = QualifiedName::from_str("@1.0.0//path").unwrap_err();

        assert_eq!(error, QualifiedNameParseError::EmptyDomain);
    }

    #[test]
    fn rejects_missing_version() {
        let error = QualifiedName::from_str("dev.yongle@//path").unwrap_err();

        assert_eq!(
            error,
            QualifiedNameParseError::InvalidVersion(String::new())
        );
    }

    #[test]
    fn rejects_missing_path() {
        let error = QualifiedName::from_str("dev.yongle@1.0.0").unwrap_err();

        assert_eq!(error, QualifiedNameParseError::MissingPathSeparator);
    }

    #[test]
    fn rejects_empty_domain_segment() {
        let error = QualifiedName::from_str("com..example@1.0.0//path").unwrap_err();

        assert_eq!(error, QualifiedNameParseError::EmptyDomainSegment);
    }

    #[test]
    fn rejects_invalid_domain_segment() {
        let error = QualifiedName::from_str("com.ex-ample@1.0.0//path").unwrap_err();

        assert_eq!(
            error,
            QualifiedNameParseError::InvalidDomainSegment("ex-ample".to_owned())
        );
    }

    #[test]
    fn rejects_invalid_path_segment() {
        let error = QualifiedName::from_str("com.example@1.0.0//path/to-name").unwrap_err();

        assert_eq!(
            error,
            QualifiedNameParseError::InvalidPathSegment("to-name".to_owned())
        );
    }

    #[test]
    fn rejects_empty_path_segment() {
        let error = QualifiedName::from_str("com.example@1.0.0//path//name").unwrap_err();

        assert_eq!(error, QualifiedNameParseError::EmptyPathSegment);
    }

    #[test]
    fn rejects_trailing_path_separator() {
        let error = QualifiedName::from_str("com.example@1.0.0//path/").unwrap_err();

        assert_eq!(error, QualifiedNameParseError::EmptyPathSegment);
    }

    #[test]
    fn rejects_empty_path() {
        let error = QualifiedName::from_str("com.example@1.0.0//").unwrap_err();

        assert_eq!(error, QualifiedNameParseError::EmptyPath);
    }
}
