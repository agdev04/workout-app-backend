use actix_web::{HttpResponse, Result};
use serde_json::json;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::schema::*;

#[derive(serde::Serialize)]
pub struct Statistics {
    users_count: i64,
    exercises_count: i64,
    workouts_count: i64,
    categories_count: i64,
    equipment_count: i64,
    body_parts_count: i64,
    meals_count: i64,
    programmes_count: i64
}

pub async fn get_statistics() -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let users_count: i64 = users::table
        .count()
        .get_result(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let exercises_count: i64 = exercises::table
        .count()
        .get_result(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let workouts_count: i64 = workouts::table
        .count()
        .get_result(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let categories_count: i64 = categories::table
        .count()
        .get_result(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let equipment_count: i64 = equipment::table
        .count()
        .get_result(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let body_parts_count: i64 = body_parts::table
        .count()
        .get_result(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let meals_count: i64 = meals::table
        .count()
        .get_result(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let programmes_count: i64 = programmes::table
        .count()
        .get_result(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let statistics = Statistics {
        users_count,
        exercises_count,
        workouts_count,
        categories_count,
        equipment_count,
        body_parts_count,
        meals_count,
        programmes_count
    };

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": statistics
    })))
}