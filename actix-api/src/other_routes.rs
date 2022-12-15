use actix_web::{get, web, HttpResponse, Responder};

async fn child2() -> impl Responder {
    "this is the second child"
}

pub fn config_with_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/child1")
            .route(web::get().to(|| async { HttpResponse::Ok().body("this is the first child") })),
    )
    .service(web::resource("/child2").route(web::get().to(child2)));
}

#[get("/config")]
async fn config_route() -> impl Responder {
    "testing from another file/module works. This is config 1"
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(config_route);
}
