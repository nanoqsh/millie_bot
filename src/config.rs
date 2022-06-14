use serde::Deserialize;

#[derive(Deserialize)]
pub struct Pg {
    user: String,
    password: String,
    host: String,
    db: String,
}

impl Pg {
    pub fn connection_string(&self) -> String {
        let Self {
            user,
            password,
            host,
            db,
        } = self;

        format!("postgres://{user}:{password}@{host}/{db}")
    }
}

#[derive(Deserialize)]
pub struct Config {
    pub pg: Pg,
}

impl Config {
    pub fn load() -> Self {
        let content = std::fs::read_to_string("./Bot.toml").expect("read config");
        toml::from_str(&content).expect("read toml")
    }
}
