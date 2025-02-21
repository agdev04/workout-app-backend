use crate::schema::programme_progress;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = programme_progress)]
pub struct ProgrammeProgress {
    pub id: i32,
    pub user_id: i32,
    pub programme_id: i32,
    pub programme_week_id: i32,
    pub exercise_id: i32,
    pub day_number: i32,
    pub completed: bool,
    pub actual_reps: Option<i32>,
    pub actual_duration_seconds: Option<i32>,
    pub actual_rest_seconds: Option<i32>,
    pub notes: Option<String>,
    pub completed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = programme_progress)]
pub struct NewProgrammeProgress {
    pub user_id: i32,
    pub programme_id: i32,
    pub programme_week_id: i32,
    pub exercise_id: i32,
    pub day_number: i32,
    pub completed: bool,
    pub actual_reps: Option<i32>,
    pub actual_duration_seconds: Option<i32>,
    pub actual_rest_seconds: Option<i32>,
    pub notes: Option<String>,
    pub completed_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = programme_progress)]
pub struct UpdateProgrammeProgress {
    pub completed: Option<bool>,
    pub actual_reps: Option<i32>,
    pub actual_duration_seconds: Option<i32>,
    pub actual_rest_seconds: Option<i32>,
    pub notes: Option<String>,
    pub completed_at: Option<NaiveDateTime>,
}