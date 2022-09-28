// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

const CARS_PER_HOUR: i32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed_converted: i32 = speed as i32;
    let perfect_cars_per_hours = (CARS_PER_HOUR * speed_converted) as f64;
    if speed < 5 {
        return perfect_cars_per_hours;
    } else if speed < 9 {
        return perfect_cars_per_hours * 0.9;
    } else {
        return perfect_cars_per_hours * 0.77;
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    return (production_rate_per_hour(speed) / 60_f64) as u32
}
