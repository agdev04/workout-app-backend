use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use diesel::prelude::*;
use crate::{db::establish_connection, schema::{favorite_meals, meals, users}};
use crate::favorite_meals::model::{FavoriteMeal, NewFavoriteMeal};
use crate::meals::model::Meal;
use crate::users::model::User;

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct FavoriteMealWithDetails {
    #[serde(flatten)]
    pub favorite: FavoriteMeal,
    pub meal: Meal,
    pub user: User,
}

pub async fn add_favorite(new_favorite: web::Json<NewFavoriteMeal>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(favorite_meals::table)
        .values(&new_favorite.into_inner())
        .get_result::<FavoriteMeal>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(json!({
        "status": "success",
        "data": result
    })))
}


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

pub async fn get_user_favorites(user_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let user_id = user_id.into_inner();
    
    let favorites_with_details: Vec<FavoriteMealWithDetails> = favorite_meals::table
        .inner_join(meals::table)
        .inner_join(users::table)
        .filter(favorite_meals::user_id.eq(user_id))
        .select((
            favorite_meals::all_columns,
            meals::all_columns,
            users::all_columns,
        ))
        .load::<(FavoriteMeal, Meal, User)>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?
        .into_iter()
        .map(|(favorite, meal, user)| FavoriteMealWithDetails {
            favorite,
            meal,
            user,
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": favorites_with_details
    })))
}