use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::{db::establish_connection, schema::{exercises, categories, equipment, body_parts, exercise_body_parts, exercise_categories, exercise_equipment}};
use diesel::prelude::*;
use crate::exercises::model::*;

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

// Exercise CRUD handlers
pub async fn create_exercise(new_exercise: web::Json<NewExercise>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(exercises::table)
        .values(&new_exercise.into_inner())
        .get_result::<Exercise>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

#[derive(Debug, serde::Serialize, Queryable)]
pub struct SubData {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ExerciseWithRelations {
    #[serde(flatten)]
    pub exercise: Exercise,
    pub body_parts: Vec<SubData>,
    pub categories: Vec<SubData>,
    pub equipment: Vec<SubData>,
}

pub async fn get_exercise_list() -> Result<HttpResponse> {
    let connection = &mut establish_connection();
    
    // Get all exercises
    let exercises = exercises::table
        .load::<Exercise>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // For each exercise, get its related data
    let exercises_with_relations: Vec<ExerciseWithRelations> = exercises
        .into_iter()
        .map(|exercise| {
            let body_part_list: Vec<SubData> = exercise_body_parts::table
                .inner_join(body_parts::table)
                .filter(exercise_body_parts::exercise_id.eq(exercise.id))
                .select((exercise_body_parts::body_part_id, body_parts::name))
                .load(connection)
                .unwrap_or_default();

            let category_list: Vec<SubData> = exercise_categories::table
                .inner_join(categories::table)
                .filter(exercise_categories::exercise_id.eq(exercise.id))
                .select((exercise_categories::category_id, categories::name))
                .load(connection)
                .unwrap_or_default();

            let equipment_list: Vec<SubData> = exercise_equipment::table
                .inner_join(equipment::table)
                .filter(exercise_equipment::exercise_id.eq(exercise.id))
                .select((exercise_equipment::equipment_id, equipment::name))
                .load(connection)
                .unwrap_or_default();

            ExerciseWithRelations {
                exercise,
                body_parts: body_part_list,
                categories: category_list,
                equipment: equipment_list,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": exercises_with_relations
    })))
}

pub async fn get_exercise(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let exercise_id = id.into_inner();
    let result = exercises::table
        .find(exercise_id)
        .first::<Exercise>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_exercise(id: web::Path<i32>, exercise: web::Json<UpdateExercise>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let exercise_id = id.into_inner();
    let result = diesel::update(exercises::table.find(exercise_id))
        .set(exercise.into_inner())
        .execute(&mut connection);

    match result {
        Ok(num_updated) => {
            if num_updated > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Exercise updated successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Exercise not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to update exercise".to_string(),
        }))
    }
}

pub async fn delete_exercise(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let exercise_id = id.into_inner();
    let result = diesel::delete(exercises::table.find(exercise_id))
        .execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Exercise deleted successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Exercise not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to delete exercise".to_string(),
        }))
    }
}

// Exercise Body Parts handlers
pub async fn add_exercise_body_part(new_relation: web::Json<NewExerciseBodyPart>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    diesel::insert_into(exercise_body_parts::table)
        .values(&new_relation.into_inner())
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Body part added to exercise successfully".to_string(),
    }))
}

pub async fn remove_exercise_body_part(params: web::Path<(i32, i32)>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let (exercise_id, body_part_id) = params.into_inner();

    let result = diesel::delete(
        exercise_body_parts::table
            .filter(exercise_body_parts::exercise_id.eq(exercise_id))
            .filter(exercise_body_parts::body_part_id.eq(body_part_id))
    ).execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Body part removed from exercise successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Relationship not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to remove body part from exercise".to_string(),
        }))
    }
}

// Exercise Categories handlers
pub async fn add_exercise_category(new_relation: web::Json<NewExerciseCategory>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    diesel::insert_into(exercise_categories::table)
        .values(&new_relation.into_inner())
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Category added to exercise successfully".to_string(),
    }))
}

pub async fn remove_exercise_category(params: web::Path<(i32, i32)>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let (exercise_id, category_id) = params.into_inner();

    let result = diesel::delete(
        exercise_categories::table
            .filter(exercise_categories::exercise_id.eq(exercise_id))
            .filter(exercise_categories::category_id.eq(category_id))
    ).execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Category removed from exercise successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Relationship not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to remove category from exercise".to_string(),
        }))
    }
}

// Exercise Equipment handlers
pub async fn add_exercise_equipment(new_relation: web::Json<NewExerciseEquipment>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    diesel::insert_into(exercise_equipment::table)
        .values(&new_relation.into_inner())
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Equipment added to exercise successfully".to_string(),
    }))
}

pub async fn remove_exercise_equipment(params: web::Path<(i32, i32)>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let (exercise_id, equipment_id) = params.into_inner();

    let result = diesel::delete(
        exercise_equipment::table
            .filter(exercise_equipment::exercise_id.eq(exercise_id))
            .filter(exercise_equipment::equipment_id.eq(equipment_id))
    ).execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Equipment removed from exercise successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Relationship not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to remove equipment from exercise".to_string(),
        }))
    }
}