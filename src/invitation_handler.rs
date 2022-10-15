use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::email_service;
use crate::models::{Invitation, Pool};
use diesel::RunQueryDsl;

#[derive(Deserialize)]
pub struct InvitationData {
    pub email: String,
}

// 发送验证
pub async fn post_invitation(
    invitation_data: web::Json<InvitationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    web::block(move || {
        // 
        create_invitation(invitation_data.into_inner().email, pool)
    }).await??;

    Ok(HttpResponse::Ok().finish())
}

fn create_invitation(
    eml: String,
    pool: web::Data<Pool>,
) -> Result<(), crate::errors::ServiceError> {
    let invitation = dbg!(query(eml, pool)?);
    email_service::send_invitation(&invitation)
}

/// Diesel query
fn query(eml: String, pool: web::Data<Pool>) -> Result<Invitation, crate::errors::ServiceError> {
    use crate::schema::invitations::dsl::invitations;

    let new_invitation: Invitation = eml.into();
    let conn = &mut pool.get()?;

    let _ = diesel::insert_into(invitations)
        .values([new_invitation.clone()])
        .execute(conn);

    Ok(new_invitation)
}
