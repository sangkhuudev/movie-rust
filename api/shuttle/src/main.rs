use actix_web::{web, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{PgPool,Executor};
use api_lib::{health, films};
use api_lib::film_repo::PostgresFilmRepository;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres()] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let film_repository = PostgresFilmRepository::new(pool);
    let film_repository = web::Data::new(film_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
            .app_data(film_repository)
            .configure(health::service)
            .configure(films::service::<PostgresFilmRepository>)
        )
        .service(
            // Relative path to the workspace root ( docs of actix-files)
            actix_files::Files::new("/", "./api/shuttle/static") 
            .show_files_listing()
            .index_file("index.html")
        );
            
    };

    Ok(config.into())
}
