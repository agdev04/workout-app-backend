use actix_web::{web, HttpResponse, Result};
use serde_json::json;
use crate::{db::establish_connection, schema::{programmes, programme_weeks, programme_days_exercises, exercises}};
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
    pub weeks: Vec<ProgrammeWeekWithExercises>,
}

#[derive(Debug, serde::Serialize)]
pub struct ProgrammeWeekWithExercises {
    #[serde(flatten)]
    pub week: ProgrammeWeek,
    pub exercises_by_day: Vec<DayExercises>,
}

#[derive(Debug, serde::Serialize)]
pub struct DayExercises {
    pub day_number: i32,
    pub exercises: Vec<ProgrammeDayExerciseWithDetails>,
}

#[derive(Debug, serde::Serialize)]
pub struct ProgrammeDayExerciseWithDetails {
    #[serde(flatten)]
    pub programme_exercise: ProgrammeDayExercise,
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
            let weeks = programme_weeks::table
                .filter(programme_weeks::programme_id.eq(programme.id))
                .load::<ProgrammeWeek>(connection)
                .unwrap_or_default();

            let weeks_with_exercises = weeks
                .into_iter()
                .map(|week| {
                    let exercises = programme_days_exercises::table
                        .filter(programme_days_exercises::programme_week_id.eq(week.id))
                        .inner_join(exercises::table)
                        .select((programme_days_exercises::all_columns, exercises::all_columns))
                        .load::<(ProgrammeDayExercise, Exercise)>(connection)
                        .unwrap_or_default();

                    let exercises_by_day = exercises
                        .into_iter()
                        .fold(Vec::new(), |mut acc, (programme_exercise, exercise)| {
                            let day_number = programme_exercise.day_number;
                            if let Some(day) = acc.iter_mut().find(|d: &&mut DayExercises| d.day_number == day_number) {
                                day.exercises.push(ProgrammeDayExerciseWithDetails {
                                    programme_exercise,
                                    exercise,
                                });
                            } else {
                                acc.push(DayExercises {
                                    day_number,
                                    exercises: vec![ProgrammeDayExerciseWithDetails {
                                        programme_exercise,
                                        exercise,
                                    }],
                                });
                            }
                            acc
                        });

                    ProgrammeWeekWithExercises {
                        week,
                        exercises_by_day,
                    }
                })
                .collect();

            ProgrammeWithDetails {
                programme,
                weeks: weeks_with_exercises,
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

    let weeks = programme_weeks::table
        .filter(programme_weeks::programme_id.eq(programme_id))
        .load::<ProgrammeWeek>(&mut connection)
        .unwrap_or_default();

    let weeks_with_exercises = weeks
        .into_iter()
        .map(|week| {
            let exercises = programme_days_exercises::table
                .filter(programme_days_exercises::programme_week_id.eq(week.id))
                .inner_join(exercises::table)
                .select((programme_days_exercises::all_columns, exercises::all_columns))
                .load::<(ProgrammeDayExercise, Exercise)>(&mut connection)
                .unwrap_or_default();

            let exercises_by_day = exercises
                .into_iter()
                .fold(Vec::new(), |mut acc, (programme_exercise, exercise)| {
                    let day_number = programme_exercise.day_number;
                    if let Some(day) = acc.iter_mut().find(|d: &&mut DayExercises| d.day_number == day_number) {
                        day.exercises.push(ProgrammeDayExerciseWithDetails {
                            programme_exercise,
                            exercise,
                        });
                    } else {
                        acc.push(DayExercises {
                            day_number,
                            exercises: vec![ProgrammeDayExerciseWithDetails {
                                programme_exercise,
                                exercise,
                            }],
                        });
                    }
                    acc
                });

            ProgrammeWeekWithExercises {
                week,
                exercises_by_day,
            }
        })
        .collect();

    let programme_with_details = ProgrammeWithDetails {
        programme,
        weeks: weeks_with_exercises,
    };

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": programme_with_details
    })))
}

pub async fn update_programme(id: web::Path<i32>, programme_update: web::Json<UpdateProgramme>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let programme_id = id.into_inner();

    let programme = diesel::update(programmes::table.find(programme_id))
        .set(&programme_update.into_inner())
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

pub async fn delete_programme_week(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let week_id = id.into_inner();

    diesel::delete(programme_weeks::table.find(week_id))
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Programme week deleted successfully".to_string(),
    }))
}

// Programme Day Exercise handlers
pub async fn add_programme_day_exercise(new_exercise: web::Json<NewProgrammeDayExercise>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    let result = diesel::insert_into(programme_days_exercises::table)
        .values(&new_exercise.into_inner())
        .get_result::<ProgrammeDayExercise>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": result
    })))
}

pub async fn delete_programme_day_exercise(id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let exercise_id = id.into_inner();

    diesel::delete(programme_days_exercises::table.find(exercise_id))
        .execute(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(GenericResponse {
        status: "success".to_string(),
        message: "Programme day exercise deleted successfully".to_string(),
    }))
}