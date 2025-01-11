
fn even_or_odd(number: i32) -> &'static str {
    if number % 2 == 0 {
        return "Even";
    } else {    
        return "Odd";
    }
}



fn main() {

    println!("{}", (55.0_f32.powf(2.0) + 100.0_f32.powf(2.0)).sqrt());

    println!("{}", even_or_odd(1));
    println!("{}", even_or_odd(0));
    println!("{}", even_or_odd(100));
    println!("{}", even_or_odd(3));
}
