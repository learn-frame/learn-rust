use std::thread;
use std::time::Duration;

use actix_cors::Cors;
use actix_web::{http::header, middleware, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    user_id: String,
    username: String,
    full_name: String,
    biography: String,
    email: String,
    profile_pic_url: String,
    follower_count: i32,
    follow_count: i32,
}

/// This handler uses json extractor
async fn get_user() -> HttpResponse {
    thread::sleep(Duration::from_millis(1000));

    let user = User {
        user_id:  Uuid::new_v4().to_string(),
        username: String::from("YanceyOfficial"),
        full_name: String::from("Yancey Leo"),
        biography: String::from("Music Producer | Software Engineer"),
        email: String::from("yanceyofficial@gmail.com"),
        profile_pic_url: String::from("https://scontent-nrt1-1.cdninstagram.com/v/t51.2885-19/50701353_260157948214698_4752524794996457472_n.jpg?stp=dst-jpg_s320x320&_nc_ht=scontent-nrt1-1.cdninstagram.com&_nc_cat=106&_nc_ohc=5GQwsIVvTR0AX8fXQAi&tn=HlJjMFEZ3tLz5Hrx&edm=AOQ1c0wBAAAA&ccb=7-5&oh=00_AT9_Ev9_ls9gHQYTZsX-rFgnMk14wdIu06NIp2mO-gnb7A&oe=632FC93E&_nc_sid=8fd12b"),
        follower_count: 1000,
        follow_count: 100
    };

    let res = web::Json(user);

    HttpResponse::Ok().json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:3002");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/user").route(web::get().to(get_user)))
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}
