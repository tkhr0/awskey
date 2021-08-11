use crate::domain::value_objects::config_version::ConfigVersion;

#[derive(Debug)]
pub struct Config {
    version: ConfigVersion,
    profiles: Vec<Profile>,
}

impl Config {
    pub fn new(version: ConfigVersion, profiles: Vec<Profile>) -> Self {
        Config { version, profiles }
    }
}
