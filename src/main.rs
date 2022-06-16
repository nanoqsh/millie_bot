mod config;

use self::config::Config;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let config = Config::load();
    let connection_string = config.pg.connection_string();
    let conn = Database::connect(&connection_string)
        .await
        .expect("connect");

    println!("{conn:?}");
}
