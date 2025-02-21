// @generated automatically by Diesel CLI.

diesel::table! {
    body_parts (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    equipment (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    exercise_body_parts (id) {
        id -> Int4,
        exercise_id -> Int4,
        body_part_id -> Int4,
    }
}

diesel::table! {
    exercise_categories (id) {
        id -> Int4,
        exercise_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    exercise_equipment (id) {
        id -> Int4,
        exercise_id -> Int4,
        equipment_id -> Int4,
    }
}

diesel::table! {
    exercises (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        is_active -> Bool,
        thumbnail_url -> Nullable<Text>,
        video_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    favorite_meals (id) {
        id -> Int4,
        user_id -> Int4,
        meal_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    favorite_workouts (id) {
        id -> Int4,
        user_id -> Int4,
        workout_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    meal_ingredients (id) {
        id -> Int4,
        meal_id -> Int4,
        name -> Varchar,
        amount -> Varchar,
    }
}

diesel::table! {
    meal_instructions (id) {
        id -> Int4,
        meal_id -> Int4,
        step_number -> Int4,
        instruction -> Text,
    }
}

diesel::table! {
    meal_plan_meals (id) {
        id -> Int4,
        meal_plan_id -> Int4,
        meal_id -> Int4,
        day_of_week -> Int4,
        meal_time -> Varchar,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    meal_plans (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        start_date -> Date,
        end_date -> Date,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    meals (id) {
        id -> Int4,
        name -> Varchar,
        category -> Varchar,
        description -> Nullable<Text>,
        image_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    programme_days_exercises (id) {
        id -> Int4,
        programme_week_id -> Int4,
        exercise_id -> Int4,
        day_number -> Int4,
        position -> Int4,
        sets_number -> Int4,
        reps -> Nullable<Int4>,
        duration_seconds -> Nullable<Int4>,
        rest_seconds -> Int4,
    }
}

diesel::table! {
    programme_progress (id) {
        id -> Int4,
        user_id -> Int4,
        programme_id -> Int4,
        programme_week_id -> Int4,
        exercise_id -> Int4,
        day_number -> Int4,
        completed -> Bool,
        actual_reps -> Nullable<Int4>,
        actual_sets_number -> Nullable<Int4>,
        actual_duration_seconds -> Nullable<Int4>,
        actual_rest_seconds -> Nullable<Int4>,
        notes -> Nullable<Text>,
        completed_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    programme_weeks (id) {
        id -> Int4,
        programme_id -> Int4,
        name -> Varchar,
        week_number -> Int4,
    }
}

diesel::table! {
    programmes (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        image_url -> Nullable<Text>,
        total_weeks -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        profile_picture -> Nullable<Varchar>,
        role -> Varchar,
        status -> Varchar,
    }
}

diesel::table! {
    workout_exercises (id) {
        id -> Int4,
        workout_id -> Int4,
        exercise_id -> Int4,
        position -> Int4,
        sets_number -> Int4,
        reps -> Nullable<Int4>,
        duration_seconds -> Nullable<Int4>,
        rest_seconds -> Nullable<Int4>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    workout_progress (id) {
        id -> Int4,
        user_id -> Int4,
        workout_id -> Int4,
        exercise_id -> Int4,
        workout_exercise_id -> Int4,
        completed -> Bool,
        actual_reps -> Nullable<Int4>,
        actual_sets_number -> Nullable<Int4>,
        actual_duration_seconds -> Nullable<Int4>,
        notes -> Nullable<Text>,
        completed_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    workouts (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        difficulty -> Varchar,
        thumbnail_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(exercise_body_parts -> body_parts (body_part_id));
diesel::joinable!(exercise_body_parts -> exercises (exercise_id));
diesel::joinable!(exercise_categories -> categories (category_id));
diesel::joinable!(exercise_categories -> exercises (exercise_id));
diesel::joinable!(exercise_equipment -> equipment (equipment_id));
diesel::joinable!(exercise_equipment -> exercises (exercise_id));
diesel::joinable!(favorite_meals -> meals (meal_id));
diesel::joinable!(favorite_meals -> users (user_id));
diesel::joinable!(favorite_workouts -> users (user_id));
diesel::joinable!(favorite_workouts -> workouts (workout_id));
diesel::joinable!(meal_ingredients -> meals (meal_id));
diesel::joinable!(meal_instructions -> meals (meal_id));
diesel::joinable!(meal_plan_meals -> meal_plans (meal_plan_id));
diesel::joinable!(meal_plan_meals -> meals (meal_id));
diesel::joinable!(meal_plans -> users (user_id));
diesel::joinable!(programme_days_exercises -> exercises (exercise_id));
diesel::joinable!(programme_days_exercises -> programme_weeks (programme_week_id));
diesel::joinable!(programme_progress -> exercises (exercise_id));
diesel::joinable!(programme_progress -> programme_weeks (programme_week_id));
diesel::joinable!(programme_progress -> programmes (programme_id));
diesel::joinable!(programme_progress -> users (user_id));
diesel::joinable!(programme_weeks -> programmes (programme_id));
diesel::joinable!(workout_exercises -> exercises (exercise_id));
diesel::joinable!(workout_exercises -> workouts (workout_id));
diesel::joinable!(workout_progress -> exercises (exercise_id));
diesel::joinable!(workout_progress -> users (user_id));
diesel::joinable!(workout_progress -> workout_exercises (workout_exercise_id));
diesel::joinable!(workout_progress -> workouts (workout_id));

diesel::allow_tables_to_appear_in_same_query!(
    body_parts,
    categories,
    equipment,
    exercise_body_parts,
    exercise_categories,
    exercise_equipment,
    exercises,
    favorite_meals,
    favorite_workouts,
    meal_ingredients,
    meal_instructions,
    meal_plan_meals,
    meal_plans,
    meals,
    programme_days_exercises,
    programme_progress,
    programme_weeks,
    programmes,
    users,
    workout_exercises,
    workout_progress,
    workouts,
);
