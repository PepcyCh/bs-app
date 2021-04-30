use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    database::Database,
    protocol::{LoginInfo, LoginResponse, RegisterInfo, RegisterResponse},
};

#[post("/login")]
async fn login(info: web::Json<LoginInfo>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.try_login(info).await {
        Ok((mail, name)) => HttpResponse::Ok().json(LoginResponse {
            success: true,
            err: "".to_string(),
            mail,
            name,
        }),
        Err(err) => HttpResponse::Ok().json(LoginResponse {
            success: false,
            err,
            mail: "".to_string(),
            name: "".to_string(),
        }),
    }
}

#[post("/register")]
async fn register(info: web::Json<RegisterInfo>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.try_register(info).await {
        Ok(_) => HttpResponse::Ok().json(RegisterResponse {
            success: true,
            err: "".to_string(),
        }),
        Err(err) => HttpResponse::Ok().json(RegisterResponse {
            success: false,
            err,
        }),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(register);
}
