use actix::Addr;
use actix_redis::{resp_array, Command, RedisActor, RespValue};
use actix_web::{web, App, HttpResponse, HttpServer};

async fn get(addr: web::Data<Addr<RedisActor>>) -> HttpResponse {
    let res = addr
        .send(Command(resp_array!["SET", "test", "value"]))
        .await;

    match res {
        Ok(Ok(resp)) => {
            assert_eq!(resp, RespValue::SimpleString("OK".to_owned()));

            let res = addr.send(Command(resp_array!["GET", "test"])).await;
            match res {
                Ok(Ok(resp)) => {
                    dbg!(&resp);
                    HttpResponse::Ok().finish()
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let addr = RedisActor::start("127.0.0.1:6379");
    HttpServer::new(move || {
        App::new()
            .route("/hello", web::get().to(get))
            .app_data(addr.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
