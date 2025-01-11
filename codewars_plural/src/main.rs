



fn plural (n: f64) -> bool {
    if n == 1.0 {
        return false;
    } else {
        return true;
    }
}



fn main() {
    println!("{}", plural(1.0));
    println!("{}", plural(0.0));
    println!("{}", plural(100.0));
    println!("{}", plural(0.5));
}
