use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use super::model::{MealPlan, MealPlanMeal, NewMealPlan, NewMealPlanMeal};
use crate::db::establish_connection;
use crate::meals::model::Meal;
use crate::schema::{meal_plan_meals, meal_plans, meals};

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct MealPlanWithMeals {
    #[serde(flatten)]
    pub meal_plan: MealPlan,
    pub meals: Vec<MealPlanMealWithDetails>,
}

#[derive(Debug, Serialize)]
pub struct MealPlanMealWithDetails {
    #[serde(flatten)]
    pub meal_plan_meal: MealPlanMeal,
    pub meal: Meal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMealPlanRequest {
    pub name: String,
    pub description: Option<String>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateMealPlanRequest {
    pub name: String,
    pub description: Option<String>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMealRequest {
    pub meal_id: i32,
    pub day_of_week: i32,
    pub meal_time: String,
    pub notes: Option<String>,
}

pub async fn get_meal_plans(user_id: web::ReqData<String>) -> Result<HttpResponse> {
    let user_id: i32 = user_id.parse().unwrap_or(0);
    let connection = &mut establish_connection();

    let meal_plans_list = meal_plans::table
        .filter(meal_plans::user_id.eq(user_id))
        .load::<MealPlan>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let meal_plans_with_meals: Vec<MealPlanWithMeals> = meal_plans_list
        .into_iter()
        .map(|meal_plan| {
            let meals = meal_plan_meals::table
                .filter(meal_plan_meals::meal_plan_id.eq(meal_plan.id))
                .inner_join(meals::table)
                .select((meal_plan_meals::all_columns, meals::all_columns))
                .load::<(MealPlanMeal, Meal)>(connection)
                .unwrap_or_default()
                .into_iter()
                .map(|(meal_plan_meal, meal)| MealPlanMealWithDetails {
                    meal_plan_meal,
                    meal,
                })
                .collect();

            MealPlanWithMeals { meal_plan, meals }
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": meal_plans_with_meals
    })))
}

pub async fn get_meal_plan(
    user_id: web::ReqData<String>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let meal_plan_id = path.into_inner();
    let user_id: i32 = user_id.parse().unwrap_or(0);
    let connection = &mut establish_connection();

    let meal_plan = meal_plans::table
        .filter(meal_plans::id.eq(meal_plan_id))
        .filter(meal_plans::user_id.eq(user_id))
        .first::<MealPlan>(connection)
        .optional()
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    match meal_plan {
        Some(meal_plan) => {
            let meals = meal_plan_meals::table
                .filter(meal_plan_meals::meal_plan_id.eq(meal_plan.id))
                .inner_join(meals::table)
                .select((meal_plan_meals::all_columns, meals::all_columns))
                .load::<(MealPlanMeal, Meal)>(connection)
                .unwrap_or_default()
                .into_iter()
                .map(|(meal_plan_meal, meal)| MealPlanMealWithDetails {
                    meal_plan_meal,
                    meal,
                })
                .collect();

            let meal_plan_with_meals = MealPlanWithMeals { meal_plan, meals };

            Ok(HttpResponse::Ok().json(json!({
                "status": "success",
                "data": meal_plan_with_meals
            })))
        }
        None => Ok(HttpResponse::NotFound().json(GenericResponse {
            status: "error".to_string(),
            message: "Meal plan not found".to_string(),
        })),
    }
}

pub async fn create_meal_plan(
    user_id: web::ReqData<String>,
    req: web::Json<CreateMealPlanRequest>,
) -> Result<HttpResponse> {
    let user_id: i32 = user_id.parse().unwrap_or(0);
    let connection = &mut establish_connection();

    let new_meal_plan = NewMealPlan {
        user_id,
        name: req.name.clone(),
        description: req.description.clone(),
        start_date: req.start_date,
        end_date: req.end_date,
    };

    let result = diesel::insert_into(meal_plans::table)
        .values(&new_meal_plan)
        .get_result::<MealPlan>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn update_meal_plan(
    user_id: web::ReqData<String>,
    path: web::Path<i32>,
    req: web::Json<UpdateMealPlanRequest>,
) -> Result<HttpResponse> {
    let meal_plan_id = path.into_inner();
    let user_id: i32 = user_id.parse().unwrap_or(0);
    let connection = &mut establish_connection();

    let result = diesel::update(meal_plans::table)
        .filter(meal_plans::id.eq(meal_plan_id))
        .filter(meal_plans::user_id.eq(user_id))
        .set((
            meal_plans::name.eq(req.name.clone()),
            meal_plans::description.eq(req.description.clone()),
            meal_plans::start_date.eq(req.start_date),
            meal_plans::end_date.eq(req.end_date),
            meal_plans::updated_at.eq(diesel::dsl::now),
        ))
        .execute(connection);

    match result {
        Ok(num_updated) => {
            if num_updated > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Meal plan updated successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Meal plan not found".to_string(),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to update meal plan".to_string(),
        })),
    }
}

