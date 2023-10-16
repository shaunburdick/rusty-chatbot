mod db;
mod llm;

use actix_web::{get, HttpServer, App, Responder, HttpResponse, middleware::{Logger, DefaultHeaders}};
use env_logger::Env;



/// A simple hello world endpoint
///
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(DefaultHeaders::new().add(("app-version", env!("CARGO_PKG_VERSION"))))
            .wrap(Logger::default())
            .service(hello)
    })
    .workers(4)
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App, dev::Service, http::StatusCode, web::Bytes};

    use super::hello;

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(App::new().service(hello)).await;

        let req = test::TestRequest::get().uri("/").to_request();

        let res = app.call(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let body = test::read_body(res).await;
        assert_eq!(body, Bytes::from_static(b"Hello World"));
    }
}
