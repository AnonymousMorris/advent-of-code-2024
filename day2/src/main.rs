use std::{path::Path};
use prelude::*;

mod prelude;
fn main() {

    let filename = "input.txt";
    let path = Path::new(filename);
    let num_lists = get_num_lists(path).unwrap();

    part1(&num_lists);
    part2(&num_lists);
}

fn part1(num_lists: &Vec<Vec<i32>>) {
    assert!(num_lists.len() > 0);
    let mut safes: Vec<bool> = Vec::with_capacity(num_lists.len());
    for idx in 0..num_lists.len() {
        safes.push(check_safe(&num_lists[idx]));
    }

    let mut ans = 0;
    for safe in &safes {
        if *safe {
            ans += 1;
        }
    }
    
    println!("ans: {ans}");
}

fn part2 (num_lists: &Vec<Vec<i32>>) {
    let mut valids: Vec<bool> = Vec::with_capacity(num_lists.len());
    for idx in 0..num_lists.len() {
        valids.push(check_permutations(&num_lists[idx]));
    }

    let mut ans = 0;
    for valid in valids {
        if valid {
            ans += 1;
        }
    }
    println!("ans: {ans}");
}

fn check_permutations(nums: &Vec<i32>) -> bool {
    let mut ans : bool = false;
    let len = nums.len();
    for i in 0..len {
        ans |= check_safe(&[&nums[0..i], &nums[i+1..len]].concat());
    }
    return ans;
}

fn check_safe(nums: &Vec<i32>) -> bool {
    assert!(nums.len() > 0);
    if nums.len() <= 1 {
        return true;
    }
    else {
        let increasing: bool = nums[1] > nums[0];
        for idx in 1..nums.len() {
            let diff = nums[idx] - nums[idx-1];
            if diff.abs() > 3 || diff == 0 {
                return false;
            }
            let same_sign = (diff > 0) == increasing;
            if !same_sign {
                return false;
            }
        }
        return true;
    }
}
