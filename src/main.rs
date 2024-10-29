use std::time::{UNIX_EPOCH, SystemTime};
use std::thread;

mod storage;

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

fn print_all_markers() {
    let doc = storage::read_data();

    println!();
    
    let mut count = 1;
    let events = doc["events"].as_array().unwrap();
    for event in events.iter() {
        let entry = event.as_inline_table().unwrap();
        let timestamp = entry["timestamp"].as_integer().unwrap();
        let description = entry["description"].as_str().unwrap();
        println!("{}: {} {}", count, timestamp, description);
        count += 1;
    }

    println!("\nTotal markers: {}\n", count - 1);
}

fn print_marker(index: u64) {
    let index = index - 1; // Convert to 0-based index
    let doc = storage::read_data();

    let events = doc["events"].as_array().unwrap();

    let event = events.get(index as usize).unwrap();
    let entry = event.as_inline_table().unwrap();

    let timestamp = entry["timestamp"].as_integer().unwrap() as u128;
    let description = entry["description"].as_str().unwrap();

    let current_unix_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u128;
    let distance_hr = get_relative_time(current_unix_sec, timestamp, TimeType::UnixSeconds);
    
    println!();
    println!("{}: {}\n", timestamp, description);
    println!("{}", distance_hr);
    println!();
}

fn add_marker(timestamp: u128, description: &str) {
    let mut doc = storage::read_data();

    let val = format!("{{timestamp = {}, description = \"{}\"}}", timestamp, description);
    let val: toml_edit::Value = val.parse().unwrap();

    let array = doc["events"].as_array_mut().unwrap();
    array.push_formatted(val);

    storage::write_data(doc);
}

fn clear_markers() {
    let mut doc = storage::read_data();

    let array = doc["events"].as_array_mut().unwrap();
    array.clear();

    storage::write_data(doc);
}

fn main() {
    const YEAR_3000_UNIX_SECONDS: u128 = 32503680000;

    let args = std::env::args().collect::<Vec<String>>();

    let mut input = String::new();
    let mut format = TimeType::UnixSeconds;
    let mut timer = false;
    let mut print_markers = false;

    let mut user_gave_format = false;

    let mut i = 1; // Skip the first argument, which is the program name
    while i < args.len() {
        let arg = &args[i];
        if arg == "-s" {
            format = TimeType::UnixSeconds;
            user_gave_format = true;
        }
        if arg == "-ms" {
            format = TimeType::UnixMilliseconds;
            user_gave_format = true;
        }
        if arg == "-t" {
            timer = true;
        }
        if arg == "markers" {
            print_markers = true;
        }
        // New marker
        if arg == "m" {
            // If there are no more arguments, print all markers
            if args.len() - 1 <= i {
                print_all_markers();
                return;
            }

            // If there is only one more argument, and it is a number, print that marker.
            if args.len() - 1 == i + 1 {
                let marker_index = args[i + 1].parse::<u64>().unwrap_or(0);
                if marker_index != 0 {
                    print_marker(marker_index);
                    return;
                }
            }

            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u128;

            // Description is every other argument
            let mut description = String::new();
            for arg in args.iter().skip(2) {
                description.push_str(&arg);
                description.push_str(" ");
            }

            description = description.trim().to_string();

            add_marker(timestamp, &description);
            println!("Added marker {}", timestamp);

            return;
        }
        if arg == "clear" {
            clear_markers();
            println!("Cleared all markers");
            return;
        }
        input = arg.to_string();
        println!("set input: {}", input);

        i += 1;
    }

    let current_unix_sec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as u128;
    let current_unix_millisec = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let current_date = chrono::DateTime::from_timestamp(current_unix_sec as i64, 0).unwrap();
    let current_date_hr = format!("{}", current_date.format("%H:%M:%S %d-%m-%Y"));

    // No input
    if input.is_empty() {
        println!();
        println!("Timestamp: {} ({})", current_unix_sec, TimeType::UnixSeconds);
        println!("Timestamp: {} ({})", current_unix_millisec, TimeType::UnixMilliseconds);
        println!("Date:      {}", current_date_hr);
        println!();

        return;
    }

    if print_markers {
        print_all_markers();
        return;
    }

    if timer {
        let mut input = input.parse::<u128>().expect("Invalid input");

        let mut timer = match format {
            TimeType::UnixSeconds => std::time::Duration::from_secs(input as u64),
            TimeType::UnixMilliseconds => std::time::Duration::from_millis(input as u64),
        };

        println!("Sleeping...");
        thread::sleep(timer);
        println!("Done!");

        return;
    }

    // If the user only gave a number
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
