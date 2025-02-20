use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::{db::establish_connection, schema::{workouts, workout_exercises, exercises}};
use diesel::prelude::*;
use crate::workouts::model::*;
use crate::exercises::model::Exercise;

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct WorkoutWithExercises {
    #[serde(flatten)]
    pub workout: Workout,
    pub exercises: Vec<WorkoutExerciseWithDetails>,
}

#[derive(Debug, serde::Serialize)]
pub struct WorkoutExerciseWithDetails {
    #[serde(flatten)]
    pub workout_exercise: WorkoutExercise,
    pub exercise: Exercise,
}

// Workout CRUD handlers
pub async fn create_workout(new_workout: web::Json<NewWorkout>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(workouts::table)
        .values(&new_workout.into_inner())
        .get_result::<Workout>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn get_workout_list() -> Result<HttpResponse> {
    let connection = &mut establish_connection();
    
    let workouts_list = workouts::table
        .load::<Workout>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let workouts_with_exercises: Vec<WorkoutWithExercises> = workouts_list
        .into_iter()
        .map(|workout| {
            let workout_exercises_with_details = workout_exercises::table
                .filter(workout_exercises::workout_id.eq(workout.id))
                .inner_join(exercises::table)
                .select((workout_exercises::all_columns, exercises::all_columns))
                .load::<(WorkoutExercise, Exercise)>(connection)
                .unwrap_or_default()
                .into_iter()
                .map(|(workout_exercise, exercise)| WorkoutExerciseWithDetails {
                    workout_exercise,
                    exercise,
                })
                .collect();

            WorkoutWithExercises {
                workout,
                exercises: workout_exercises_with_details,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": workouts_with_exercises
    })))
}

pub async fn get_workout(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let workout_id = id.into_inner();

    let workout = workouts::table
        .find(workout_id)
        .first::<Workout>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let workout_exercises_with_details = workout_exercises::table
        .filter(workout_exercises::workout_id.eq(workout_id))
        .inner_join(exercises::table)
        .select((workout_exercises::all_columns, exercises::all_columns))
        .load::<(WorkoutExercise, Exercise)>(&mut connection)
        .unwrap_or_default()
        .into_iter()
        .map(|(workout_exercise, exercise)| WorkoutExerciseWithDetails {
            workout_exercise,
            exercise,
        })
        .collect();

    let workout_with_exercises = WorkoutWithExercises {
        workout,
        exercises: workout_exercises_with_details,
    };

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": workout_with_exercises
    })))
}

pub async fn update_workout(id: web::Path<i32>, update_workout: web::Json<UpdateWorkout>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let workout_id = id.into_inner();

    let workout = diesel::update(workouts::table.find(workout_id))
        .set(&update_workout.into_inner())
        .get_result::<Workout>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": workout
    })))
}

pub async fn delete_workout(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let workout_id = id.into_inner();

    diesel::delete(workouts::table.find(workout_id))
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Workout deleted successfully".to_string(),
    }))
}

// Workout Exercise handlers
pub async fn add_workout_exercise(new_exercise: web::Json<NewWorkoutExercise>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(workout_exercises::table)
        .values(&new_exercise.into_inner())
        .get_result::<WorkoutExercise>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn delete_workout_exercise(params: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let id = params.into_inner();

    diesel::delete(
        workout_exercises::table
        .find(id)
    )
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Workout exercise deleted successfully".to_string(),
    }))
}