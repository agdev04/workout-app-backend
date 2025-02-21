use crate::schema::favorite_meals;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = favorite_meals)]
pub struct FavoriteMeal {
    pub id: i32,
    pub user_id: i32,
    pub meal_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = favorite_meals)]
pub struct NewFavoriteMeal {
    pub user_id: i32,
    pub meal_id: i32,
}