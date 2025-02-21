use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::workout_progress)]
pub struct WorkoutProgress {
    pub id: i32,
    pub user_id: i32,
    pub workout_id: i32,
    pub exercise_id: i32,
    pub workout_exercise_id: i32,
    pub completed: bool,
    pub actual_reps: Option<i32>,
    pub actual_duration_seconds: Option<i32>,
    pub notes: Option<String>,
    pub completed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::workout_progress)]
pub struct NewWorkoutProgress {
    pub user_id: i32,
    pub workout_id: i32,
    pub exercise_id: i32,
    pub workout_exercise_id: i32,
    pub completed: bool,
    pub actual_reps: Option<i32>,
    pub actual_duration_seconds: Option<i32>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::workout_progress)]
pub struct UpdateWorkoutProgress {
    pub completed: Option<bool>,
    pub actual_reps: Option<i32>,
    pub actual_duration_seconds: Option<i32>,
    pub notes: Option<String>,
}