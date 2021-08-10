use regex::Regex;

#[derive(Debug)]
pub struct SerialNumber {
    value: String,
}

impl SerialNumber {
    pub fn new(value: String) -> Result<Self, String> {
        let this = SerialNumber { value };

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
        let result = Regex::new(r"^[[:word:]:,\.@/-]{1,100}$")
            .unwrap()
            .is_match(&self.value);

        if result {
            Ok(())
        } else {
            Err("Serial number format is wrong".to_string()) // TODO: change to clear message
        }
    }
}

#[test]
fn new_returns_none_when_value_is_blank() {
    let serial_number = SerialNumber::new("".to_string());

    assert!(serial_number.is_err())
}

#[test]
fn new_validate_value_format() {
    assert!(SerialNumber::new("aA:,.@/-".to_string()).is_ok())
}
