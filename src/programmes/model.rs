use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::programmes)]
pub struct Programme {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub total_weeks: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::programmes)]
pub struct NewProgramme {
    pub name: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub total_weeks: i32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::programmes)]
pub struct UpdateProgramme {
    pub name: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub total_weeks: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::programme_weeks)]
pub struct ProgrammeWeek {
    pub id: i32,
    pub programme_id: i32,
    pub name: String,
    pub week_number: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::programme_weeks)]
pub struct NewProgrammeWeek {
    pub programme_id: i32,
    pub name: String,
    pub week_number: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::programme_days_exercises)]
pub struct ProgrammeDayExercise {
    pub id: i32,
    pub programme_week_id: i32,
    pub exercise_id: i32,
    pub day_number: i32,
    pub position: i32,
    pub reps: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub rest_seconds: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::programme_days_exercises)]
pub struct NewProgrammeDayExercise {
    pub programme_week_id: i32,
    pub exercise_id: i32,
    pub day_number: i32,
    pub position: i32,
    pub reps: Option<i32>,
    pub duration_seconds: Option<i32>,
    pub rest_seconds: i32,
}