/// Check if String literal is matching SQL time format: HH:MM:SS or HH:MM:SS.SSS
pub fn is_valid_time_format(time_str: &str) -> bool {
    // Check length of the string
    if !(8..=12).contains(&time_str.len()) {
        return false;
    }

    // Split the string into hours, minutes, seconds, and optional milliseconds
    let parts: Vec<&str> = time_str.split(':').collect();
    if parts.len() < 3 || parts.len() > 4 {
        return false;
    }

    // Extract hours, minutes, seconds, and optionally milliseconds
    let hours = parts[0].parse::<u32>().ok();
    let minutes = parts[1].parse::<u32>().ok();
    let seconds_parts: Vec<&str> = parts[2].split('.').collect();
    let seconds = seconds_parts[0].parse::<u32>().ok();
    let milliseconds = if seconds_parts.len() == 2 {
        seconds_parts[1].parse::<u32>().ok()
    } else {
        Some(0)
    };

    // Validate the parsed values
    hours.is_some()
        && minutes.is_some()
        && seconds.is_some()
        && milliseconds.is_some()
        && hours.unwrap() < 24
        && minutes.unwrap() < 60
        && seconds.unwrap() < 60
        && milliseconds.unwrap() < 1000
}

/// Check if String literal is matching SQL Date format: YYYY-MM-DD
pub fn is_valid_date_format(date_str: &str) -> bool {
    // Check length of the string
    if date_str.len() != 10 {
        return false;
    }

    // Split the string into year, month, and day
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    // Extract year, month, and day
    let year = parts[0].parse::<u32>().ok();
    let month = parts[1].parse::<u32>().ok();
    let day = parts[2].parse::<u32>().ok();

    // Validate the parsed values
    year.is_some()
        && month.is_some()
        && day.is_some()
        && year.unwrap() >= 1
        && month.unwrap() >= 1
        && month.unwrap() <= 12
        && day.unwrap() >= 1
        && day.unwrap() <= 31
}

/// Check if String literal is matching SQL Date format: YYYY-MM-DD HH:MM:SS or YYYY-MM-DD HH:MM:SS.SSS
pub fn is_valid_datetime_format(datetime_str: &str) -> bool {
    // Check length of the string
    if !(19..=23).contains(&datetime_str.len()) {
        return false;
    }

    // Split the string into date and time components
    let parts: Vec<&str> = datetime_str.split_whitespace().collect();
    if parts.len() != 2 {
        return false;
    }

    // Check the validity of date and time components
    is_valid_date_format(parts[0]) && is_valid_time_format(parts[1])
}
