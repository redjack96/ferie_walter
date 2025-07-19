use axum::Json;
use crate::entity::dipendente::Dipendente;

pub async fn get_all() -> Json<Vec<Dipendente>> {
    Json(vec![Dipendente::default()])
}