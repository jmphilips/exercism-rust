pub fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        true
    } else if year % 4 != 0 || year % 100 == 0 {
        false
    } else {
        true
    }
}
