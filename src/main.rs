use actix_web::{
    web::{self, Data},
    App, HttpResponse, HttpServer,
};
use redis::{aio::MultiplexedConnection, AsyncCommands};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379").expect("failed to connect to redis");
    let redis_conn = client
        .get_multiplexed_tokio_connection()
        .await
        .expect("failed to create multiplexed connection");
    let redis_conn = Data::new(redis_conn);

    HttpServer::new(move || {
        App::new()
            .route("/test_redis", web::get().to(test_redis))
            .app_data(redis_conn.clone())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn test_redis(redis_conn: web::Data<MultiplexedConnection>) -> HttpResponse {
    let mut redis_conn = redis_conn.as_ref().clone();
    let _ = set(&mut redis_conn, "my_key", "my_value").await;
    HttpResponse::Ok().finish()
}

async fn set(
    redis_conn: &mut MultiplexedConnection,
    key: &str,
    value: &str,
) -> redis::RedisResult<()> {
    redis_conn.set(key, value).await?;
    Ok(())
}
