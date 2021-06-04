use crate::database::Database;
use actix_web::{post, web, HttpResponse, Responder};
use common::{
    request::{
        CreateDeviceRequest, FetchDeviceListRequest, FetchDeviceProfileRequest, FetchDeviceRequest,
        FetchMessageListRequest, LoginRequest, ModifyDeviceRequest, RegisterRequest,
        RemoveDeviceRequest,
    },
    response::{
        ErrorResponse, FetchDeviceListResponse, FetchDeviceProfileResponse, FetchDeviceResponse,
        FetchMessageListResponse, LoginResponse, SimpleResponse,
    },
};

#[post("/login")]
async fn login(info: web::Json<LoginRequest>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.login(info).await {
        Ok((login_token, mail, name)) => HttpResponse::Ok().json(LoginResponse {
            success: true,
            err: "".to_string(),
            login_token,
            mail,
            name,
        }),
        Err(err) => HttpResponse::Ok().json(LoginResponse::err(err)),
    }
}

#[post("/register")]
async fn register(info: web::Json<RegisterRequest>, db: web::Data<Database>) -> impl Responder {
    let info = info.into_inner();
    match db.register(info).await {
        Ok(_) => HttpResponse::Ok().json(SimpleResponse {
            success: true,
            err: "".to_string(),
        }),
        Err(err) => HttpResponse::Ok().json(SimpleResponse::err(err)),
    }
}

#[post("/logout")]
async fn logout(info: web::Json<String>, db: web::Data<Database>) -> impl Responder {
    let login_token = info.into_inner();
    match db.logout(&login_token).await {
        Ok(_) => HttpResponse::Ok().json(SimpleResponse {
            success: true,
            err: "".to_string(),
        }),
        Err(err) => HttpResponse::Ok().json(SimpleResponse::err(err)),
    }
}

#[post("/check_login")]
async fn check_login(info: web::Json<String>, db: web::Data<Database>) -> impl Responder {
    let login_token = info.into_inner();
    if let Ok(res) = db.check_login(&login_token).await {
        if res {
            return HttpResponse::Ok().json(SimpleResponse {
                success: true,
                err: "".to_string(),
            })
        }
    }
    HttpResponse::Ok().json(SimpleResponse::err("Login has expired"))
}

#[post("/create_device")]
async fn create_device(
    info: web::Json<CreateDeviceRequest>,
    db: web::Data<Database>,
) -> impl Responder {
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
async fn remove_device(
    info: web::Json<RemoveDeviceRequest>,
    db: web::Data<Database>,
) -> impl Responder {
    let info = info.into_inner();
    match db.remove_device(info).await {
        Ok(_) => HttpResponse::Ok().json(SimpleResponse {
            success: true,
            err: "".to_string(),
        }),
        Err(err) => HttpResponse::Ok().json(SimpleResponse::err(err)),
    }
}

#[post("/modify_device")]
async fn modify_device(
    info: web::Json<ModifyDeviceRequest>,
    db: web::Data<Database>,
) -> impl Responder {
    let info = info.into_inner();
    match db.modify_device(info).await {
        Ok(_) => HttpResponse::Ok().json(SimpleResponse {
            success: true,
            err: "".to_string(),
        }),
        Err(err) => HttpResponse::Ok().json(SimpleResponse::err(err)),
    }
}

#[post("/fetch_device")]
async fn fetch_device(
    info: web::Json<FetchDeviceRequest>,
    db: web::Data<Database>,
) -> impl Responder {
    let info = info.into_inner();
    match db.fetch_device(info).await {
        Ok((id, name, info)) => HttpResponse::Ok().json(FetchDeviceResponse {
            success: true,
            err: "".to_string(),
            id,
            name,
            info,
        }),
        Err(err) => HttpResponse::Ok().json(FetchDeviceResponse::err(err)),
    }
}

#[post("/fetch_device_profile")]
async fn fetch_device_profile(
    info: web::Json<FetchDeviceProfileRequest>,
    db: web::Data<Database>,
) -> impl Responder {
    let info = info.into_inner();
    match db.fetch_device_profile(info).await {
        Ok(info) => HttpResponse::Ok().json(FetchDeviceProfileResponse {
            success: true,
            err: "".to_string(),
            name: info.name,
            message_count: info.message_count,
            alert_message_count: info.alert_message_count,
        }),
        Err(err) => HttpResponse::Ok().json(FetchDeviceProfileResponse::err(err)),
    }
}

#[post("/fetch_device_list")]
async fn fetch_device_list(
    info: web::Json<FetchDeviceListRequest>,
    db: web::Data<Database>,
) -> impl Responder {
    let info = info.into_inner();
    match db.fetch_device_list(info).await {
        Ok(devices) => HttpResponse::Ok().json(FetchDeviceListResponse {
            success: true,
            err: "".to_string(),
            devices,
        }),
        Err(err) => HttpResponse::Ok().json(FetchDeviceListResponse::err(err)),
    }
}

#[post("/fetch_message_list")]
async fn fetch_message_list(
    info: web::Json<FetchMessageListRequest>,
    db: web::Data<Database>,
) -> impl Responder {
    let info = info.into_inner();
    match db.fetch_message_list(info).await {
        Ok(messages) => HttpResponse::Ok().json(FetchMessageListResponse {
            success: true,
            err: "".to_string(),
            messages,
        }),
        Err(err) => HttpResponse::Ok().json(FetchMessageListResponse::err(err)),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login)
        .service(register)
        .service(logout)
        .service(check_login)
        .service(create_device)
        .service(remove_device)
        .service(modify_device)
        .service(fetch_device)
        .service(fetch_device_profile)
        .service(fetch_device_list)
        .service(fetch_message_list);
}
