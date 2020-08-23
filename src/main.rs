use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::{StreamExt, TryStreamExt};
use hashbrown::HashMap;
use std::clone::Clone;
use std::env;
use std::str;
use std::sync::{Arc, Mutex};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let mut a = HashMap::new();
    a.insert(
        "/".to_string(),
        "under construction, please comeback later".to_string(),
    );

    let data = Arc::new(Mutex::new(a.to_owned()));
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be a number");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(data.clone())
            .route("/test", web::get().to(fetch))
            .route("/archive/", web::post().to(archive))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

async fn fetch(data: web::Data<Arc<Mutex<HashMap<String, String>>>>) -> impl Responder {
    let guard = data.lock().unwrap();
    let content = guard.get("test");
    let a = match content {
        Some(value) => value.to_owned(),
        None => "not found".to_owned(),
    };
    std::mem::drop(guard);
    HttpResponse::Ok().content_type("text/html").body(a)
}

async fn archive(
    mut payload: Multipart,
    data: web::Data<Arc<Mutex<HashMap<String, String>>>>,
) -> Result<HttpResponse, Error> {
    let key = "test";

    let mut content = String::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Some(chunk) = field.next().await {
            let incoming_data = chunk.unwrap();
            let x = str::from_utf8(&incoming_data[..]).unwrap();
            content.push_str(x);
        }
    }
    data.lock().unwrap().insert(key.to_string(), content);
    std::mem::drop(data);
    Ok(HttpResponse::Ok().body(key))
}
