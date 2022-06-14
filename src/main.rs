use sea_orm::{Database, DatabaseConnection};

#[tokio::main]
async fn main() {
    let conn: DatabaseConnection = Database::connect("postgres://millie:millie@0.0.0.0/millie")
        .await
        .expect("connect");

    println!("{conn:?}");
}
