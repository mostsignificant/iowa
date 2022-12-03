use actix_web::{get, post, web, http::header::ContentType, App, HttpResponse, HttpServer};
use clap::Parser;
use std::collections::{HashMap};
use std::sync::Mutex;

/// App state which stores all keys and values.
struct AppState {
    store: Mutex<HashMap<String, String>>,
}

/// Returns a value from the given path.
#[get("/{key:.*}")]
async fn get(path: web::Path<(String,)>, data: web::Data<AppState>) -> HttpResponse {
    let key = path.into_inner().0;
    match data.store.lock().unwrap().get(&key) {
        Some(value) =>
            return 
                HttpResponse::Ok()
                    .content_type(ContentType::plaintext())
                    .body(value.to_string()),
        _ => 
            return
                HttpResponse::NotFound()
                    .content_type(ContentType::plaintext())
                    .body("key not found"),
    }
}

/// Sets a value to the given path.
#[post("/{key:.*}")]
async fn set(path: web::Path<(String,)>, value: web::Bytes, data: web::Data<AppState>) -> HttpResponse {
    let key = path.into_inner().0;
    let value = String::from_utf8(value.to_vec()).unwrap();
    match data.store.lock().unwrap().insert(key, value) {
        None =>
            HttpResponse::Created()
                .content_type(ContentType::plaintext())
                .body("new value inserted"),
        Some(_) =>
            HttpResponse::Accepted()
                .content_type(ContentType::plaintext())
                .body("value updated"),
    }
}

/// Simple HTTP-based key-value-store
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host of the web service
   #[arg(default_value_t = String::from("0.0.0.0"))]
   host: String,

   /// Port of the web service
   #[arg(default_value_t = 1984)]
   port: u16,
}

/// Main method to instantiate key value store and start http server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let state = 
        web::Data::new(AppState {
            store: Mutex::new(HashMap::new()),
        });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get)
            .service(set)
    })
    .bind((args.host, args.port))?
    .run()
    .await
}