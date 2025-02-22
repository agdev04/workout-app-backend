use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use diesel::prelude::*;
use crate::{db::establish_connection, schema::{favorite_workouts, workouts, users}};
use crate::favorite_workouts::model::{FavoriteWorkout, NewFavoriteWorkout};
use crate::workouts::model::Workout;
use crate::users::model::User;

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct FavoriteWorkoutWithDetails {
    #[serde(flatten)]
    pub favorite: FavoriteWorkout,
    pub workout: Workout,
    pub user: User,
}

pub async fn add_favorite_workout(new_favorite: web::Json<NewFavoriteWorkout>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(favorite_workouts::table)
        .values(&new_favorite.into_inner())
        .get_result::<FavoriteWorkout>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(json!({
        "status": "success",
        "data": result
    })))
}

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

pub async fn get_user_favorite_workouts(user_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let user_id = user_id.into_inner();
    
    let favorites_with_details: Vec<FavoriteWorkoutWithDetails> = favorite_workouts::table
        .inner_join(workouts::table)
        .inner_join(users::table)
        .filter(favorite_workouts::user_id.eq(user_id))
        .select((
            favorite_workouts::all_columns,
            workouts::all_columns,
            users::all_columns,
        ))
        .load::<(FavoriteWorkout, Workout, User)>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .into_iter()
        .map(|(favorite, workout, user)| FavoriteWorkoutWithDetails {
            favorite,
            workout,
            user,
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": favorites_with_details
    })))
}