use actix_web::{delete, get, post, web, HttpResponse, Result};
use serde_json::json;
use diesel::prelude::*;
use crate::{db::establish_connection, schema::favorite_workouts};
use crate::favorite_workouts::model::{FavoriteWorkout, NewFavoriteWorkout};

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[post("")]
pub async fn add_favorite_workout(new_favorite: web::Json<NewFavoriteWorkout>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(favorite_workouts::table)
        .values(&new_favorite.into_inner())
        .get_result::<FavoriteWorkout>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e));

    match result {
        Ok(favorite) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "data": favorite
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to add favorite workout".to_string(),
        }))
    }
}

#[delete("/{id}")]
pub async fn remove_favorite_workout(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let workout_id = id.into_inner();

    let result = diesel::delete(favorite_workouts::table.find(workout_id))
        .execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Favorite workout removed successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Favorite workout not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to remove favorite workout".to_string(),
        }))
    }
}

#[get("/user/{user_id}")]
pub async fn get_user_favorite_workouts(user_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    
    let results = favorite_workouts::table
        .filter(favorite_workouts::user_id.eq(user_id.into_inner()))
        .load::<FavoriteWorkout>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": results
    })))
}