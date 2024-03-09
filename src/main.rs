use actix_cors::Cors;
use actix_files as fs;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use std::env;
use std::fmt::Debug;
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

struct AppState {
    ip: String,
    port: u16,
    download_route: String,
}

async fn handle_upload(
    MultipartForm(form): MultipartForm<UploadForm>,
    data: web::Data<AppState>,
) -> String {
    let dir = env::var("file_path").unwrap();
    let file = form.file;
    // 拿出原始的文件名和拓展名
    let filename = file.file_name.unwrap();
    let ext = Path::new(&filename).to_str().unwrap();
    // 重新组合并存储
    let new_filename = format!("{}{}", Uuid::new_v4(), ext);
    let path = format!("{}{}{}", dir, MAIN_SEPARATOR, new_filename);
    file.file.persist(path).unwrap();
    // 组合供外部访问的url
    let ip = &data.ip;
    let port = &data.port;
    let download_route = &data.download_route;
    let url = format!("http://{}:{}{}/{}", ip, port, download_route, new_filename);
    url
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("文件服务启动中")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let file_path = env::var("file_path").unwrap();
    let upload_route = env::var("upload_route").unwrap();
    let download_route = env::var("download_route").unwrap();
    std::fs::create_dir_all(&file_path)?;
    let ip: String = env::var("ip").unwrap();
    let port: u16 = env::var("port").unwrap().parse().unwrap();
    println!("文件服务启动,http://{ip}:{port}");
    let ip_clone = ip.clone();
    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .app_data(TempFileConfig::default().directory(&file_path))
            .app_data(web::Data::new(AppState {
                ip: ip_clone.clone(),
                port: port.clone(),
                download_route: download_route.clone(),
            }))
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
