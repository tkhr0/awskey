use std::fmt;

#[derive(Debug)]
pub struct ConfigVersion {
    major: u8,
    minor: u8,
}

impl ConfigVersion {
    pub fn new(major: u8, minor: u8) -> Self {
        ConfigVersion { major, minor }
    }

    pub fn parse(version: &str) -> Result<ConfigVersion, String> {
        let splited: Vec<&str> = version.split(".").collect();

        if splited.len() != 2 {
            return Err("Version format is wrong".to_string());
        }

        let major = splited[0].parse::<u8>();
        let minor = splited[1].parse::<u8>();

        if major.is_ok() && minor.is_ok() {
            Ok(Self::new(major.unwrap(), minor.unwrap()))
        } else {
            Err("Version format is wrong".to_string())
        }
    }
}

#[test]
fn new() {
    assert_eq!(
        ConfigVersion::new(1, 2),
        ConfigVersion { major: 1, minor: 2 }
    )
}

#[test]
fn parse() {
    assert_eq!(
        ConfigVersion::parse("1.2").unwrap(),
        ConfigVersion { major: 1, minor: 2 }
    )
}

#[test]
fn parse_returns_err_when_version_contains_no_dots() {
    assert!(ConfigVersion::parse("1").is_err())
}

#[test]
fn parse_returns_err_when_version_contains_two_dots() {
    assert!(ConfigVersion::parse("1.1.1").is_err())
}

#[test]
fn parse_returns_err_when_version_contains_non_numeric_value() {
    assert!(ConfigVersion::parse("a.b").is_err())
}

impl fmt::Display for ConfigVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[test]
fn it_can_format() {
    let v = ConfigVersion::new(1, 1);
    assert_eq!(format!("{}", v), "1.1")
}

#[test]
fn it_can_convert_to_string() {
    let v = ConfigVersion::new(1, 1);
    assert_eq!(v.to_string(), "1.1".to_string())
}
