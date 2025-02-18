use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::programmes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Programme {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub weeks: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::programmes)]
pub struct NewProgramme {
    pub name: String,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub weeks: i32,
}

#[derive(Debug, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::programmes)]
pub struct UpdateProgramme {
    pub name: Option<String>,
    pub description: Option<String>,
    pub thumbnail_url: Option<String>,
    pub weeks: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::programme_weeks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProgrammeWeek {
    pub id: i32,
    pub programme_id: i32,
    pub week_number: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::programme_weeks)]
pub struct NewProgrammeWeek {
    pub programme_id: i32,
    pub week_number: i32,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::programme_days)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProgrammeDay {
    pub id: i32,
    pub programme_week_id: i32,
    pub day_number: i32,
    pub exercise_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::programme_days)]
pub struct NewProgrammeDay {
    pub programme_week_id: i32,
    pub day_number: i32,
    pub exercise_id: i32,
}