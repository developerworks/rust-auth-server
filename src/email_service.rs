// email_service.rs
use crate::errors::ServiceError;
use crate::models::Invitation;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

pub fn send_invitation(invitation: &Invitation) -> Result<(), ServiceError> {

    let smtp_account = std::env::var("SMTP_ACCOUNT").expect("SMTP_ACCOUNT must be set");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    let mail_from = std::env::var("MAIL_FROM").expect("MAIL_FROM must be set");
    let mail_to = std::env::var("MAIL_TO").expect("MAIL_TO must be set");

    let email_body = format!(
        "Please click on the link below to complete registration. <br/>
         <a href=\"http://localhost:3000/register.html?id={}&email={}\">
         http://localhost:3030/register</a> <br>
         your Invitation expires on <strong>{}</strong>",
        invitation.id,
        invitation.email,
        invitation.expires_at.format("%I:%M %p %A, %-d %B, %C%y")
    );

    // complete the email message with details
    // email
    //     .add_recipient(recipient)
    //     .options(options)
    //     .subject("You have been invited to join Simple-Auth-Server Rust")
    //     .html(email_body);

    // let result = tm.send(&email);

    let email = Message::builder()
        .from(mail_from.parse().unwrap())
        .reply_to(mail_from.parse().unwrap())
        .to(mail_to.parse().unwrap())
        .subject("Registration validation link")
        .body(email_body)
        .unwrap();

    let creds = Credentials::new(smtp_account, smtp_password);

    let mailer = SmtpTransport::relay("smtp.163.com")
        .unwrap()
        .credentials(creds)
        // .authentication(vec!(Mechanism::Plain))
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            // println!("Response: {:?}", resp);
            println!("Email sent successfully!");
            Ok(())
        }
        Err(e) => {
            println!("Could not send email: {:?}", e);
            Err(ServiceError::InternalServerError)
        }
    }
}
