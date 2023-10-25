use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use shared::models::{CreateFilm, Film};
use uuid::Uuid;
use crate::film_repo::FilmRepository;

pub fn service<R: FilmRepository>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
        .route("", web::get().to(get_all_films::<R>))
        .route("/{film_id}", web::get().to(get_film_by_id::<R>))
        .route("", web::post().to(post_film::<R>))
        .route("", web::put().to(update_film::<R>))
        .route("/{film_id}", web::delete().to(delete_film_by_id::<R>))
    );
}

async fn get_all_films<R: FilmRepository>(repo: web::Data<R>) -> HttpResponse {
    match repo.get_films().await {
        Ok(films) => HttpResponse::Ok().json(films),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn get_film_by_id<R: FilmRepository>(
    film_id: web::Path<Uuid>, 
    repo: web::Data<R>
) -> HttpResponse {
    match repo.get_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(_) => HttpResponse::NotFound().body("Not found"),
    }
}

async fn post_film<R: FilmRepository>(
    create_film: web::Json<CreateFilm>,
    repo: web::Data<R>,
) -> HttpResponse {
    match repo.create_film(&create_film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}

async fn update_film<R: FilmRepository>(
    film: web::Json<Film>, 
    repo: web::Data<R>
) -> HttpResponse {
    match repo.update_film(&film).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => HttpResponse::NotFound().body(format!("Internal server error: {:?}", e)),
    }
}

async fn delete_film_by_id<R: FilmRepository>(
    film_id: web::Path<Uuid>, 
    repo: web::Data<R>
) -> HttpResponse {
    match repo.delete_film(&film_id).await {
        Ok(film) => HttpResponse::Ok().json(film),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Internal server error: {:?}", e))
        }
    }
}