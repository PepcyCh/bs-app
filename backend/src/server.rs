use actix_web::{post, web, HttpResponse, Responder};
use common::{request::{CreateDeviceRequest, FetchDevicesRequest, FetchMessagesRequest, LoginRequest, RegisterRequest, RemoveDeviceRequest}, response::{ErrorResponse, FetchDevicesResponse, FetchMessagesResponse, LoginResponse, SimpleResponse}};
use crate::database::Database;

#[post("/login")]
async fn login(info: web::Json<LoginRequest>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.try_login(info).await {
        Ok((mail, name)) => HttpResponse::Ok().json(LoginResponse {
            success: true,
            err: "".to_string(),
            mail,
            name,
        }),
        Err(err) => HttpResponse::Ok().json(LoginResponse::err(err)),
    }
}

#[post("/register")]
async fn register(info: web::Json<RegisterRequest>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.try_register(info).await {
        Ok(_) => HttpResponse::Ok().json(SimpleResponse {
            success: true,
            err: "".to_string(),
        }),
        Err(err) => HttpResponse::Ok().json(SimpleResponse::err(err)),
    }
}

#[post("/create_device")]
async fn create_device(info: web::Json<CreateDeviceRequest>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.create_device(info).await {
        Ok(_) => HttpResponse::Ok().json(SimpleResponse {
            success: true,
            err: "".to_string(),
        }),
        Err(err) => HttpResponse::Ok().json(SimpleResponse::err(err)),
    }
}

#[post("/remove_device")]
async fn remove_device(info: web::Json<RemoveDeviceRequest>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.remove_device(info).await {
        Ok(_) => HttpResponse::Ok().json(SimpleResponse {
            success: true,
            err: "".to_string(),
        }),
        Err(err) => HttpResponse::Ok().json(SimpleResponse::err(err)),
    }
}

#[post("/fetch_devices")]
async fn fetch_devices(info: web::Json<FetchDevicesRequest>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.fetch_devices(info).await {
        Ok(devices) => HttpResponse::Ok().json(FetchDevicesResponse {
            success: true,
            err: "".to_string(),
            devices,
        }),
        Err(err) => HttpResponse::Ok().json(FetchDevicesResponse::err(err)),
    }
}

#[post("/fetch_messages")]
async fn fetch_messages(info: web::Json<FetchMessagesRequest>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.fetch_messages(info).await {
        Ok(messages) => HttpResponse::Ok().json(FetchMessagesResponse {
            success: true,
            err: "".to_string(),
            messages,
        }),
        Err(err) => HttpResponse::Ok().json(FetchMessagesResponse::err(err)),
    }
}

// TODO - modify device

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(create_device)
        .service(remove_device)
        .service(fetch_devices)
        .service(fetch_messages);
}
