use crate::domain::entities::profile::Profile;
use crate::domain::value_objects::config_version::ConfigVersion;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    version: ConfigVersion,
    profiles: Vec<Profile>,
}

impl Config {
    pub fn new(version: ConfigVersion, profiles: Vec<Profile>) -> Self {
        Config { version, profiles }
    }

    pub fn serialize(&self) -> Result<Vec<u8>, String> {
        match toml::to_vec(&self) {
            Ok(data) => Ok(data),
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn deserialize(&mut self, content: Vec<u8>) -> Result<(), String> {
        match toml::from_slice(&content) {
            Ok(config) => {
                *self = config;
                Ok(())
            }
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
use crate::domain::value_objects::{
    profile_name::ProfileName, role_arn::RoleArn, secret_code::SecretCode,
    serial_number::SerialNumber,
};

#[test]
fn serialize() {
    let profile = Profile::new(
        ProfileName::new("test".to_string()).unwrap(),
        SerialNumber::new("secret".to_string()).unwrap(),
        RoleArn::new("arn:aws:iam:123456789012:role/HOGE-fuga_bar".to_string()).unwrap(),
        SecretCode::new("123456".to_string()).unwrap(),
    );

    let config = Config {
        profiles: vec![profile],
        version: ConfigVersion::parse("1.1").unwrap(),
    };
    let expected = r#"[version]
major = 1
minor = 1

[[profiles]]
[profiles.name]
value = "test"

[profiles.serial_number]
value = "secret"

[profiles.role_arn]
value = "arn:aws:iam:123456789012:role/HOGE-fuga_bar"

[profiles.secret_code]
value = "123456"
"#;

    assert_eq!(
        String::from_utf8(config.serialize().unwrap()).unwrap(),
        expected
    );
}
