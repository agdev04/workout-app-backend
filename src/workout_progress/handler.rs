use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;

use crate::db::establish_connection;
use crate::schema::workout_progress;
use super::model::{NewWorkoutProgress, UpdateWorkoutProgress, WorkoutProgress};

pub async fn record_progress(new_progress: web::Json<NewWorkoutProgress>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(workout_progress::table)
        .values(&new_progress.into_inner())
        .get_result::<WorkoutProgress>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": result
    })))
}

pub async fn update_progress(
    id: web::Path<i32>,
    update_data: web::Json<UpdateWorkoutProgress>
) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let progress_id = id.into_inner();

    let result = diesel::update(workout_progress::table.find(progress_id))
        .set(update_data.into_inner())
        .get_result::<WorkoutProgress>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": result
    })))
}

pub async fn get_user_workout_progress(user_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let progress = workout_progress::table
        .filter(workout_progress::user_id.eq(user_id.into_inner()))
        .order_by(workout_progress::completed_at.desc())
        .load::<WorkoutProgress>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": progress
    })))
}

pub async fn get_workout_progress(
    workout_id: web::Path<i32>
) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let progress = workout_progress::table
        .filter(workout_progress::workout_id.eq(workout_id.into_inner()))
        .order_by(workout_progress::completed_at.desc())
        .load::<WorkoutProgress>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": progress
    })))
}