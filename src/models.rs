use serde::{Serialize, Deserialize};

// Esta anotación da la posibilidad de serializar a JSON
#[derive(Serialize, Clone)]
pub struct Bookmark {
    pub id: u32,
    pub title: String,
    pub url: String,
    pub tag: String // También puede ser un Vec<String> para muchas tags
}

/*
Serialize convierte este struct

const V1 = Bookmark {
    id: 1,
    title: String::from("Marcador de ejemplo"),
    url: String::from("www.ejemplo.com"),
    tag: String::from("Etiqueta de ejemplo")
};

En un JSON

"id": "1",
"title": "Marcador de ejemplo",
"url": "www.ejemplo.com",
"tag": "Etiqueta de ejemplo"

Deserialize hace el proceso contrario
*/

#[derive(Deserialize)]
pub struct TagFilter {
    pub tag: Option<String>
}

#[derive(Deserialize)]
pub struct NewBookmark {
    pub title: String,
    pub url: String,
    pub tag: String
}