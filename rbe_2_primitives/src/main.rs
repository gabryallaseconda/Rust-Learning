#![allow(dead_code)]

fn main() {


    let logical: bool = true; 

    let a_float: f64 = 1.0;

    let an_integer: i32 = 2;

    println!("exp {}", a_float as i32 *an_integer);

    let mut inferred_type = 100;
    println!("inferred_type = {}", inferred_type);
    inferred_type = 200;
    println!("inferred_type = {}", inferred_type);

    let my_array: [i8; 5]  = [1,2,3,4,5];

    let my_tuple = (5u32, (1u8, -4i8), true, -5.04f32);

    println!("Value one: {}", my_tuple.0);
    println!("Value two: {:#?}", my_tuple.1);




}
