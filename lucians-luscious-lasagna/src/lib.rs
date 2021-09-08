#![allow(unused)]

const COOKING_TIME: i32 = 40;
const LAYER_PREP: i32 = 2;

pub fn expected_minutes_in_oven() -> i32 {
    COOKING_TIME
}

pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    COOKING_TIME - actual_minutes_in_oven
}

pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    LAYER_PREP * number_of_layers
}

pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    let preparation: i32 = LAYER_PREP * number_of_layers;
    preparation + actual_minutes_in_oven
}