pub async fn delete_meal_plan(
    user_id: web::ReqData<String>,
    path: web::Path<i32>,
) -> Result<HttpResponse> {
    let meal_plan_id = path.into_inner();
    let user_id: i32 = user_id.parse().unwrap_or(0);
    let connection = &mut establish_connection();

    let result = diesel::delete(
        meal_plans::table
            .filter(meal_plans::id.eq(meal_plan_id))
            .filter(meal_plans::user_id.eq(user_id)),
    )
    .execute(connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Meal plan deleted successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Meal plan not found".to_string(),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to delete meal plan".to_string(),
        })),
    }
}

pub async fn add_meal_to_plan(
    user_id: web::ReqData<String>,
    path: web::Path<i32>,
    req: web::Json<AddMealRequest>,
) -> Result<HttpResponse> {
    let meal_plan_id = path.into_inner();
    let user_id: i32 = user_id.parse().unwrap_or(0);
    let connection = &mut establish_connection();

    // Verify meal plan belongs to user
    let meal_plan_exists = meal_plans::table
        .filter(meal_plans::id.eq(meal_plan_id))
        .filter(meal_plans::user_id.eq(user_id))
        .count()
        .get_result::<i64>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if meal_plan_exists == 0 {
        return Ok(HttpResponse::NotFound().json(GenericResponse {
            status: "error".to_string(),
            message: "Meal plan not found".to_string(),
        }));
    }

    let new_meal = NewMealPlanMeal {
        meal_plan_id,
        meal_id: req.meal_id,
        day_of_week: req.day_of_week,
        meal_time: req.meal_time.clone(),
        notes: req.notes.clone(),
    };

    let result = diesel::insert_into(meal_plan_meals::table)
        .values(&new_meal)
        .get_result::<MealPlanMeal>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Created().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn delete_meal_from_plan(
    user_id: web::ReqData<String>,
    path: web::Path<(i32, i32)>,
) -> Result<HttpResponse> {
    let (meal_plan_id, meal_plan_meal_id) = path.into_inner();
    let user_id: i32 = user_id.parse().unwrap_or(0);
    let connection = &mut establish_connection();

    // Verify meal plan belongs to user
    let meal_plan_exists = meal_plans::table
        .filter(meal_plans::id.eq(meal_plan_id))
        .filter(meal_plans::user_id.eq(user_id))
        .count()
        .get_result::<i64>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    if meal_plan_exists == 0 {
        return Ok(HttpResponse::NotFound().json(GenericResponse {
            status: "error".to_string(),
            message: "Meal plan not found".to_string(),
        }));
    }

    let result = diesel::delete(
        meal_plan_meals::table
            .filter(meal_plan_meals::id.eq(meal_plan_meal_id))
            .filter(meal_plan_meals::meal_plan_id.eq(meal_plan_id)),
    )
    .execute(connection);

    match result {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                Ok(HttpResponse::Ok().json(GenericResponse {
                    status: "success".to_string(),
                    message: "Meal removed from plan successfully".to_string(),
                }))
            } else {
                Ok(HttpResponse::NotFound().json(GenericResponse {
                    status: "error".to_string(),
                    message: "Meal not found in plan".to_string(),
                }))
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError().json(GenericResponse {
            status: "error".to_string(),
            message: "Failed to remove meal from plan".to_string(),
        })),
    }
}
