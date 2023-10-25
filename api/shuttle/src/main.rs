use actix_web::{web, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{PgPool,Executor};
use api_lib::health;

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
            .configure(health::service);
    };

    Ok(config.into())
}
