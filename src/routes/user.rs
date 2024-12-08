use actix_web::{web, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;

use crate::models::user::UserInput;

pub async fn query_db(pool: web::Data<Pool>) -> impl Responder {
    let mut conn = pool.get_conn().expect("无法获取数据库连接");
    let result: Vec<(i32, String)> = conn.query("SELECT id, name FROM users").expect("查询失败");
    HttpResponse::Ok().json(result)
}

pub async fn insert_user(pool: web::Data<Pool>, user: web::Json<UserInput>) -> impl Responder {
    let mut conn = pool.get_conn().expect("无法获取数据库连接");
    conn.exec_drop(
        "INSERT INTO users (name) VALUES (:name)",
        params! {
            "name" => &user.name,
        },
    ).expect("插入失败");
    HttpResponse::Ok().body("用户插入成功")
}


pub async fn delete_user(pool: web::Data<Pool>, id: web::Path<i32>) -> impl Responder {
    let mut conn = pool.get_conn().expect("无法获取数据库连接");
    match conn.exec_drop(
        "DELETE FROM users WHERE id = :id",
        params! {
            "id" => id.into_inner(),
        },
    ) {
        Ok(_) => HttpResponse::Ok().body("用户删除成功"),
        Err(_) => HttpResponse::InternalServerError().body("删除失败"),
    }
}

pub async fn update_user(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    user: web::Json<UserInput>,
) -> impl Responder {
    let mut conn = pool.get_conn().expect("无法获取数据库连接");
    match conn.exec_drop(
        "UPDATE users SET name = :name WHERE id = :id",
        params! {
            "id" => id.into_inner(),
            "name" => &user.name,
        },
    ) {
        Ok(_) => HttpResponse::Ok().body("用户更新成功"),
        Err(_) => HttpResponse::InternalServerError().body("更新失败"),
    }
}