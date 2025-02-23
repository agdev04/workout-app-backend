use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde::Deserialize;

use crate::db::establish_connection;
use crate::schema::users;
use crate::users::model::{UpdateUser, User};
use crate::users::handler::hash_password;

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

pub async fn update_me(user_id: web::ReqData<String>, user: web::Json<UpdateUser>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let user_id: i32 = user_id.parse().unwrap_or(0);

    let result = diesel::update(users::table.find(user_id))
        .set(user.into_inner())
        .execute(&mut connection);

    match result {
        Ok(num_updated) => {
            if num_updated > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Profile updated successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "User not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to update profile".to_string(),
        }))
    }
}

pub async fn reset_password(user_id: web::ReqData<String>, req: web::Json<ResetPasswordRequest>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let user_id: i32 = user_id.parse().unwrap_or(0);

    // Get the user to verify current password
    let _user = match users::table.find(user_id).first::<User>(&mut connection) {
        Ok(user) => user,
        Err(_) => return Ok(HttpResponse::NotFound().json(GenericResponse {
            status: "error".to_string(),
            message: "User not found".to_string(),
        }))
    };

    // Hash the new password
    let hashed_password = match hash_password(req.new_password.clone()) {
        Ok(hashed) => hashed,
        Err(_) => return Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to process new password".to_string(),
        }))
    };

    // Update the password
    let result = diesel::update(users::table.find(user_id))
        .set(users::password.eq(hashed_password))
        .execute(&mut connection);

    match result {
        Ok(num_updated) => {
            if num_updated > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Password updated successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "User not found".to_string(),
                }))
            }
        },
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to update password".to_string(),
        }))
    }
}



