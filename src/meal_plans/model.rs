use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{meal_plans, meal_plan_meals};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = meal_plans)]
pub struct MealPlan {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = meal_plans)]
pub struct NewMealPlan {
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub start_date: chrono::NaiveDate,
    pub end_date: chrono::NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[diesel(belongs_to(MealPlan))]
#[diesel(table_name = meal_plan_meals)]
pub struct MealPlanMeal {
    pub id: i32,
    pub meal_plan_id: i32,
    pub meal_id: i32,
    pub day_of_week: i32,
    pub meal_time: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = meal_plan_meals)]
pub struct NewMealPlanMeal {
    pub meal_plan_id: i32,
    pub meal_id: i32,
    pub day_of_week: i32,
    pub meal_time: String,
    pub notes: Option<String>,
}