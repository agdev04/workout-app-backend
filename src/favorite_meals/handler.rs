use actix_web::{delete, get, post, web, HttpResponse, Result};
use serde_json::json;
use diesel::prelude::*;
use crate::{db::establish_connection, schema::favorite_meals};
use crate::favorite_meals::model::{FavoriteMeal, NewFavoriteMeal};

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[post("")]
pub async fn add_favorite(new_favorite: web::Json<NewFavoriteMeal>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(favorite_meals::table)
        .values(&new_favorite.into_inner())
        .get_result::<FavoriteMeal>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e));

    match result {
        Ok(favorite) => Ok(HttpResponse::Ok().json(json!({
            "status": "success",
            "data": favorite
        }))),
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to add favorite meal".to_string(),
        }))
    }
}

#[delete("/{id}")]
pub async fn remove_favorite(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let meal_id = id.into_inner();

    let result = diesel::delete(favorite_meals::table.find(meal_id))
        .execute(&mut connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Favorite meal removed successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Favorite meal not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to remove favorite meal".to_string(),
        }))
    }
}

#[get("/user/{user_id}")]
pub async fn get_user_favorites(user_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    
    let results = favorite_meals::table
        .filter(favorite_meals::user_id.eq(user_id.into_inner()))
        .load::<FavoriteMeal>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": results
    })))
}