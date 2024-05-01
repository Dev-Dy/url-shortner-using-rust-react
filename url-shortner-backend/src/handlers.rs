use crate::models::{ShortenRequest, UrlMapping};
use crate::utils::{get_long_url, shorten_url};
use actix_web::{web, HttpResponse};
use sqlx::SqlitePool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/shorten").route(web::post().to(shorten)))
        .service(web::resource("/{short_code}").route(web::get().to(redirect)));
}

async fn shorten(
    req_body: web::Json<ShortenRequest>,
    db_pool: web::Data<SqlitePool>,
) -> HttpResponse {
    let short_code = shorten_url(&req_body.url);
    match UrlMapping::create(&db_pool, &short_code, &req_body.url).await {
        Ok(_) => HttpResponse::Ok().json(short_code),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn redirect(short_code: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    match get_long_url(&db_pool, &short_code).await {
        Ok(Some(long_url)) => HttpResponse::PermanentRedirect()
            .header("Location", long_url)
            .finish(),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
