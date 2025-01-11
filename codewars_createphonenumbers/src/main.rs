//https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/rust
// https://doc.rust-lang.org/std/iter/trait.Iterator.html

fn create_phone_number(numbers: &[u8]) -> String {
    
    let mut cellnumber: String = String::new();

    for (i, number) in numbers.iter().enumerate(){
        if i == 0{
            cellnumber.push('(');
        } else if i == 3{
            cellnumber.push_str(") ");
        } else if i == 6 {
            cellnumber.push('-');
        }
        
        cellnumber.push_str(&number.to_string());

    }

    return cellnumber;

}


fn main() {
    println!("{}", create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));
}
