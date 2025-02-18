use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::workouts)]
pub struct Workout {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub difficulty: String,
    pub thumbnail_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::workouts)]
pub struct NewWorkout {
    pub name: String,
    pub description: Option<String>,
    pub difficulty: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::workouts)]
pub struct UpdateWorkout {
    pub name: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub thumbnail_url: Option<String>,
}

// Workout Exercises models
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::workout_exercises)]
pub struct WorkoutExercise {
    pub id: i32,
    pub workout_id: i32,
    pub exercise_id: i32,
    pub position: i32,
    pub reps: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub rest_seconds: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::workout_exercises)]
pub struct NewWorkoutExercise {
    pub workout_id: i32,
    pub exercise_id: i32,
    pub position: i32,
    pub reps: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub rest_seconds: Option<i32>,
}