use actix_identity::Identity;
use actix_web::{delete, get, post, HttpMessage};
use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest, HttpResponse};
use diesel::prelude::*;
use futures::future::{err, ok, Ready};
use serde::Deserialize;

use crate::errors::ServiceError;
use crate::models::{Pool, SlimUser, User};
use crate::utils::verify;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
}

// we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        // dbg!(req);

        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Ok(user_object) = identity.id() {
                if let Ok(user) = serde_json::from_str(&user_object) {
                    return ok(user);
                }
            }
        }
        err(ServiceError::Unauthorized.into())
    }

    fn extract(req: &HttpRequest) -> Self::Future {
        Self::from_request(req, &mut Payload::None)
    }
}

#[delete("/auth")]
pub async fn logout(id: Identity) -> HttpResponse {
    id.logout();
    HttpResponse::Ok().finish()
}

#[post("/auth")]
pub async fn login(
    request: HttpRequest,
    auth_data: web::Json<AuthRequest>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user = web::block(move || query(auth_data.into_inner(), pool))
        .await?
        .unwrap();
    let user_string = serde_json::to_string(&user)?;
    Identity::login(&request.extensions(), user_string).unwrap();
    Ok(HttpResponse::Ok().json(user))
}

#[get("/auth")]
pub async fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}

/// Diesel query
fn query(auth_data: AuthRequest, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{email, users};
    let conn = &mut pool.get()?;
    let mut items = users
        .filter(email.eq(&auth_data.email))
        .load::<User>(conn)?;

    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.hash, &auth_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}
