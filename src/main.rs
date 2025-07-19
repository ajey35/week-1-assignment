use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpServer};
use crate::routes::{todo::{create_todo, get_todos}, user::{sign_in, sign_up}};
pub mod routes;
pub mod db;
pub mod middleware;
pub mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Arc::new(Mutex::new(db::Db::default()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(sign_up)
            .service(sign_in)
            .service(create_todo)
            .service(get_todos)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
