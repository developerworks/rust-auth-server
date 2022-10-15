use actix_web::{web, HttpResponse};
use diesel::{prelude::*, RunQueryDsl};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Invitation, Pool, SlimUser, User};
use crate::schema;
use crate::utils::hash_password;

use schema::invitations::dsl::{email, id, invitations};
use schema::users::dsl::users;

// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
}

pub async fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    // Diesel 不支持异步, 因此使用 web::block 把数据库操作包装到异步块中, 避免阻塞
    let user = web::block(move || {
        //
        query(invitation_id.into_inner(), user_data.into_inner(), pool)
    })
    .await??;

    Ok(HttpResponse::Ok().json(&user))
}

fn query(
    invitation_id: String,
    user_data: UserData,
    pool: web::Data<Pool>,
) -> Result<SlimUser, crate::errors::ServiceError> {
    let conn = &mut pool.get()?;
    invitations
        .filter(id.eq(invitation_id))
        .filter(email.eq(&user_data.email))
        // .load::<Invitation>(conn)
        .first::<Invitation>(conn)
        .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
        .and_then(|invitation| {
            // if invitation is not expired
            if invitation.expires_at > chrono::Local::now().naive_local() {
                // try hashing the password, else return the error that will be converted to ServiceError
                let password: String = hash_password(&user_data.password)?;
                let user = User::from_details(invitation.email, password);
                let _ = diesel::insert_into(users).values([user.clone()]).execute(conn);
                dbg!(&user);
                return Ok(user.into());
            }
            Err(ServiceError::BadRequest("Invalid Invitation".into()))
        })
}
