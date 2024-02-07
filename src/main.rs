use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;
use std::path::{Path, MAIN_SEPARATOR};

use actix_multipart::form::{
    tempfile::{TempFile, TempFileConfig},
    MultipartForm,
};
use uuid::Uuid;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    file: TempFile,
}

async fn handle_upload(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let dir = env::var("file_path").unwrap();
    let file = form.file;
    let filename = file.file_name.unwrap();
    let ext = Path::new(&filename).to_str().unwrap();
    let path = format!("{}{}{}{}", dir, MAIN_SEPARATOR, Uuid::new_v4(), ext);
    file.file.persist(path).unwrap();
    Ok(HttpResponse::Ok())
}

#[get("/ping")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("启动");
    dotenv().ok();
    let file_path = env::var("file_path").unwrap();
    let upload_route = env::var("upload_route").unwrap();
    let download_route = env::var("download_route").unwrap();
    let ip = env::var("ip").unwrap();
    let port: u16 = env::var("port").unwrap().parse().unwrap();
    std::fs::create_dir_all(&file_path)?;
    HttpServer::new(move || {
        App::new()
            .app_data(TempFileConfig::default().directory(&file_path))
            .service(
                fs::Files::new(download_route.as_str(), &file_path), // .show_files_listing()
            )
            .route(upload_route.as_str(), web::post().to(handle_upload))
            .service(hello)
    })
    .bind((ip, port))?
    .run()
    .await
}
