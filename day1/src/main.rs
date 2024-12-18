use prelude::*;
use std::path::Path;


mod prelude;


fn main() {
    let filename = "input.txt";
    let path = Path::new(filename);
    let (mut nums_1, mut nums_2) = get_pair_int_from_path(path).unwrap();
    nums_1.sort();
    nums_2.sort();

    let mut ans = 0;
    for i in 0..nums_1.len() {
        let diff = nums_1[i] - nums_2[i];
        ans += diff.abs();
    }
    println!("ans: {ans}");
}
