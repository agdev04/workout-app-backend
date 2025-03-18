use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;

use super::model::{NewWorkoutProgress, UpdateWorkoutProgress, WorkoutProgress};
use crate::db::establish_connection;
use crate::exercises::model::Exercise;
use crate::schema::{exercises, workout_progress, workouts};
use crate::workouts::model::Workout;

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
    update_data: web::Json<UpdateWorkoutProgress>,
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
        .inner_join(exercises::table)
        .inner_join(workouts::table.on(workout_progress::workout_id.eq(workouts::id)))
        .filter(workout_progress::user_id.eq(user_id.into_inner()))
        .order_by(workout_progress::completed_at.desc())
        .select((
            workout_progress::all_columns,
            exercises::all_columns,
            workouts::all_columns,
        ))
        .load::<(WorkoutProgress, Exercise, Workout)>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let result: Vec<serde_json::Value> = progress
        .into_iter()
        .map(|(progress, exercise, workout)| {
            serde_json::json!({
                "id": progress.id,
                "user_id": progress.user_id,
                "workout_id": progress.workout_id,
                "exercise_id": progress.exercise_id,
                "workout_exercise_id": progress.workout_exercise_id,
                "completed": progress.completed,
                "actual_sets_number": progress.actual_sets_number,
                "actual_reps": progress.actual_reps,
                "actual_duration_seconds": progress.actual_duration_seconds,
                "notes": progress.notes,
                "completed_at": progress.completed_at,
                "created_at": progress.created_at,
                "updated_at": progress.updated_at,
                "deleted_at": progress.burned_calories,
                "exercise": {
                    "id": exercise.id,
                    "name": exercise.name,
                    "description": exercise.description,
                    "is_active": exercise.is_active,
                    "thumbnail_url": exercise.thumbnail_url,
                    "video_url": exercise.video_url
                },
                "workout": {
                    "id": workout.id,
                    "name": workout.name,
                    "description": workout.description,
                    "thumbnail_url": workout.thumbnail_url,
                    "created_at": workout.created_at,
                    "updated_at": workout.updated_at
                }
            })
        })
        .collect();

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "data": result
    })))
}

pub async fn get_workout_progress(workout_id: web::Path<i32>) -> Result<HttpResponse> {
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
