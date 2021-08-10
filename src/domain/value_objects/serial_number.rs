use regex::Regex;

#[derive(Debug)]
pub struct SerialNumber {
    value: String,
}

impl SerialNumber {
    pub fn new(value: String) -> Option<Self> {
        let this = SerialNumber { value };

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
        Regex::new(r"^[[:word:]:,\.@/-]{1,100}$")
            .unwrap()
            .is_match(&self.value)
    }
}

#[test]
fn new_returns_none_when_value_is_blank() {
    let serial_number = SerialNumber::new("".to_string());

    assert!(serial_number.is_none())
}

#[test]
fn new_validate_value_format() {
    assert!(SerialNumber::new("aA:,.@/-".to_string()).is_some())
}
