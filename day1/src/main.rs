use prelude::*;
use std::{collections::{HashMap, HashSet}, path::Path};

mod prelude;

fn main() {
    part1();
    part2();
}

fn part1() {
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

fn part2() {
    let filename = "input.txt";
    let path = Path::new(filename);
    let (nums_1, nums_2) = get_pair_int_from_path(path).unwrap();

    let mut map: HashMap<i32, i32>  = HashMap::new();
    for num in nums_2 {
        *map.entry(num).or_insert(0) += 1;
    }

    let set: HashSet<i32> = nums_1.into_iter().collect();

    let mut ans = 0;
    for num in set {
        if map.contains_key(&num) {
            ans += num * map[&num];
        }
    }

    println!("ans: {ans}");
}
