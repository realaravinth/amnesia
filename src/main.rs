use actix_multipart::Multipart;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer, Responder};
use futures::{StreamExt, TryStreamExt};
use std::env;
use std::str;
use std::sync::{Arc, RwLock};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = Arc::new(RwLock::new("Data unavailable".to_string()));
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

async fn fetch(data: web::Data<Arc<RwLock<String>>>) -> impl Responder {
    let content = data.read().unwrap();
    HttpResponse::Ok().content_type("text/html").body(&*content)
}
async fn archive(
    mut payload: Multipart,
    data: web::Data<Arc<RwLock<String>>>,
) -> Result<HttpResponse, Error> {
    use std::mem::swap;
    let mut content = String::new();
    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Some(chunk) = field.next().await {
            let incoming_data = chunk.unwrap();
            let x = str::from_utf8(&incoming_data[..]).unwrap();
            content.push_str(x);
        }
    }

    swap(&mut content, &mut *data.write().unwrap());
    drop(data);
    Ok(HttpResponse::Ok().finish())
}
