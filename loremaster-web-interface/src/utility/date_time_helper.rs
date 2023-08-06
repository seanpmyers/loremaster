use super::constants::{
    APRIL, AUGUST, DECEMBER, FEBRUARY, JANUARY, JULY, JUNE, MARCH, MAY, NOVEMBER, OCTOBER,
    SEPTEMBER,
};

pub fn get_day_of_week_from_integer(day_number: u32) -> String {
    match day_number {
        0 => String::from("Sunday"),
        1 => String::from("Monday"),
        2 => String::from("Tuesday"),
        3 => String::from("Wednesday"),
        4 => String::from("Thursday"),
        5 => String::from("Friday"),
        6 => String::from("Saturday"),
        _ => {
            log::error!("Invalid day of week integer!");
            String::from("")
        }
    }
}

pub fn get_month_from_integer(month_number: u32) -> String {
    match month_number {
        0 => String::from(JANUARY),
        1 => String::from(FEBRUARY),
        2 => String::from(MARCH),
        3 => String::from(APRIL),
        4 => String::from(MAY),
        5 => String::from(JUNE),
        6 => String::from(JULY),
        7 => String::from(AUGUST),
        8 => String::from(SEPTEMBER),
        9 => String::from(OCTOBER),
        10 => String::from(NOVEMBER),
        11 => String::from(DECEMBER),
        _ => {
            log::error!("Invalid month integer");
            String::from("")
        }
    }
}
