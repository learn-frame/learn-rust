use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{
    get, http::header, middleware, post, web, App, HttpResponse, HttpServer, Responder,
};
use fake::{
    faker::boolean::en::Boolean,
    faker::internet::en::{SafeEmail, Username},
    faker::lorem::en::Paragraph,
    faker::name::en::Name,
    uuid::UUIDv4,
    Fake,
};
use futures::{StreamExt, TryStreamExt};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{io::Write, thread};

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

#[derive(Serialize, Deserialize)]
struct Like {
    like_count: i32,
    has_liked: bool,
}

fn generate_user() -> User {
    User {
        user_id: UUIDv4.fake(),
        username: Username().fake(),
        full_name: Name().fake(),
        biography: Paragraph(3..5).fake(),
        email: SafeEmail().fake(),
        profile_pic_url: format!(
            "https://dummyimage.com/{}x{}.png",
            rand::thread_rng().gen_range(300..600),
            rand::thread_rng().gen_range(300..600),
        ),
        follower_count: rand::thread_rng().gen_range(100..6000),
        follow_count: rand::thread_rng().gen_range(100..6000),
    }
}

#[get("/user")]
async fn get_user() -> impl Responder {
    thread::sleep(Duration::from_millis(1000));

    let user = generate_user();
    HttpResponse::Ok().json(user)
}

#[get("/users")]
async fn get_users() -> impl Responder {
    thread::sleep(Duration::from_millis(1000));

    let rand_num = rand::thread_rng().gen_range(2..10);
    let mut users: Vec<User> = vec![];

    for _ in (1..=rand_num).rev() {
        users.push(generate_user());
    }

    HttpResponse::Ok().json(users)
}

#[post("/user")]
async fn create_user() -> impl Responder {
    thread::sleep(Duration::from_millis(1000));

    let user = generate_user();
    HttpResponse::Ok().json(user)
}

#[get("/like")]
async fn get_like() -> impl Responder {
    thread::sleep(Duration::from_millis(1000));

    HttpResponse::Ok().json(Like {
        like_count: rand::thread_rng().gen_range(100..200),
        has_liked: Boolean(1).fake(),
    })
}

#[post("/like")]
async fn handle_like(req: web::Json<Like>) -> impl Responder {
    thread::sleep(Duration::from_millis(1000));

    let like_count = match req.has_liked {
        true => req.like_count - 1, // ✅
        false => req.like_count + 1,
    };

    HttpResponse::Ok().json(Like {
        like_count,
        has_liked: !req.has_liked,
    })
}

#[post("/upload")]
async fn upload(mut payload: Multipart) -> impl Responder {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap().to_string();

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filename))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || {
                let mut file = f.unwrap();
                file.write_all(&data).map(|_| file)
            })
            .await
            .unwrap();
        }
    }

    HttpResponse::Ok().json(true)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:3002");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(4096))
            .service(get_user)
            .service(get_users)
            .service(create_user)
            .service(get_like)
            .service(handle_like)
            .service(upload)
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}
