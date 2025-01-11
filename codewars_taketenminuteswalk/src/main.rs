
fn is_valid_walk(walk: &[char]) -> bool {

    let mut vertical: i32 = 0;
    let mut horizontal: i32 = 0;
    let mut counter: i32 = 0;

    for direction in walk{

        counter += 1;

        if (*direction == 'n'){
            vertical +=1;
        } else if (*direction == 's'){
            vertical -= 1;
        } else if (*direction == 'w'){
            horizontal += 1;
        } else if (*direction == 'e'){
            horizontal -= 1;
        } else {
            println!("Error!")
        }
    }

    if (counter == 10) && (vertical == 0) && (horizontal == 0) {
        return true;
    } else {
        return false;
    }
}


fn main() {
    println!("{}", is_valid_walk(&['n','s','n','s','n','s','n','s','e', 'w']));
}
