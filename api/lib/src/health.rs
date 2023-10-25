use actix_web::{get, web};
use sqlx::PgPool;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(
    pool: web::Data<PgPool>,
) -> String {
    tracing::info!("Getting version");
    let result: Result<String, sqlx::Error> = sqlx::query_scalar(
        "SELECT version()"
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(version) => version,
        Err(e) => format!("Error: {:?}", e) 
    }
}
