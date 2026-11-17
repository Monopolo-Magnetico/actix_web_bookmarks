use std::sync::Mutex;
use crate::models::Bookmark;

pub struct AppState {
    pub bookmarks: Mutex<Vec<Bookmark>>
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            bookmarks: Mutex::new(vec![
                Bookmark {
                    id: 1,
                    title: String::from("Marcador de ejemplo 1"),
                    url: String::from("www.ejemplo1.com"),
                    tag: String::from("Etiqueta1")
                },
                Bookmark {
                    id: 2,
                    title: String::from("Marcador de ejemplo 2"),
                    url: String::from("www.ejemplo2.com"),
                    tag: String::from("Etiqueta2")
                }
            ])
        }
    }
}