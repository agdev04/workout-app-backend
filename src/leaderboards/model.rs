use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct LeaderboardEntry {
    pub user_id: i32,
    pub username: String,
    pub total_workouts: i32,
    pub total_time: i32, // in seconds
    pub total_exercises: i32,
    pub rank: i32,
    pub total_calories: f64,
}

#[derive(Debug, Serialize)]
pub struct UserRanking {
    pub user_id: i32,
    pub username: String,
    pub rank: i32,
    pub total_workouts: i32,
    pub total_time: i32,
    pub total_exercises: i32,
}

#[derive(Debug, Deserialize)]
pub struct LeaderboardQuery {
    pub metric: Option<String>, // total_workouts, total_time, total_exercises
    pub period: Option<String>, // weekly, monthly, all-time
    pub limit: Option<i32>,
}
