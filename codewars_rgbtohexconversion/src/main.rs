// https://www.codewars.com/kata/513e08acc600c94f01000001
//https://www.educative.io/answers/rgb-to-hex
//https://doc.rust-lang.org/rust-by-example/primitives/array.html
//https://doc.rust-lang.org/rust-by-example/types/cast.html

// finish this

use std::collections::HashMap;


fn single_unit_conversion(x: i32) -> String {
    let digit_to_hex: HashMap<i32, String> = HashMap::from([
        (0, "0".to_string()),
        (1, "1".to_string()),
        (2, "2".to_string()),
        (3, "3".to_string()),
        (4, "4".to_string()),
        (5, "5".to_string()),
        (6, "6".to_string()),
        (7, "7".to_string()),
        (8, "8".to_string()),
        (9, "9".to_string()),
        (10, "A".to_string()),
        (11, "B".to_string()),
        (12, "C".to_string()),
        (13, "D".to_string()),
        (14, "E".to_string()),
        (15, "F".to_string()),
    ]);

    let x_conversion: f32 = x as f32;
    let first_char = x_conversion / 16f32;
    let first_char_int: i32 = first_char as i32;
    let remainder: f32 = x_conversion - (first_char_int*16) as f32;
    let second_char_int: i32 = remainder as i32;

    let mut hexa: String = String::new();
    hexa.push_str(&digit_to_hex[&first_char_int]);
    hexa.push_str(&digit_to_hex[&second_char_int]);

    return hexa;


}

fn rgb(r: i32, g: i32, b: i32) -> String {
    let mut hexadecimal:String = String::new();

    let rgb: [i32; 3] = [r, g, b];

    for x in rgb {
        let x_hex = single_unit_conversion(x);
        hexadecimal.push_str(&x_hex);

    }

    return hexadecimal;


  }
  

fn rgb_pro_version(r: i32, g: i32, b: i32) -> String {
    format!("{:02X}{:02X}{:02X}", 
        r.clamp(0, 255), 
        g.clamp(0, 255), 
        b.clamp(0, 255)
    )
}


fn main() {
    println!("{}", rgb(0,0,0));
    println!("{}", rgb(220,20,60));

    println!("{}", rgb_pro_version(0,0,0));
    println!("{}", rgb_pro_version(220,20,60));
}
