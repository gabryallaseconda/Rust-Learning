use std::collections::HashMap; 


fn count_duplicates(text: &str) -> u32 {

    let mut dict: HashMap<char, u32> = HashMap::new();

    let mytext: String = text.to_lowercase();

    for c in mytext.chars(){
        if dict.contains_key(&c){
            dict.entry(c).and_modify(|value| *value += 1);
        } else {
            dict.insert(c, 1);
        }
    }

    let mut counter: u32 = 0;

    for (_key, value) in dict.iter() {
        if *value > 1 {
            counter += 1;
        }
    }
    counter
}


fn count_duplicates_exploitinghashapi(text: &str) -> u32 {

    let mut dict: HashMap<char, u32> = HashMap::new();

    let mytext: String = text.to_lowercase();

    for c in mytext.chars(){
        *dict.entry(c).or_insert(0) += 1;
    }

    let mut counter: u32 = 0;

    for (_key, value) in dict.iter() {
        if *value > 1 {
            counter += 1;
        }
    }
    counter
}



fn main() {
    println!("{}", count_duplicates_exploitinghashapi("helloo00"));
}
