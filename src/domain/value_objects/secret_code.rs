use regex::Regex;

#[derive(Debug)]
pub struct SecretCode {
    value: String,
}

impl SecretCode {
    pub fn new(value: String) -> Result<Self, String> {
        let this = SecretCode { value };

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
        let result = Regex::new(r"^[[:digit:]]{6}$")
            .unwrap()
            .is_match(&self.value);

        if result {
            Ok(())
        } else {
            Err("Secret Code format is wrong".to_string()) // TODO: change to clear message
        }
    }
}

#[test]
fn new_returns_some_when_value_is_7_digits() {
    assert!(SecretCode::new("1234567".to_string()).is_err())
}

#[test]
fn new_returns_some_when_value_is_6_digits() {
    assert!(SecretCode::new("123456".to_string()).is_ok())
}

#[test]
fn new_returns_some_when_value_is_5_digits() {
    assert!(SecretCode::new("12345".to_string()).is_err())
}
