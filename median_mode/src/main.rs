///Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
/// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;
fn main() {
    let mut intes = vec![5,2,6,4,3,7,7,7,7,7,7,7,7,7,7,7,7,7,8,9,10,15,12,11,14,13,26,88,92,35,72,22,45,65,90];
    let lent:usize = ((intes.len())/2).try_into().unwrap();
    println!("Median is at position : {lent}");
    intes.sort();
    println!("Median is {}",intes[lent-1]);
    
    let mut map = HashMap::new();

    for i in intes{
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    println!("Mode list of the elements:");
    println!("{:?}",map);
}


