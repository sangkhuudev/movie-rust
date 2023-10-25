use actix_web::{get, web, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{PgPool,Executor};

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/version")]
async fn version(
    pool: web::Data<PgPool>,
) -> String {
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

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let pool = web::Data::new(pool);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(pool)
            .service(hello_world)
            .service(version);
    };

    Ok(config.into())
}
