#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::thread;
use std::path::PathBuf;

use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, HttpRequest, Result, http::header::{DispositionType, ContentDisposition, DispositionParam}};
use lopdf::Document;
use mime;

#[tauri::command]
fn greet(name: &str, v1: u32, v2: i32, v3: f32, v4: f64) -> String {
    format!("Hello, {}! You've been greeted from Rust!, {}, {}, {}, {}", name, v1, v2, v3, v4)
}    

pub mod pdf; // присоединяет модуль к маину
use crate::pdf::*; // импортит модуль и его функции

#[tauri::command]
fn parse_codes(text: &str) -> String {
    
    let documents = vec![
        Document::load("./static/MS-RD-T1-2-11.pdf").unwrap(),
        Document::load("./static/MS-RD-T1-2-12.pdf").unwrap(),
    ];

    merge_pdf_files(documents).unwrap();

    format!("{}", text)
}    

// #[get("/merged")]
// async fn hello() -> impl Responder {


//     let documents = vec![
//         Document::load("./static/MS-RD-T1-2-11.pdf").unwrap(),    
//         Document::load("./static/MS-RD-T1-2-12.pdf").unwrap(),
//     ];

//     let s: Stream = Stream::new(dict, content)
//     let document = merge_pdf_files(documents).unwrap().save_to(|w| -> {

//     } );    
//     //actix_web::Responder::respond_to(self, req)


//     HttpResponse::Ok().body()
// }

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();

    let path = match path.to_str() {
        Some(str) => str,
        None => "",
    };    

    Ok(NamedFile::open(format!("./static/{}", path))?.set_content_disposition( ContentDisposition {
        disposition: DispositionType::Inline,
        parameters: vec![DispositionParam::Filename(String::from("barcode.pdf"))],
    }).set_content_type(mime::APPLICATION_PDF))    
}    

#[actix_web::main]
async fn actix_main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/{filename:.*}", web::get().to(index))
    })    
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}    

fn main() {
    thread::spawn(actix_main);

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, parse_codes])
    .run(tauri::generate_context!())
    .expect("error while running tauri application")
}

