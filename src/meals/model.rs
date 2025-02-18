use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::meals)]
pub struct Meal {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::meals)]
pub struct NewMeal {
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::meals)]
pub struct UpdateMeal {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

// Meal Ingredients models
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::meal_ingredients)]
pub struct MealIngredient {
    pub id: i32,
    pub meal_id: i32,
    pub name: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::meal_ingredients)]
pub struct NewMealIngredient {
    pub meal_id: i32,
    pub name: String,
    pub amount: String,
}

// Meal Instructions models
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::meal_instructions)]
pub struct MealInstruction {
    pub id: i32,
    pub meal_id: i32,
    pub step_number: i32,
    pub instruction: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::meal_instructions)]
pub struct NewMealInstruction {
    pub meal_id: i32,
    pub step_number: i32,
    pub instruction: String,
}