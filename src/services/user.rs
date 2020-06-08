use actix_web::{HttpResponse, web};
use serde::{Serialize, Deserialize};

use crate::db::{DbPool, models::User};

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    email: Option<String>,
    phone: Option<String>,
}

pub fn create(user_form: web::Json<UserForm>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match User::create(user_form.email.as_deref(), user_form.phone.as_deref(), &conn) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::InternalServerError().json("Could not create user")
    }
}

pub fn index(pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    HttpResponse::Ok().json(User::list(&conn))
}

pub fn get(id: web::Path<String>, pool: web::Data<DbPool>) -> HttpResponse {
    let conn = pool.get().unwrap();

    match User::by_id(&id, &conn) {
        Some(user) => HttpResponse::Ok().json(user),
        _ => HttpResponse::NotFound().json("Not Found")
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    /* 
     * index: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users
     * get: curl -i -X GET -H "Content-Type: application/json" http://localhost:5000/users/<id>
     * post: curl -i -X POST -H "Content-Type: application/json" -d '{"email":"xxx", "phone": "yyy"}' http://localhost:5000/users
     */
    
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create))
            .route(web::get().to(index))
    )
    .service(
        web::scope("/users")
            .route("/{id}", web::get().to(get)),
    );
}