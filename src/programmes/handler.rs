use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::{db::establish_connection, schema::{programmes, programme_weeks, programme_days, exercises}};
use diesel::prelude::*;
use crate::programmes::model::*;
use crate::exercises::model::Exercise;

#[derive(serde::Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, serde::Serialize)]
pub struct ProgrammeWithDetails {
    #[serde(flatten)]
    pub programme: Programme,
    pub week_list: Vec<ProgrammeWeekWithDays>,
}

#[derive(Debug, serde::Serialize)]
pub struct ProgrammeWeekWithDays {
    #[serde(flatten)]
    pub week: ProgrammeWeek,
    pub days: Vec<ProgrammeDayWithExercise>,
}

#[derive(Debug, serde::Serialize)]
pub struct ProgrammeDayWithExercise {
    #[serde(flatten)]
    pub day: ProgrammeDay,
    pub exercise: Exercise,
}

// Programme CRUD handlers
pub async fn create_programme(new_programme: web::Json<NewProgramme>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(programmes::table)
        .values(&new_programme.into_inner())
        .get_result::<Programme>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn get_programme_list() -> Result<HttpResponse> {
    let connection = &mut establish_connection();
    
    let programmes_list = programmes::table
        .load::<Programme>(connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let programmes_with_details: Vec<ProgrammeWithDetails> = programmes_list
        .into_iter()
        .map(|programme| {
            let week_list = programme_weeks::table
                .filter(programme_weeks::programme_id.eq(programme.id))
                .load::<ProgrammeWeek>(connection)
                .unwrap_or_default()
                .into_iter()
                .map(|week| {
                    let days_with_exercises = programme_days::table
                        .filter(programme_days::programme_week_id.eq(week.id))
                        .inner_join(exercises::table)
                        .select((programme_days::all_columns, exercises::all_columns))
                        .load::<(ProgrammeDay, Exercise)>(connection)
                        .unwrap_or_default()
                        .into_iter()
                        .map(|(day, exercise)| ProgrammeDayWithExercise {
                            day,
                            exercise,
                        })
                        .collect();

                    ProgrammeWeekWithDays {
                        week,
                        days: days_with_exercises,
                    }
                })
                .collect();

            ProgrammeWithDetails {
                programme,
                week_list,
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": programmes_with_details
    })))
}

pub async fn get_programme(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let programme_id = id.into_inner();

    let programme = programmes::table
        .find(programme_id)
        .first::<Programme>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let week_list = programme_weeks::table
        .filter(programme_weeks::programme_id.eq(programme_id))
        .load::<ProgrammeWeek>(&mut connection)
        .unwrap_or_default()
        .into_iter()
        .map(|week| {
            let days_with_exercises = programme_days::table
                .filter(programme_days::programme_week_id.eq(week.id))
                .inner_join(exercises::table)
                .select((programme_days::all_columns, exercises::all_columns))
                .load::<(ProgrammeDay, Exercise)>(&mut connection)
                .unwrap_or_default()
                .into_iter()
                .map(|(day, exercise)| ProgrammeDayWithExercise {
                    day,
                    exercise,
                })
                .collect();

            ProgrammeWeekWithDays {
                week,
                days: days_with_exercises,
            }
        })
        .collect();

    let programme_with_details = ProgrammeWithDetails {
        programme,
        week_list,
    };

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": programme_with_details
    })))
}

pub async fn update_programme(id: web::Path<i32>, update_programme: web::Json<UpdateProgramme>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let programme_id = id.into_inner();

    let programme = diesel::update(programmes::table.find(programme_id))
        .set(&update_programme.into_inner())
        .get_result::<Programme>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": programme
    })))
}

pub async fn delete_programme(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let programme_id = id.into_inner();

    diesel::delete(programmes::table.find(programme_id))
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Programme deleted successfully".to_string(),
    }))
}

// Programme Week handlers
pub async fn add_programme_week(new_week: web::Json<NewProgrammeWeek>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(programme_weeks::table)
        .values(&new_week.into_inner())
        .get_result::<ProgrammeWeek>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn delete_programme_week(params: web::Path<(i32, i32)>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let (programme_id, week_number) = params.into_inner();

    diesel::delete(
        programme_weeks::table
            .filter(programme_weeks::programme_id.eq(programme_id))
            .filter(programme_weeks::week_number.eq(week_number))
    )
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Programme week deleted successfully".to_string(),
    }))
}

// Programme Day handlers
pub async fn add_programme_day(new_day: web::Json<NewProgrammeDay>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(programme_days::table)
        .values(&new_day.into_inner())
        .get_result::<ProgrammeDay>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn delete_programme_day(params: web::Path<(i32, i32, i32)>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let (programme_week_id, day_number, exercise_id) = params.into_inner();

    diesel::delete(
        programme_days::table
            .filter(programme_days::programme_week_id.eq(programme_week_id))
            .filter(programme_days::day_number.eq(day_number))
            .filter(programme_days::exercise_id.eq(exercise_id))
    )
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Programme day deleted successfully".to_string(),
    }))
}