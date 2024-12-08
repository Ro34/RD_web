use actix_web::{web, App, HttpServer};
use mysql::*;

mod models;
mod routes;
mod services;

use routes::user::{query_db, insert_user, delete_user, update_user};
use services::db::DbService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 数据库连接配置
    let user = "root";
    let password = "123456Aa";
    let host = "127.0.0.1";
    let port = 3308; // 确保端口号正确
    let db_name = "my_db";
    let url = format!("mysql://{}:{}@{}:{}/{}", user, password, host, port, db_name);

    // 创建数据库连接池
    let pool = Pool::new(url.as_str())
        .expect("无法创建数据库连接池");

    // 初始化 DbService
    let db_service = DbService::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_service.clone()))
            .route("/query", web::get().to(query_db))
            .route("/insert", web::post().to(insert_user))
            .route("/delete/{id}", web::delete().to(delete_user))
            .route("/update/{id}", web::put().to(update_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}