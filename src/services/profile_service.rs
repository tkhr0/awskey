use crate::domain::entities::profile::Profile;
use crate::domain::value_objects::{
    profile_name::ProfileName, role_arn::RoleArn, secret_code::SecretCode,
    serial_number::SerialNumber,
};

pub struct ProfileService {}

impl ProfileService {
    pub fn build(
        name: String,
        serial_number: String,
        role_arn: String,
        secret_code: String,
    ) -> Result<Profile, String> {
        let profile = Profile::new(
            ProfileName::new(name)?,
            SerialNumber::new(serial_number)?,
            RoleArn::new(role_arn)?,
            SecretCode::new(secret_code)?,
        );

        Ok(profile)
    }
}
