extern crate awskey;
use awskey::services::profile_service::ProfileService;

fn main() {
    println!("Hello, world!");

    let profile = ProfileService::build(
        "name".to_string(),
        "serial_number".to_string(),
        "arn:aws:iam:123456789012:role/HOGE-fuga_bar".to_string(),
        "123456".to_string(),
    );

    println!("{:?}", profile);
}
