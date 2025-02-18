use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable)]
#[diesel(table_name = crate::schema::body_parts)]
pub struct BodyPart {
    pub id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::body_parts)]
pub struct NewBodyPart {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::schema::body_parts)]
pub struct UpdateBodyPart {
    pub name: Option<String>,
}