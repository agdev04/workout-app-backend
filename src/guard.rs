

use actix_web::{middleware::from_fn, web};
use crate::{body_parts::body_part_config, categories::category_config, equipment::equipment_config, exercises::exercises_config, meals::meals_config, programmes::programmes_config, statistics::statistics_config, upload::route::upload_config, users::route::user_config, workouts::workouts_config};

use std::env;

use actix_web::{
  body::MessageBody, dev::{ServiceRequest, ServiceResponse}, middleware::Next, Error, HttpMessage,
};
use actix_web::error::ErrorUnauthorized;
use jsonwebtoken::{errors::ErrorKind, Algorithm, Validation, decode, DecodingKey};
use crate::users::handler::Claims;
use crate::config::get_auth_setup;
use crate::{db::establish_connection, schema::users, users::model::User};
use diesel::prelude::*;

async fn get_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {

    match get_auth_setup().as_str() {
        "http_only" => {
          let cookie = req.cookie("jwt");

          match cookie {
              Some(cookie) => {
                  let key = env::var("AUTH_TOKEN").unwrap_or_else(|_| "secret".to_string());
                  let key = key.as_bytes();
                  let token = cookie.value();
                  let mut validation = Validation::new(Algorithm::HS256);
                  validation.sub = Some("someone".to_string());
                  validation.set_required_spec_claims(&["exp", "sub"]);

                  match decode::<Claims>(token, &DecodingKey::from_secret(key), &validation) {
                      Ok(token_data) => {
                        let mut connection = establish_connection();
                        let user_id: i32 = token_data.claims.company.parse().unwrap_or(0);
                        let user = users::table.find(user_id).first::<User>(&mut connection);
                        
                        match user {
                            Ok(user) => {
                                if user.role != "admin" || user.status != "active" {
                                    return Err(ErrorUnauthorized("Insufficient permissions"));
                                }
                                req.extensions_mut().insert(token_data.claims.company.clone());
                                next.call(req).await
                            }
                            Err(_) => Err(ErrorUnauthorized("User not found"))
                        }
                      }
                      Err(err) => match *err.kind() {
                          ErrorKind::InvalidToken => Err(ErrorUnauthorized("Invalid token")),
                          ErrorKind::InvalidIssuer => Err(ErrorUnauthorized("Invalid issuer")),
                          _ => Err(ErrorUnauthorized("Authentication failed")),
                      },
                  }
              }
              None => Err(ErrorUnauthorized("No authorization header")),
          }
        }
        _ => {
          let auth_header = req.headers().get("Authorization");

          match auth_header {
              Some(auth_value) => {
                  let auth_str = auth_value.to_str().map_err(|_| ErrorUnauthorized("Invalid authorization header"))?;
                  if !auth_str.starts_with("Bearer ") {
                      return Err(ErrorUnauthorized("Invalid authorization header format"));
                  }
                  let token = &auth_str[7..];

                  let key = env::var("AUTH_TOKEN").unwrap_or_else(|_| "secret".to_string());
                  let key = key.as_bytes();

                  let mut validation = Validation::new(Algorithm::HS256);
                  validation.sub = Some("someone".to_string());
                  validation.set_required_spec_claims(&["exp", "sub"]);

                  match decode::<Claims>(token, &DecodingKey::from_secret(key), &validation) {
                      Ok(token_data) => {
                          let mut connection = establish_connection();
                          let user_id: i32 = token_data.claims.company.parse().unwrap_or(0);
                          let user = users::table.find(user_id).first::<User>(&mut connection);
                          
                          match user {
                              Ok(user) => {
                                  let path = req.path();
                                  let is_special_module = path.starts_with("/api/workout-progress") ||
                                      path.starts_with("/programme-progress") ||
                                      path.starts_with("/favorite-meals") ||
                                      path.starts_with("/favorite-workouts") ||
                                      path.starts_with("/meal-plans");

                                  if is_special_module {
                                      if user.status != "active" {
                                          return Err(ErrorUnauthorized("User account is not active"));
                                      }
                                  } else if user.role != "admin" || user.status != "active" {
                                      return Err(ErrorUnauthorized("Insufficient permissions"));
                                  }
                                  req.extensions_mut().insert(token_data.claims.company.clone());
                                  next.call(req).await
                              }
                              Err(_) => Err(ErrorUnauthorized("User not found"))
                          }
                      }
                      Err(err) => match *err.kind() {
                          ErrorKind::InvalidToken => Err(ErrorUnauthorized("Invalid token")),
                          ErrorKind::InvalidIssuer => Err(ErrorUnauthorized("Invalid issuer")),
                          _ => Err(ErrorUnauthorized("Authentication failed")),
                      },
                  }
              }
              None => Err(ErrorUnauthorized("No authorization header")),
          }
        },
    }
}

pub fn guard_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::scope("")
    .wrap(from_fn(get_middleware))
    .configure(user_config)
    .configure(upload_config)
    .configure(category_config)
    .configure(body_part_config)
    .configure(equipment_config)
    .configure(exercises_config)
    .configure(meals_config)
    .configure(workouts_config)
    .configure(programmes_config)
    .configure(statistics_config)
  );
}
 