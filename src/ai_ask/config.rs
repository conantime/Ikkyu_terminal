struct Config {
    api_key: String,
    base_url: String,
}

pub fn get_config() -> Config {
    Config {
        api_key: "0d8bb39d2e3145a68e3c5dd0c07c723d".to_string(),
        base_url: "https://api.aimlapi.com".to_string(),
    }
}