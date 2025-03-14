use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::meals)]

// Relationship to meal instructions is handled through foreign keys and associations
pub struct Meal {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub prep_time: String,
    pub servings: i32,
    pub calories: String,
    pub protein: String,
    pub fat: String,
    pub carbs: String,
    pub difficulty: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::meals)]
pub struct NewMeal {
    pub name: String,
    pub category: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub prep_time: String,
    pub servings: i32,
    pub calories: String,
    pub protein: String,
    pub fat: String,
    pub carbs: String,
    pub difficulty: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::meals)]
pub struct UpdateMeal {
    pub name: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub prep_time: Option<String>,
    pub servings: Option<i32>,
    pub calories: Option<String>,
    pub protein: Option<String>,
    pub fat: Option<String>,
    pub carbs: Option<String>,
    pub difficulty: Option<String>,
}

// Meal Ingredients models
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::meal_ingredients)]
#[diesel(belongs_to(Meal))] // Add relationship to parent meal
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
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(table_name = crate::schema::meal_instructions)]
#[diesel(belongs_to(Meal))] // Add relationship to parent meal
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
