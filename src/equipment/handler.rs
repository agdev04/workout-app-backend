use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::{db::establish_connection, schema::equipment};
use diesel::prelude::*;
use crate::equipment::model::{Equipment, NewEquipment, UpdateEquipment};

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub async fn create_equipment(new_equipment: web::Json<NewEquipment>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    diesel::insert_into(equipment::table)
        .values(&new_equipment.into_inner())
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Equipment created successfully".to_string(),
    }))
}

pub async fn get_equipment_list() -> Result<HttpResponse> {
    let connection = &mut establish_connection();
    let results = equipment::table
        .load::<Equipment>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": results
    })))
}

pub async fn get_equipment(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let equipment_id = id.into_inner();
    let result = equipment::table
        .find(equipment_id)
        .first::<Equipment>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_equipment(id: web::Path<i32>, equipment: web::Json<UpdateEquipment>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let equipment_id = id.into_inner();
    let result = diesel::update(equipment::table.find(equipment_id))
        .set(equipment.into_inner())
        .execute(&mut connection);

    match result {
        Ok(num_updated) => {
            if num_updated > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Equipment updated successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Equipment not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to update equipment".to_string(),
        }))
    }
}

pub async fn delete_equipment(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let equipment_id = id.into_inner();
    let result = diesel::delete(equipment::table.find(equipment_id))
        .execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Equipment deleted successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Equipment not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to delete equipment".to_string(),
        }))
    }
}