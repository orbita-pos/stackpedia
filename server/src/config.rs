use std::env;

pub struct Config {
    pub database_url: String,
    pub secret: Vec<u8>,
    pub port: u16,
    pub rust_env: String,
    pub cors_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Self {
        let cors_origins = env::var("CORS_ORIGINS")
            .map(|v| v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect())
            .unwrap_or_else(|_| vec![
                "http://localhost:5173".into(),
                "http://localhost:3000".into(),
                "http://localhost:3001".into(),
            ]);

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
            rust_env: env::var("RUST_ENV").unwrap_or_else(|_| "development".into()),
            cors_origins,
        }
    }

    pub fn is_production(&self) -> bool {
        self.rust_env == "production"
    }
}
