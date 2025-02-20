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
    }
}

diesel::table! {
    favorite_workouts (id) {
        id -> Int4,
        user_id -> Int4,
        workout_id -> Int4,
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
        reps -> Nullable<Int4>,
        duration_seconds -> Nullable<Int4>,
        rest_seconds -> Int4,
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
        reps -> Nullable<Int4>,
        duration_seconds -> Nullable<Int4>,
        rest_seconds -> Nullable<Int4>,
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
diesel::joinable!(programme_days_exercises -> exercises (exercise_id));
diesel::joinable!(programme_days_exercises -> programme_weeks (programme_week_id));
diesel::joinable!(programme_weeks -> programmes (programme_id));
diesel::joinable!(workout_exercises -> exercises (exercise_id));
diesel::joinable!(workout_exercises -> workouts (workout_id));

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
    meals,
    programme_days_exercises,
    programme_weeks,
    programmes,
    users,
    workout_exercises,
    workouts,
);
