use regex::Regex;

#[derive(Debug)]
pub struct ProfileName {
    value: String,
}

impl ProfileName {
    pub fn new(name: String) -> Option<Self> {
        let this = ProfileName { value: name };

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
        Regex::new(r"^[[:word:]]{1,100}$")
            .unwrap()
            .is_match(&self.value)
    }
}

#[test]
fn new_returns_none_when_value_is_blank() {
    assert!(ProfileName::new("".to_string()).is_none())
}

#[test]
fn new_returns_none_when_value_contains_space() {
    assert!(ProfileName::new("hoge foo".to_string()).is_none())
}

#[test]
fn new_returns_some_value_when_name_is_correct() {
    assert!(ProfileName::new("hoge".to_string()).is_some())
}
