mod auth_handler;
mod email_service;
mod errors;
mod invitation_handler;
mod models;
mod register_handler;
mod schema;
mod utils;

use actix_identity::IdentityMiddleware;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};

use actix_web::{
    cookie::{self, Key},
    middleware, web, App, HttpServer,
};
use diesel::{r2d2::ConnectionManager, MysqlConnection};
#[allow(unused)]
use log::{debug, error, info, trace, warn};

#[allow(unused)]
async fn callback() {
    info!("Server listing on localhost:3000");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    dotenv::dotenv().ok();

    std::env::set_var(
        "RUST_LOG",
        "rust-auth-server=debug,actix_web=debug,actix_server=debug",
    );
    env_logger::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    // 创建数据库连接池
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let _domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    log::info!("starting HTTP server at http://localhost:3000");

    let secret_key = Key::generate();

    HttpServer::new(move || {
        let mw_identity = IdentityMiddleware::default();
        // 会话中间件
        let mw_session =
            SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                // disable secure cookie for local testing
                .cookie_secure(false)
                .session_lifecycle(
                    PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                )
                .build();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(mw_identity)
            .wrap(mw_session)
            .app_data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/invitation")
                            .route(web::post().to(invitation_handler::post_invitation)),
                    )
                    .service(
                        web::resource("/register/{invitation_id}")
                            .route(web::post().to(register_handler::register_user)),
                    )
                    .service(auth_handler::login)
                    .service(auth_handler::logout)
                    .service(auth_handler::get_me),
            )
    })
    // .bind(("127.0.0.1", 8080))?
    .bind("localhost:3000")?
    .run()
    .await
}
