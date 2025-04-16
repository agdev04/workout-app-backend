use super::model::{LeaderboardEntry, LeaderboardQuery, UserRanking};
use crate::db::establish_connection;
use crate::schema::{programme_progress, users, workout_progress};
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use serde_json::json;

pub async fn get_leaderboard(query: web::Query<LeaderboardQuery>) -> Result<HttpResponse> {
    let mut connection = establish_connection();

    // Get workout progress calories
    let workout_calories = workout_progress::table
        .inner_join(users::table)
        .group_by(users::id)
        .select((
            users::id,
            users::name,
            diesel::dsl::sql::<diesel::sql_types::BigInt>(
                "SUM(CASE WHEN burned_calories IS NOT NULL THEN CAST(burned_calories AS DECIMAL(10,2)) ELSE 0 END)",
            ),
        ))
        .load::<(i32, String, i64)>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Get programme progress calories
    let programme_calories = programme_progress::table
        .inner_join(users::table)
        .group_by(users::id)
        .select((
            users::id,
            users::name,
            diesel::dsl::sql::<diesel::sql_types::BigInt>(
                "SUM(CASE WHEN burned_calories IS NOT NULL THEN CAST(burned_calories AS DECIMAL(10,2)) ELSE 0 END)",
            ),
        ))
        .load::<(i32, String, i64)>(&mut connection)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Combine and calculate total calories
    let mut combined_stats: std::collections::HashMap<i32, (String, i64)> =
        std::collections::HashMap::new();

    for (user_id, username, calories) in workout_calories {
        combined_stats.insert(user_id, (username, calories));
    }

    for (user_id, username, calories) in programme_calories {
        combined_stats
            .entry(user_id)
            .and_modify(|(_, total_calories)| *total_calories += calories)
            .or_insert((username, calories));
    }

    // Convert to LeaderboardEntry and sort by calories
    let mut leaderboard: Vec<LeaderboardEntry> = combined_stats
        .into_iter()
        .map(|(user_id, (username, total_calories))| LeaderboardEntry {
            user_id,
            username,
            total_workouts: 0, // These could be calculated if needed
            total_time: 0,
            total_exercises: 0,
            rank: 0,
            total_calories: total_calories as f64,
        })
        .collect();

    leaderboard.sort_by(|a, b| b.total_calories.partial_cmp(&a.total_calories).unwrap());

    // Add ranks
    for (i, entry) in leaderboard.iter_mut().enumerate() {
        entry.rank = (i + 1) as i32;
    }

    // Apply limit if specified
    if let Some(limit) = query.limit {
        leaderboard.truncate(limit as usize);
    }

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": leaderboard
    })))
}

pub async fn get_user_ranking(user_id: web::Path<i32>) -> Result<HttpResponse> {
    let mut connection = establish_connection();
    let user_id = user_id.into_inner();

    // Get user's workout progress calories
    let _workout_calories = workout_progress::table
        .filter(workout_progress::user_id.eq(user_id))
        .select(diesel::dsl::sql::<diesel::sql_types::BigInt>(
            "SUM(CASE WHEN burned_calories IS NOT NULL THEN CAST(burned_calories AS DECIMAL(10,2)) ELSE 0 END)",
        ))
        .first::<i64>(&mut connection)
        .unwrap_or(0);

    // Get user's programme progress calories
    let _programme_calories = programme_progress::table
        .filter(programme_progress::user_id.eq(user_id))
        .select(diesel::dsl::sql::<diesel::sql_types::BigInt>(
            "SUM(CASE WHEN burned_calories IS NOT NULL THEN CAST(burned_calories AS DECIMAL(10,2)) ELSE 0 END)",
        ))
        .first::<i64>(&mut connection)
        .unwrap_or(0);

    // Get total calories for all users
    let mut _all_user_calories: Vec<(i32, String, i64)> = Vec::new();

    // Get workout progress calories for all users
    let workout_all_calories = workout_progress::table
        .inner_join(users::table)
        .group_by(users::id)
        .select((
            users::id,
            users::name,
            diesel::dsl::sql::<diesel::sql_types::BigInt>(
                "SUM(CASE WHEN burned_calories IS NOT NULL THEN CAST(burned_calories AS DECIMAL(10,2)) ELSE 0 END)",
            ),
        ))
        .load::<(i32, String, i64)>(&mut connection)
        .unwrap_or_default();

    // Get programme progress calories for all users
    let programme_all_calories = programme_progress::table
        .inner_join(users::table)
        .group_by(users::id)
        .select((
            users::id,
            users::name,
            diesel::dsl::sql::<diesel::sql_types::BigInt>(
                "SUM(CASE WHEN burned_calories IS NOT NULL THEN CAST(burned_calories AS DECIMAL(10,2)) ELSE 0 END)",
            ),
        ))
        .load::<(i32, String, i64)>(&mut connection)
        .unwrap_or_default();

    // Combine calories from both sources
    let mut combined_stats: std::collections::HashMap<i32, (String, i64)> =
        std::collections::HashMap::new();

    for (user_id, username, calories) in workout_all_calories {
        combined_stats.insert(user_id, (username, calories));
    }

    for (user_id, username, calories) in programme_all_calories {
        combined_stats
            .entry(user_id)
            .and_modify(|(_, total_calories)| *total_calories += calories)
            .or_insert((username, calories));
    }

    // Convert to vector and sort by calories
    let mut all_users: Vec<(i32, String, i64)> = combined_stats
        .into_iter()
        .map(|(id, (name, calories))| (id, name, calories))
        .collect();
    all_users.sort_by(|a, b| b.2.cmp(&a.2));

    // Find user's rank
    let user_rank = all_users
        .iter()
        .position(|&(id, _, _)| id == user_id)
        .map(|pos| pos + 1)
        .unwrap_or(0) as i32;

    // Get user's name
    let username = users::table
        .find(user_id)
        .select(users::name)
        .first::<String>(&mut connection)
        .unwrap_or_else(|_| String::from("Unknown"));

    // Get total workouts count
    let total_workouts = workout_progress::table
        .filter(workout_progress::user_id.eq(user_id))
        .count()
        .get_result::<i64>(&mut connection)
        .unwrap_or(0) as i32;

    // Calculate total time (in seconds)
    let total_time = workout_progress::table
        .filter(workout_progress::user_id.eq(user_id))
        .select(diesel::dsl::sql::<diesel::sql_types::BigInt>(
            "SUM(COALESCE(actual_duration_seconds, 0))",
        ))
        .first::<i64>(&mut connection)
        .unwrap_or(0) as i32;

    // Count total exercises
    let total_exercises = workout_progress::table
        .filter(workout_progress::user_id.eq(user_id))
        .select(diesel::dsl::sql::<diesel::sql_types::BigInt>(
            "COUNT(DISTINCT exercise_id)",
        ))
        .first::<i64>(&mut connection)
        .unwrap_or(0) as i32;

    let ranking = UserRanking {
        user_id,
        username,
        rank: user_rank,
        total_workouts,
        total_time,
        total_exercises,
    };

    Ok(HttpResponse::Ok().json(json!({
        "status": "success",
        "data": ranking
    })))
}
