// src/services/db.rs

use mysql::*;
use mysql::prelude::*;
use serde::Serialize;

#[derive(Clone)]
pub struct DbService {
    pool: Pool,
}

impl DbService {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    /// 创建用户
    pub fn create_user(&self, name: &str) -> Result<(), Error> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "INSERT INTO users (name) VALUES (:name)",
            params! {
                "name" => name,
            },
        )?;
        Ok(())
    }

    /// 查询所有用户
    pub fn read_users(&self) -> Result<Vec<User>, Error> {
        let mut conn = self.pool.get_conn()?;
        let users = conn.query_map(
            "SELECT id, name FROM users",
            |(id, name)| User { id, name },
        )?;
        Ok(users)
    }

    /// 更新用户
    pub fn update_user(&self, id: i32, name: &str) -> Result<(), Error> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "UPDATE users SET name = :name WHERE id = :id",
            params! {
                "id" => id,
                "name" => name,
            },
        )?;
        Ok(())
    }

    /// 删除用户
    pub fn delete_user(&self, id: i32) -> Result<(), Error> {
        let mut conn = self.pool.get_conn()?;
        conn.exec_drop(
            "DELETE FROM users WHERE id = :id",
            params! {
                "id" => id,
            },
        )?;
        Ok(())
    }
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}