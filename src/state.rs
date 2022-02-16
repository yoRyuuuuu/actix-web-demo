use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct State {
    pub pool: Pool<Postgres>,
}

pub type AppStateRaw = std::sync::Arc<State>;
pub type AppState = actix_web::web::Data<AppStateRaw>;
