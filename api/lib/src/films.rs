use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/v1/films")
        .route("", web::get().to(get_all_films))
        .route("/{film_id}", web::get().to(get_film_by_id))
        .route("", web::post().to(post_film))
        .route("", web::put().to(update_film))
        .route("/{film_id}", web::delete().to(delete_film_by_id))
    );
}

async fn get_all_films() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn get_film_by_id() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn post_film() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn update_film() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn delete_film_by_id() -> HttpResponse {
    HttpResponse::Ok().finish()
}