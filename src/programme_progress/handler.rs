use actix_web::{web, HttpResponse, Result};
use chrono::Utc;
use diesel::prelude::*;
use serde_json::json;

use crate::db::establish_connection;
use crate::schema::programme_progress;
use crate::programme_progress::model::{NewProgrammeProgress, ProgrammeProgress, UpdateProgrammeProgress};

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub async fn create_progress(new_progress: web::Json<NewProgrammeProgress>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(programme_progress::table)
        .values(&new_progress.into_inner())
        .get_result::<ProgrammeProgress>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

use crate::schema::{programme_weeks, exercises, programmes};
use crate::exercises::model::Exercise;
use crate::programmes::model::{ProgrammeWeek, Programme};

#[derive(Debug, serde::Serialize)]
pub struct ProgrammeProgressWithDetails {
    #[serde(flatten)]
    pub progress: ProgrammeProgress,
    pub exercise: Exercise,
    pub week: ProgrammeWeek,
    pub programme: Programme,
}

pub async fn get_user_programme_progress(path: web::Path<(i32, i32)>) -> Result<HttpResponse> {
    let (user_id, programme_id) = path.into_inner();
    let mut connection = establish_connection();

    let progress_with_details = programme_progress::table
        .inner_join(exercises::table.on(programme_progress::exercise_id.eq(exercises::id)))
        .inner_join(programme_weeks::table.on(programme_progress::programme_week_id.eq(programme_weeks::id)))
        .inner_join(programmes::table.on(programme_progress::programme_id.eq(programmes::id)))
        .filter(programme_progress::user_id.eq(user_id))
        .filter(programme_progress::programme_id.eq(programme_id))
        .select((
            programme_progress::all_columns,
            exercises::all_columns,
            programme_weeks::all_columns,
            programmes::all_columns
        ))
        .load::<(ProgrammeProgress, Exercise, ProgrammeWeek, Programme)>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let result: Vec<ProgrammeProgressWithDetails> = progress_with_details
        .into_iter()
        .map(|(progress, exercise, week, programme)| ProgrammeProgressWithDetails {
            progress,
            exercise,
            week,
            programme,
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn update_progress(id: web::Path<i32>, update_data: web::Json<UpdateProgrammeProgress>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let progress_id = id.into_inner();
    
    let mut update_data = update_data.into_inner();
    if update_data.completed.unwrap_or(false) {
        update_data.completed_at = Some(Utc::now().naive_utc());
    }

    let result = diesel::update(programme_progress::table.find(progress_id))
        .set(update_data)
        .execute(&mut connection);

    match result {
        Ok(num_updated) => {
            if num_updated > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Progress updated successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Progress not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to update progress".to_string(),
        }))
    }
}

pub async fn delete_progress(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let progress_id = id.into_inner();
    let result = diesel::delete(programme_progress::table.find(progress_id))
        .execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Progress deleted successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Progress not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to delete progress".to_string(),
        }))
    }
}
