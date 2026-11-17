use actix_web::{ get, post, put, delete, HttpResponse, Responder, web };

use crate::models::{Bookmark, NewBookmark, TagFilter};
use crate::state::AppState;

#[get("/bookmarks")]
pub async fn list_bookmarks(
    data: web::Data<AppState>,
    query: web::Query<TagFilter>
) -> impl Responder {

    // De data vas a buscar bookmarks, luego lo bloqueas con la funcion lock() de mutex y lo desenvuelves.
    // Aquí sí se debe usar unwrap por cuestion de lógica: Si falla el lock entonces la petición debe crashear para evitar más errores.
    // lock() falla si otro hilo entra en pánico mientras mantenía el bloqueo
    let bookmarks = data.bookmarks.lock().unwrap();

    let filtered: Vec<Bookmark> = bookmarks
        .iter()
        .filter(|b| match &query.tag {
            Some(tag) => &b.tag == tag,
            None => true
        })
        .cloned()
        .collect();
    
    // Se convierte a JSON para poder ser enviado
    web::Json(filtered)

    // El bloqueo se va cuando bookmarks sale del alcance de la función
}

#[get("/bookmarks/{id}")]
pub async fn get_bookmark(
    data: web::Data<AppState>,
    path: web::Path<u32>
) -> impl Responder {
    // into_inner() hace unwrap de Path y nos deja solo con el u32 pelón
    let id = path.into_inner();
    let bookmarks = data.bookmarks.lock().unwrap();

    match bookmarks.iter().find( |b| b.id == id ) {
        Some(bookmark) => HttpResponse::Ok().json(bookmark),
        None => HttpResponse::NotFound().body("No se encontró el marcador") // Si el usuario no puso tag de filter regresa toda la lista
    }

}

#[post("/bookmarks")]
pub async fn create_bookmark(
    data: web::Data<AppState>,
    body: web::Json<NewBookmark>
) -> impl Responder {

    // Recordar que into_inner() permite que accedamos directamente al contenido
    let new = body.into_inner();
    let mut bookmarks = data.bookmarks.lock().unwrap();

    // Convertimos bookmarks a iterable, y luego hacemos una operacion de mapeo para obtener todos los id, buscamos el maximo y le sumamos 1.
    // Si no existen bookmarks no habrá id, por lo que se establece por defecto en 0 y luego le sumamos 1 para que el primero sea id: 1.
    let id = bookmarks.iter().map(|b| b.id)
        .max().unwrap_or(0) + 1;

    let bookmark = Bookmark {
        id,
        title: new.title,
        url: new.url,
        tag: new.tag
    };

    bookmarks.push(bookmark.clone());

    HttpResponse::Created().json(bookmark)

}

#[put("/bookmarks/{id}")]
pub async fn update_bookmark(
    data: web::Data<AppState>,
    path: web::Path<u32>,
    body: web::Json<NewBookmark>
) -> impl Responder {

    let id = path.into_inner();
    let changes = body.into_inner();

    let mut bookmarks = data.bookmarks.lock().unwrap();

    match bookmarks.iter_mut().find(|b| b.id == id) {
        Some(bookmark) => {
            bookmark.title = changes.title;
            bookmark.url = changes.url;
            bookmark.tag = changes.tag;
            // Respondemos con Ok y un json de los cambios
            HttpResponse::Ok().json(bookmark)
        }
        None => HttpResponse::NotFound().body("No se encontró el marcador")
    }

}

#[delete("/bookmarks/{id}")]
pub async fn delete_bookmark(
    data: web::Data<AppState>,
    path: web::Path<u32>,
) -> impl Responder {

    let id = path.into_inner();
    let mut bookmarks = data.bookmarks.lock().unwrap();

    // position() encuentra el indice en el cual existen coincidencias con los criterios de búsqueda.
    // No necesitamos iter_mut() porque no vamos a alterar los elementos del iterable; no así el caso de bookmarks en si mismo.
    match bookmarks.iter().position(|b| b.id == id) {
        Some(index) => {
            bookmarks.remove(index);
            HttpResponse::NoContent().finish()
        }
        None => HttpResponse::NotFound().body("No se encontró el marcador")
    }

}