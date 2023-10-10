use actix_web::{
    delete, get, http::header::ContentType, middleware::Logger, post, web, App, HttpRequest,
    HttpResponse, HttpServer,
};
use clap::Parser;
use std::collections::HashMap;
use std::sync::Mutex;

/// App state which stores all keys and values.
struct AppState {
    store: Mutex<HashMap<String, String>>,
}

/// Returns a value from the given path.
#[get("/{key:.*}")]
async fn get(request: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let key: String = request.uri().to_string();

    match data.store.lock().unwrap().get(&key) {
        Some(value) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body(value.to_string())
        }
        _ => {
            return HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("key not found")
        }
    }
}

/// Sets a value to the given path.
#[post("/{key:.*}")]
async fn set(request: HttpRequest, post: web::Bytes, data: web::Data<AppState>) -> HttpResponse {
    let key: String = request.uri().to_string();
    let value = String::from_utf8(post.to_vec()).unwrap();

    match data.store.lock().unwrap().insert(key, value) {
        None => HttpResponse::Created()
            .content_type(ContentType::plaintext())
            .body("new value inserted"),
        Some(_) => HttpResponse::Accepted()
            .content_type(ContentType::plaintext())
            .body("value updated"),
    }
}

/// Removes a given key.
#[delete("/{key:.*}")]
async fn del(request: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let key: String = request.uri().to_string();

    match data.store.lock().unwrap().remove(&key) {
        Some(_) => {
            return HttpResponse::Ok()
                .content_type(ContentType::plaintext())
                .body("key and value removed")
        }
        _ => {
            return HttpResponse::NotFound()
                .content_type(ContentType::plaintext())
                .body("key not found")
        }
    }
}

/// Simple HTTP-based key-value-store
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host of the web service
    #[arg(long, default_value_t = String::from("0.0.0.0"))]
    host: String,

    /// Port of the web service
    #[arg(long, default_value_t = 1984)]
    port: u16,
}

/// Main method to instantiate key value store and start http server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let state = web::Data::new(AppState {
        store: Mutex::new(HashMap::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(get)
            .service(set)
            .service(del)
            .wrap(Logger::new("%a %{User-Agent}i"))
    })
    .bind((args.host, args.port))?
    .run()
    .await
}
