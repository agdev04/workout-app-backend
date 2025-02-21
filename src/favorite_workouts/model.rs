use crate::schema::favorite_workouts;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = favorite_workouts)]
pub struct FavoriteWorkout {
    pub id: i32,
    pub user_id: i32,
    pub workout_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = favorite_workouts)]
pub struct NewFavoriteWorkout {
    pub user_id: i32,
    pub workout_id: i32,
}