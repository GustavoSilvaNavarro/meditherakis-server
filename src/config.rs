use dotenv::dotenv;
use env_logger;
use std::env;

pub struct Config {
    pub port: i32,
    pub host: String,
}

pub fn read_config() -> Config {
    dotenv().ok();
    env_logger::init();

    let config = Config {
        port: env::var("PORT")
            .expect("PORT has not been defined. Defined!")
            .parse()
            .expect("Bad PORT definition."),
        host: env::var("APP_HOST").expect("APP_HOST has not been defined. Defined!"),
    };

    return config;
}
