// src/routes/user.rs

use actix_web::{web, HttpResponse, Responder};
use crate::services::db::DbService;
use crate::models::user::UserInput;
use serde::Serialize;
use mysql::Error;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

/// 查询用户列表的处理函数
pub async fn query_db(pool: web::Data<DbService>) -> impl Responder {
    let db = pool.clone();
    match db.read_users() {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().body("查询失败"),
    }
}

/// 插入新用户的处理函数
pub async fn insert_user(pool: web::Data<DbService>, user: web::Json<UserInput>) -> impl Responder {
    let db = pool.clone();
    match db.create_user(&user.name) {
        Ok(_) => HttpResponse::Ok().body("用户插入成功"),
        Err(_) => HttpResponse::InternalServerError().body("插入失败"),
    }
}

/// 删除用户的处理函数
pub async fn delete_user(pool: web::Data<DbService>, id: web::Path<i32>) -> impl Responder {
    let db = pool.clone();
    match db.delete_user(id.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("用户删除成功"),
        Err(_) => HttpResponse::InternalServerError().body("删除失败"),
    }
}

/// 更新用户的处理函数
pub async fn update_user(
    pool: web::Data<DbService>,
    id: web::Path<i32>,
    user: web::Json<UserInput>,
) -> impl Responder {
    let db = pool.clone();
    match db.update_user(id.into_inner(), &user.name) {
        Ok(_) => HttpResponse::Ok().body("用户更新成功"),
        Err(_) => HttpResponse::InternalServerError().body("更新失败"),
    }
}