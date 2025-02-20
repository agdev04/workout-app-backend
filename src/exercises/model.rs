use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::exercises)]
pub struct Exercise {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::exercises)]
pub struct NewExercise {
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::exercises)]
pub struct UpdateExercise {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub thumbnail_url: Option<String>,
    pub video_url: Option<String>,
}

// Exercise relationship models
#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::exercise_body_parts)]
pub struct ExerciseBodyPart {
    pub id: i32,
    pub exercise_id: i32,
    pub body_part_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::exercise_body_parts)]
pub struct NewExerciseBodyPart {
    pub exercise_id: i32,
    pub body_part_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::exercise_categories)]
pub struct ExerciseCategory {
    pub id: i32,
    pub exercise_id: i32,
    pub category_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::exercise_categories)]
pub struct NewExerciseCategory {
    pub exercise_id: i32,
    pub category_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::exercise_equipment)]
pub struct ExerciseEquipment {
    pub id: i32,
    pub exercise_id: i32,
    pub equipment_id: i32,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::exercise_equipment)]
pub struct NewExerciseEquipment {
    pub exercise_id: i32,
    pub equipment_id: i32,
}