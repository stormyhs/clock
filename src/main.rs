use std::time::{UNIX_EPOCH, SystemTime};

#[derive(Clone)]
enum TimeType {
    UnixSeconds,
    UnixMilliseconds,
}

impl std::fmt::Display for TimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TimeType::UnixSeconds => write!(f, "Unix Sec"),
            TimeType::UnixMilliseconds => write!(f, "Unix Millisec"),
        }
    }
}

/// Returns a human-readable string representing the time difference between two dates.
///
/// Example return: "1 year, 2 months, 3 days, 4 hours, 5 minutes, 6 seconds ago"
fn get_relative_time(current: u128, against: u128, format: TimeType) -> String {
    let current = match format {
        TimeType::UnixSeconds => current,
        TimeType::UnixMilliseconds => current / 1000,
    };

    let against = match format {
        TimeType::UnixSeconds => against,
        TimeType::UnixMilliseconds => against / 1000,
    };

    let direction = if current > against { "past" } else { "future" };
    let mut diff = if current > against { current - against } else { against - current };

    let mut result = String::new();

    let years = diff / 31536926;
    diff %= 31536000;
    let months = diff / 2629743;
    diff %= 2592000;
    let days = diff / 86400;
    diff %= 86400;
    let hours = diff / 3600;
    diff %= 3600;
    let minutes = diff / 60;
    let seconds = diff % 60;

    if years > 0 {
        result.push_str(&format!("{} year{}", years, if years > 1 { "s" } else { "" }));
    }
    if months > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} month{}", months, if months > 1 { "s" } else { "" }));
    }
    if days > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} day{}", days, if days > 1 { "s" } else { "" }));
    }
    if hours > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} hour{}", hours, if hours > 1 { "s" } else { "" }));
    }
    if minutes > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} minute{}", minutes, if minutes > 1 { "s" } else { "" }));
    }
    if seconds > 0 {
        if !result.is_empty() {
            result.push_str(", ");
        }
        result.push_str(&format!("{} second{}", seconds, if seconds > 1 { "s" } else { "" }));
    }

    if direction == "future" {
        result.push_str(" from now");
    } else {
        result.push_str(" ago");
    }

    result
}

fn main() {
    const YEAR_3000_UNIX_SECONDS: u128 = 32503680000;

    let args = std::env::args().collect::<Vec<String>>();

    let mut input = String::new();
    let mut format = TimeType::UnixSeconds;

    let mut user_gave_format = false;

    for arg in args.iter().skip(1) {
        if arg == "-s" {
            format = TimeType::UnixSeconds;
            user_gave_format = true;
        }
        if arg == "-ms" {
            format = TimeType::UnixMilliseconds;
            user_gave_format = true;
        }
        input = arg.to_string();
    }

    let current_unix_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u128;
    let current_unix_millisec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let current_date = chrono::DateTime::from_timestamp(current_unix_sec as i64, 0).unwrap();
    let current_date_hr = format!("{}", current_date.format("%H:%M:%S %d-%m-%Y"));

    if input.is_empty() {
        println!();
        println!("Timestamp: {} ({})", current_unix_sec, TimeType::UnixSeconds);
        println!("Timestamp: {} ({})", current_unix_millisec, TimeType::UnixMilliseconds);
        println!("Date:      {}", current_date_hr);
        println!();

        return;
    }

    let input = input.parse::<u128>().expect("Invalid input");

    if !user_gave_format && input >= YEAR_3000_UNIX_SECONDS {
        format = TimeType::UnixMilliseconds;
    }

    let date = match format {
        TimeType::UnixSeconds => chrono::DateTime::from_timestamp(input as i64, 0).unwrap(),
        TimeType::UnixMilliseconds => chrono::DateTime::from_timestamp((input / 1000) as i64, 0).unwrap(),
    };
    let date_hr = format!("{}", date.format("%H:%M:%S %d-%m-%Y"));

    let current_unix_matches_format = match format {
        TimeType::UnixSeconds => current_unix_sec,
        TimeType::UnixMilliseconds => current_unix_millisec,
    };
    let distance_hr = get_relative_time(current_unix_matches_format, input, format.clone());

    let alt_input = match format {
        TimeType::UnixSeconds => input * 1000,
        TimeType::UnixMilliseconds => input / 1000,
    };
    let alt_format = match format {
        TimeType::UnixSeconds => TimeType::UnixMilliseconds,
        TimeType::UnixMilliseconds => TimeType::UnixSeconds,
    };

    println!();
    println!("Timestamp: {} ({})", input, format);
    println!("Timestamp: {} ({})", alt_input, alt_format);
    println!("Date:      {}", date_hr);
    println!("Distance:  {}", distance_hr);
    println!();
}
