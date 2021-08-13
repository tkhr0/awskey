use crate::domain::value_objects::{
    profile_name::ProfileName, role_arn::RoleArn, secret_code::SecretCode,
    serial_number::SerialNumber,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    name: ProfileName,
    serial_number: SerialNumber,
    role_arn: RoleArn,
    secret_code: SecretCode,
}

impl Profile {
    pub fn new(
        name: ProfileName,
        serial_number: SerialNumber,
        role_arn: RoleArn,
        secret_code: SecretCode,
    ) -> Self {
        Profile {
            name,
            serial_number,
            role_arn,
            secret_code,
        }
    }
}
