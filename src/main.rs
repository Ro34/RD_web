use actix_web::{web, App, HttpServer};
use mysql::*;

mod models;
mod routes;

use routes::user::{query_db, insert_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let user = "root";
    let password = "123456Aa";
    let host = "127.0.0.1";
    let port = 3308;
    let db_name = "my_db";
    let url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, db_name);

    let pool = Pool::new(url.as_str())
        .expect("无法创建数据库连接池");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/query", web::get().to(query_db))
            .route("/insert", web::post().to(insert_user))
            .route("/delete/{id}", web::delete().to(routes::user::delete_user))
            .route("/update/{id}", web::put().to(routes::user::update_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}