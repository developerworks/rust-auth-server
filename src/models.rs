use crate::schema::{invitations, users};
use diesel::prelude::*;

use chrono::{Duration, Local, NaiveDateTime};
use diesel::{r2d2::ConnectionManager, MysqlConnection};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub email: String,
    pub hash: String,
    pub created_at: NaiveDateTime,
}
impl User {
    /**
     * 通过用户详细信息构造 User 对象
     * 参数 S, T 必须是能够转换为 String 类型的类型
     */
    pub fn from_details<S: Into<String>, T: Into<String>>(email: S, pwd: T) -> Self {
        User {
            email: email.into(),
            hash: pwd.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, PartialEq, Eq)]
#[diesel(table_name = invitations)]
pub struct Invitation {
    pub id: String,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}
impl<T> From<T> for Invitation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        Invitation {
            id: uuid::Uuid::new_v4().to_string(),
            email: email.into(),
            expires_at: Local::now().naive_local() + Duration::hours(24),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser { email: user.email }
    }
}
