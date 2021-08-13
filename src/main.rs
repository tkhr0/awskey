extern crate awskey;
use awskey::domain::entities::config::Config;
use awskey::domain::value_objects::config_version::ConfigVersion;
use awskey::services::profile_service::ProfileService;

fn main() {
    println!("Hello, world!");

    let profile = ProfileService::build(
        "name".to_string(),
        "serial_number".to_string(),
        "arn:aws:iam:123456789012:role/HOGE-fuga_bar".to_string(),
        "123456".to_string(),
    )
    .unwrap();

    println!("{:?}", profile);

    let config = Config::new(ConfigVersion::parse("1.1").unwrap(), vec![profile]);

    println!("{:?}", config.serialize().unwrap());
}
