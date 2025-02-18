use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::{db::establish_connection, schema::categories};
use diesel::prelude::*;
use crate::categories::model::{Category, NewCategory, UpdateCategory};

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub async fn create_category(new_category: web::Json<NewCategory>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    diesel::insert_into(categories::table)
        .values(&new_category.into_inner())
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Category created successfully".to_string(),
    }))
}

pub async fn get_categories() -> Result<HttpResponse> {
    let connection = &mut establish_connection();
    let results = categories::table
        .load::<Category>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": results
    })))
}

pub async fn get_category(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let category_id = id.into_inner();
    let result = categories::table
        .find(category_id)
        .first::<Category>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(result))
}

pub async fn update_category(id: web::Path<i32>, category: web::Json<UpdateCategory>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let category_id = id.into_inner();
    let result = diesel::update(categories::table.find(category_id))
        .set(category.into_inner())
        .execute(&mut connection);

    match result {
        Ok(num_updated) => {
            if num_updated > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Category updated successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Category not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to update category".to_string(),
        }))
    }
}

pub async fn delete_category(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let category_id = id.into_inner();
    let result = diesel::delete(categories::table.find(category_id))
        .execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Category deleted successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Category not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to delete category".to_string(),
        }))
    }
}