use actix_web::{get, Responder};

#[get("/")]
pub async fn hola() -> impl Responder {
    "Oal itu. Beinvenido"
}

#[get("/about")]
pub async fn about() -> impl Responder {
    "¡¡¡Este sitio web es una API para marcadores programado en Actix Web!!!1"
}


pub async fn health() -> impl Responder {
    "La API está funcando"
}