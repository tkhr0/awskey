use regex::Regex;

#[derive(Debug)]
pub struct SecretCode {
    value: String,
}

impl SecretCode {
    pub fn new(value: String) -> Option<Self> {
        let this = SecretCode { value };

        if this.validate() {
            Some(this)
        } else {
            None
        }
    }

    fn validate(&self) -> bool {
        self.validate_value_format()
    }

    fn validate_value_format(&self) -> bool {
        Regex::new(r"^[[:digit:]]{6}$")
            .unwrap()
            .is_match(&self.value)
    }
}

#[test]
fn new_returns_some_when_value_is_7_digits() {
    assert!(SecretCode::new("1234567".to_string()).is_none())
}

#[test]
fn new_returns_some_when_value_is_6_digits() {
    assert!(SecretCode::new("123456".to_string()).is_some())
}

#[test]
fn new_returns_some_when_value_is_5_digits() {
    assert!(SecretCode::new("12345".to_string()).is_none())
}
