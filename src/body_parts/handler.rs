use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::{db::establish_connection, schema::body_parts};
use diesel::prelude::*;
use crate::body_parts::model::{BodyPart, NewBodyPart, UpdateBodyPart};

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub async fn create_body_part(new_body_part: web::Json<NewBodyPart>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    diesel::insert_into(body_parts::table)
        .values(&new_body_part.into_inner())
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Body part created successfully".to_string(),
    }))
}

pub async fn get_body_parts() -> Result<HttpResponse> {
    let connection = &mut establish_connection();
    let results = body_parts::table
        .load::<BodyPart>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": results
    })))
}

pub async fn get_body_part(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let body_part_id = id.into_inner();
    let result = body_parts::table
        .find(body_part_id)
        .first::<BodyPart>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_body_part(id: web::Path<i32>, body_part: web::Json<UpdateBodyPart>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let body_part_id = id.into_inner();
    let result = diesel::update(body_parts::table.find(body_part_id))
        .set(body_part.into_inner())
        .execute(&mut connection);

    match result {
        Ok(num_updated) => {
            if num_updated > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Body part updated successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Body part not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to update body part".to_string(),
        }))
    }
}

pub async fn delete_body_part(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let body_part_id = id.into_inner();
    let result = diesel::delete(body_parts::table.find(body_part_id))
        .execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Body part deleted successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Body part not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to delete body part".to_string(),
        }))
    }
}