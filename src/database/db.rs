use dotenv::dotenv;
use sqlx::{ postgres::PgPoolOptions, Pool, Postgres };

pub struct AppState {
    pub db: Pool<Postgres>,
}

pub async fn connect() -> Pool<Postgres> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match PgPoolOptions::new().max_connections(10).connect(&database_url).await {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}

pub async fn migrate(pool: &Pool<Postgres>) {
    // prettier-ignore
    sqlx::migrate!("./migrations").run(pool).await.expect("Failed to migrate the database");
}
