use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use crate::storage::{store_file, retrieve_file};

async fn upload(data: web::Bytes) -> impl Responder {
    let hash = store_file("upload.dat", &data);
    HttpResponse::Ok().body(format!("Stored with hash: {}", hash))
}

async fn download(hash: web::Path<String>) -> impl Responder {
    match retrieve_file(&hash) {
        Some(content) => HttpResponse::Ok().body(content),
        None => HttpResponse::NotFound().body("File not found"),
    }
}

#[actix_web::main]
pub async fn start_api() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/upload", web::post().to(upload))
            .route("/download/{hash}", web::get().to(download))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
