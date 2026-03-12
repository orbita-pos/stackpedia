use std::env;

pub struct Config {
    pub database_url: String,
    pub secret: Vec<u8>,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            secret: hex::decode(
                env::var("STACKPEDIA_SECRET").expect("STACKPEDIA_SECRET must be set"),
            )
            .expect("STACKPEDIA_SECRET must be valid hex"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "3000".into())
                .parse()
                .expect("PORT must be a number"),
        }
    }
}
