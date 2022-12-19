use std::sync::Mutex;

use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};

mod other_routes;

struct AppInfo {
    app_name: String,
}

struct VisitorState {
    visitors: Mutex<u32>,
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_howdy() -> impl Responder {
    HttpResponse::Ok().body("Howdy!")
}

async fn static_index_html() -> impl Responder {
    // this did not work, it was worth trying
    // "<div style=\"width: 128px; height: 128px; background: green;\">index.html code</div>"

    "This route works"
}

#[get("/")]
async fn index_with_state(info: web::Data<AppInfo>) -> String {
    let app_name = &info.app_name;
    format!("Hello from {app_name}")
}

// #[get("/visitors")]
async fn visitor_count(data: web::Data<VisitorState>) -> String {
    let mut num_visitors = data.visitors.lock().unwrap();
    *num_visitors += 1;

    format!("{} people have visited this site", num_visitors)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let visitors = web::Data::new(VisitorState {
        visitors: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppInfo {
                app_name: String::from("Testing Server"),
            }))
            .app_data(visitors.clone())
            .service(hello)
            .service(echo)
            .route("/howdy", web::get().to(manual_howdy))
            .service(web::scope("/static").route("/index.html", web::get().to(static_index_html)))
            .service(index_with_state)
            .route("/visitors", web::get().to(visitor_count))
            .service(
                web::scope("/filter-from-rust-site")
                    .guard(guard::Header("Host", "www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("cool") })),
            )
            .configure(other_routes::config)
            .service(web::scope("/parent").configure(other_routes::config_with_scope))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/*
summary
- we can create routes manually or with macros
- creating with macros allows us to specify the method type and route

- with web::scope we can create a resource group prefix, this will prepend
a path ("/users" for example) onto the routes we specify
    let scope = web::scope("/users").service(show_users)
    App::new().service(scope)

- states initialized in the HttpServer::new closure are local to the worker thread
and can be de-synced if modified
- for globally shared state, it must be created outside the closure and moved or cloned in
    - i believe the mutex fixes various thread bugs and sync issues

*/
