use regex::Regex;

#[derive(Debug)]
pub struct ProfileName {
    value: String,
}

impl ProfileName {
    pub fn new(name: String) -> Result<Self, String> {
        let this = ProfileName { value: name };

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
        let result = Regex::new(r"^[[:word:]]{1,100}$")
            .unwrap()
            .is_match(&self.value);

        if result {
            Ok(())
        } else {
            Err("Profile name format is wrong".to_string()) // TODO: change to clear message
        }
    }
}

#[test]
fn new_returns_none_when_value_is_blank() {
    assert!(ProfileName::new("".to_string()).is_err())
}

#[test]
fn new_returns_none_when_value_contains_space() {
    assert!(ProfileName::new("hoge foo".to_string()).is_err())
}

#[test]
fn new_returns_some_value_when_name_is_correct() {
    assert!(ProfileName::new("hoge".to_string()).is_ok())
}
