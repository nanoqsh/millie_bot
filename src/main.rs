mod config;

use self::config::Config;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    let config = Config::load();

    let connection_string = config.pg.connection_string();
    let conn = Database::connect(&connection_string)
        .await
        .expect("connect");

    println!("{conn:?}");
}
