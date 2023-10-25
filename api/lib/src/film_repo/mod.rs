use shared::models::{Film, CreateFilm};
use uuid::Uuid;

mod postgres_film_repo;
pub use postgres_film_repo::PostgresFilmRepository;
pub type FilmError = String;
pub type FilmResult<T> = Result<T, FilmError>;

#[async_trait::async_trait]
pub trait FilmRepository: Send + Sync + 'static {
    async fn get_films(&self) -> FilmResult<Vec<Film>>;
    async fn get_film(&self, film_id: &Uuid) -> FilmResult<Film>;
    async fn create_film(&self, create_film: &CreateFilm) -> FilmResult<Film>;
    async fn update_film(&self, film: &Film) -> FilmResult<Film>;
    async fn delete_film(&self, film_id: &Uuid) -> FilmResult<Uuid>;
}