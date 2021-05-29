use chrono::{DateTime, Local, Timelike};
use google_speech::{Lang,Speech};
use ntext::to_text;

fn main() {
    let now: DateTime<Local>;

    now = Local::now();
    let (_is_pm, hour) = now.hour12();
    let mut hour = hour as usize;
    let minute = now.minute() as usize;
    if hour == 0 {
        hour = 12
    }

    let time: String;
    match minute {
        0 => time = format!("It is {} o'clock", to_text!(hour)),
        15 => time = format!("It is quarter past {} o'clock", to_text!(hour)),
        30 => time = format!("It is half past {} o'clock", to_text!(hour)),
        45 => time = format!("It is quarter to {} o'clock", to_text!(hour + 1)),
        1..=14 | 16..=29 => {
            time = format!("It is {} past {} o'clock", to_text!(minute), to_text!(hour))
        }
        31..=44 | 46..=59 => {
            time = format!(
                "It is {} to {} o'clock",
                to_text!(60 - minute),
                to_text!(hour + 1)
            )
        }
        _ => time = "Unable to obtain time".to_string(),
    }
    println!("{}", time);
    let speech = Speech::new(time,Lang::en_us).unwrap();
    speech.play().unwrap();
}
