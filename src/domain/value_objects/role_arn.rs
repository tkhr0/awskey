use regex::Regex;

#[derive(Debug)]
pub struct RoleArn {
    value: String,
}

impl RoleArn {
    pub fn new(value: String) -> Result<Self, String> {
        let this = RoleArn { value };

        match this.validate() {
            Ok(_) => Ok(this),
            Err(message) => Err(message),
        }
    }

    fn validate(&self) -> Result<(), String> {
        self.validate_value_format()?;

        Ok(())
    }

    fn validate_value_format(&self) -> Result<(), String> {
        let result = Regex::new(r"^arn:aws:iam:[[:digit:]]{12}:role/[[:word:]-]+$")
            .unwrap()
            .is_match(&self.value);

        if result {
            Ok(())
        } else {
            Err("Role arn format is wrong".to_string()) // TODO: change to clear message
        }
    }
}

#[test]
fn new_validate_value_format() {
    let arn = RoleArn::new("arn:aws:iam:123456789012:role/HOGE-fuga_bar".to_string());
    assert!(arn.is_ok())
}
