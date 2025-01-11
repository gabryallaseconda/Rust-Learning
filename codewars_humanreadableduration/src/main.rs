// https://www.codewars.com/kata/52685f7382004e774f0001f7




fn format_duration(seconds: u64) -> String {

    //println!("seconds: {}", seconds);

    let dis_seconds = seconds % 60;
    //println!("dis_seconds: {}", dis_seconds);

    let minutes = seconds / 60;
    //println!("minutes: {}", minutes);

    let dis_minutes = minutes % 60;
    //println!("dis_minutes: {}", dis_minutes);

    let hours = minutes / 60;
    //println!("hours: {}", hours);

    let dis_hours = hours % 24;
    //println!("dis_hours: {}", dis_hours);
    
    let days = hours / 24;
    //println!("days: {}", days);

    let dis_days = days % 365;
    //println!("dis_days: {}", dis_days);
    
    let years = days / 365;
    //println!("years: {}", years);

    let mut sentence: Vec<String> = Vec::new();

    if years > 0 {
        if years == 1 {
            sentence.push(format!("{} year", years));
        } else {
            sentence.push(format!("{} years", years));
        }
    }

    if dis_days > 0 {
        if dis_days == 1 {
            sentence.push(format!("{} day", dis_days));
        } else {
            sentence.push(format!("{} days", dis_days));
        }
    }

    if dis_hours > 0 {
        if dis_hours == 1 {
            sentence.push(format!("{} hour", dis_hours));
        } else {
            sentence.push(format!("{} hours", dis_hours));
        }
    }

    if dis_minutes > 0 {
        if dis_minutes == 1 {
            sentence.push(format!("{} minute", dis_minutes));    
        } else {
            sentence.push(format!("{} minutes", dis_minutes));
        }
    }

    if dis_seconds > 0 {
        if dis_seconds == 1 {
            sentence.push(format!("{} second", dis_seconds));
        } else {
            sentence.push(format!("{} seconds", dis_seconds));
        }
    }
    
    //println!("{:?}", sentence);

    //println!("{:#?}", sentence);

    let size = sentence.len();
    
    if size == 0 {
        return "now".to_string();
    } else if size == 1 {
        return sentence[0].clone();
    } else if size == 2 {
        return format!("{} and {}", sentence[0], sentence[1]);
    } else {

        let mut final_sentence: String = String::new();

        for i in 0..size{
            final_sentence.push_str(&sentence[i]);
            
            if i == size-2 {
                final_sentence.push_str(" and ");
            } else if i != size-1 {
                final_sentence.push_str(", ");
            }

        }

        return final_sentence;

    }     
}


fn format_duration_pro(mut amt: u64) -> String {
    let mut result: Vec<String> = [
        ("second", 60),
        ("minute", 60),
        ("hour", 24),
        ("day", 365),
        ("year", u64::MAX),
    ]
    .iter()
    .map(|(unit, modulo)| {
        let rem = amt % modulo;
        amt /= modulo;
        (unit, rem)
    })
    .filter_map(|(unit, duration)| match duration {
        0 => None,
        1 => Some(format!("{} {}", duration, unit)),
        _ => Some(format!("{} {}s", duration, unit)),
    })
    .collect();
    result.reverse();
    match result.len() {
        0 => "now".into(),
        1 => result.join(""),
        _ => result
            .split_last()
            .map(|(r, l)| l.join(", ") + " and " + r)
            .unwrap(),
    }
}

fn main() {
    println!("{}", format_duration(0));
    println!("{}", format_duration(152041896534));

}
