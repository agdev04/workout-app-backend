use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::{db::establish_connection, schema::{meals, meal_ingredients, meal_instructions}};
use diesel::prelude::*;
use crate::meals::model::*;

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct MealWithRelations {
    #[serde(flatten)]
    pub meal: Meal,
    pub ingredients: Vec<MealIngredient>,
    pub instructions: Vec<MealInstruction>,
}

// Meal CRUD handlers
pub async fn create_meal(new_meal: web::Json<NewMeal>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(meals::table)
        .values(&new_meal.into_inner())
        .get_result::<Meal>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn get_meal_list() -> Result<HttpResponse> {
    let connection = &mut establish_connection();
    
    let meals_list = meals::table
        .load::<Meal>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let meals_with_relations: Vec<MealWithRelations> = meals_list
        .into_iter()
        .map(|meal| {
            let ingredients = meal_ingredients::table
                .filter(meal_ingredients::meal_id.eq(meal.id))
                .load::<MealIngredient>(connection)
                .unwrap_or_default();

            let instructions = meal_instructions::table
                .filter(meal_instructions::meal_id.eq(meal.id))
                .order(meal_instructions::step_number.asc())
                .load::<MealInstruction>(connection)
                .unwrap_or_default();

            MealWithRelations {
                meal,
                ingredients,
                instructions,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": meals_with_relations
    })))
}

pub async fn get_meal(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let meal_id = id.into_inner();

    let meal = meals::table
        .find(meal_id)
        .first::<Meal>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let ingredients = meal_ingredients::table
        .filter(meal_ingredients::meal_id.eq(meal_id))
        .load::<MealIngredient>(&mut connection)
        .unwrap_or_default();

    let instructions = meal_instructions::table
        .filter(meal_instructions::meal_id.eq(meal_id))
        .order(meal_instructions::step_number.asc())
        .load::<MealInstruction>(&mut connection)
        .unwrap_or_default();

    let meal_with_relations = MealWithRelations {
        meal,
        ingredients,
        instructions,
    };

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": meal_with_relations
    })))
}

pub async fn update_meal(id: web::Path<i32>, meal_data: web::Json<UpdateMeal>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let meal_id = id.into_inner();

    let result = diesel::update(meals::table.find(meal_id))
        .set(&meal_data.into_inner())
        .get_result::<Meal>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn delete_meal(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let meal_id = id.into_inner();

    diesel::delete(meals::table.find(meal_id))
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Meal deleted successfully".to_string(),
    }))
}

// Meal Ingredients handlers
pub async fn add_meal_ingredient(_meal_id: web::Path<i32>, new_ingredient: web::Json<NewMealIngredient>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(meal_ingredients::table)
        .values(&new_ingredient.into_inner())
        .get_result::<MealIngredient>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn delete_meal_ingredient(params: web::Path<(i32, i32)>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let (meal_id, ingredient_id) = params.into_inner();

    diesel::delete(
        meal_ingredients::table
            .filter(meal_ingredients::meal_id.eq(meal_id))
            .filter(meal_ingredients::id.eq(ingredient_id))
    )
    .execute(&mut connection)
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Ingredient removed successfully".to_string(),
    }))
}

// Meal Instructions handlers
pub async fn add_meal_instruction(_meal_id: web::Path<i32>, new_instruction: web::Json<NewMealInstruction>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(meal_instructions::table)
        .values(&new_instruction.into_inner())
        .get_result::<MealInstruction>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn delete_meal_instruction(params: web::Path<(i32, i32)>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let (meal_id, instruction_id) = params.into_inner();

    diesel::delete(
        meal_instructions::table
            .filter(meal_instructions::meal_id.eq(meal_id))
            .filter(meal_instructions::id.eq(instruction_id))
    )
    .execute(&mut connection)
    .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Instruction removed successfully".to_string(),
    }))
}