// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

const DEFAULT_PRODUCED_EACH_HOUR: f64 = 221.00;

pub fn get_percentage_of_value(value: f64, porcentage: f64) -> f64 {
    (value * porcentage / 100.0)
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed == 0 {
        return 0.00;
    }
    let mut success_rate = 100.00;

    if speed >= 1 && speed <= 4 {
        success_rate = 100.00;
    } else if speed >= 5 && speed <= 8 {
        success_rate = 90.00;
    } else {
        success_rate = 77.00;
    }
    let total_cars = speed as f64 * DEFAULT_PRODUCED_EACH_HOUR;

    if success_rate == 100.00 {
        total_cars
    } else {
        get_percentage_of_value(total_cars, success_rate)
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let production_rate = production_rate_per_hour(speed) as u32;
    println!("result of rate_per_hour : {}", production_rate);
    let rate = production_rate / 60;
    rate
}
