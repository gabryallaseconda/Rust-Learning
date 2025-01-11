



fn multiples_of_3_or_5_prod(number: i32) -> i128 {
    let mut product: i128 = 1;

    if number == 0 {
        return 0;
    }

    for i in 1..number {
        if i % 3 == 0 || i % 5 == 0 {
            product *= i as i128;
        }
    }

    product


}


fn multiples_of_3_or_5(number: i32) -> i32 {
    let mut sum: i32 = 0;


    for i in 0..number {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i as i32;
        }
    }

    sum


}






fn main() {
    println!("{}", multiples_of_3_or_5(50));
}
