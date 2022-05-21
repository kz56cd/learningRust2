use actix_web::{get, App, HttpResponse, HttpServer, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {}

// MyErrorは、actix_webのエラーに準拠させる
impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<HttpResponse, MyError> {
    let response_body = "Hello rust world!";

    Ok(HttpResponse::Ok().body(response_body))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    // App生成、渡してサーバ起動
    HttpServer::new(move || App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await?;
    Ok(())
}