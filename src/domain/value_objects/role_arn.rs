use regex::Regex;

#[derive(Debug)]
pub struct RoleArn{
    value: String
}

impl RoleArn {
    pub fn new(value: String) -> Option<Self> {
        let this = RoleArn { value };

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
        Regex::new(r"^arn:aws:iam:[[:digit:]]{12}:role/[[:word:]-]+$")
            .unwrap()
            .is_match(&self.value)
    }
}

#[test]
fn new_validate_value_format() {
    assert!(RoleArn::new("arn:aws:iam:123456789012:role/HOGE-fuga_bar".to_string()).is_some())
}
