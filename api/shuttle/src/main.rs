use actix_web::{web, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{PgPool,Executor};
use api_lib::{health, films};
use api_lib::film_repo::{PostgresFilmRepository, FilmRepository};

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let film_repository = PostgresFilmRepository::new(pool);
    let film_repository = web::Data::new(film_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(film_repository)
            .configure(health::service)
            .configure(films::service::<PostgresFilmRepository>);
    };

    Ok(config.into())
}
