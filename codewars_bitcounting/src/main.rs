// https://www.codewars.com/kata/526571aae218b8ee490006f4
// then, go for kyu 5

// useful link: https://doc.rust-lang.org/std/fmt/

fn count_bits(n: i64) -> u32 {
    // code here

    let binary: String = format!("{:b}", n);

    println!("{}", binary);

    let mut counter: u32 = 0;

    for c in binary.chars(){
        if c == '1'{
            counter += 1;
        }

    }


    return counter;
  }


fn main() {
    println!("{}", count_bits(64));
    println!("{}", count_bits(2458624));
    println!("{}", count_bits(5635));
    }
