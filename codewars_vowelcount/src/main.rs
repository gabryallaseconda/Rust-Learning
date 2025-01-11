
fn get_count(string: &str) -> usize {
    let mut vowels_count: usize = 0;
  
    // Do your magic here

    for c in string.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => vowels_count += 1,
            _ => (),
        }
    } 

    
    vowels_count
  }
  


fn get_count_recursive(string: &str) -> usize {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut vowels_count: usize = 0;

    if string.len() == 0 {
        return 0;
    }

    let first_char = string.chars().next();

    println!("{:?}", first_char);

    match first_char {
        Some('a') | Some('e') | Some('i') | Some('o') | Some('u') => vowels_count += 1,
        _ => (),
    }

    vowels_count + get_count(&string[1..])
}




fn main() {
    println!("{}", get_count("abracadabra"));
}
