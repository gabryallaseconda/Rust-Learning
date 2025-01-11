
/* Your goal in this kata is to implement a difference function, which subtracts one list from another and returns the result.

It should remove all values from list a, which are present in list b keeping their order.

arrayDiff([1,2],[1]) == [2]
If a value is present in b, all of its occurrences must be removed from the other:

arrayDiff([1,2,2,2,3],[2]) == [1,3]

*/

/* 
fn arrayDiff<const M: usize, const N: usize>(
    main: [i32;N], 
    adjoint: [i32; M]
) -> [i32;N] {

    let mut position: usize = 0;
    let mut result: [i32;N] = [0;N];

    for n in main{

        let mut count = n;

        for m in adjoint{
            count += m;
        }

        result[position] = count;
        position += 1;
    }

    result   
}
*/

fn arrayDiff<const M: usize, const N: usize>(
    main: [i32;N], 
    adjoint: [i32; M]
) -> Vec<i32> {

    let mut result = Vec::new();
    let mut insert:bool = true;

    for n in main{

        insert = true;


        for m in adjoint{
            if n == m{
                insert = false;
                break;
            }
        }

        if insert{
            result.push(n);
        }
    }

    result   
}



fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut result = Vec::new();
    //let mut insert:bool = true;

    for n in a{

        let mut insert = true;

        for m in &b{
            if n == *m{
                insert = false;
                break;
            }
        }

        if insert{
            result.push(n);
        }
    }

    result  
}


fn array_diff_2<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    
    let mut result = Vec::new();
    //let mut insert:bool = true;

    for n in a{

        let mut insert = true;

        for m in &b{
            if n == *m{
                insert = false;
                break;
            }
        }

        if insert{
            result.push(n);
        }
    }

    result  
}






fn main() {
    println!("result {:?}", arrayDiff([1,2,2,2,3], [2]));
    println!("result {:?}", arrayDiff([1,2], [1]));
    println!("result {:?}", arrayDiff([1,2,4,3,7,5], [2, 5]));

    let a = vec![1, 2, 2, 3];
    let b = vec![2];
    
    let result = array_diff(a, b);
    println!("{:?}", result);  // Prints: [1, 3]
}
