// https://www.codewars.com/kata/52685f7382004e774f0001f7

fn make_readable(seconds: u32) -> String {
    let dis_seconds = seconds % (60 as u32);
    let minutes = ((seconds - dis_seconds) / 60u32 ) as u32;
    let dis_minutes = minutes % 60u32;
    let hours = ((minutes-dis_minutes) / 60u32) as u32;

    let mut display: String = String::new();

    if hours < 10 {
        display.push_str("0");
    }
    display.push_str(&hours.to_string());
    display.push(':');
    
    if dis_minutes < 10 {
        display.push_str("0");
    }
    display.push_str(&dis_minutes.to_string());
    display.push(':');
    
    if dis_seconds < 10 {
        display.push_str("0");
    }
    display.push_str(&dis_seconds.to_string());

    return display;
}


fn make_readable_pro(seconds: u32) -> String {
    let dis_seconds = seconds % 60;
    let minutes = seconds / 60;
    let dis_minutes = minutes % 60;
    let hours = minutes / 60;

    format!("{:02}:{:02}:{:02}", hours, dis_minutes, dis_seconds)
}

fn main() {
    println!("{}", make_readable(0));
    println!("{}", make_readable_pro(0));

}
