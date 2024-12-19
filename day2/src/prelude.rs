use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;


fn get_line(line: &str) -> Vec<i32> {
    let parts:Vec<&str> = line.trim().split_whitespace().collect();
    let mut ans:Vec<i32> = vec![];
    for part in parts {
        ans.push(part.parse::<i32>().unwrap());
    }
    ans
}

pub fn get_num_lists(path: &Path) -> std::io::Result<Vec<Vec<i32>>> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut ans: Vec<Vec<i32>> = vec![];

    for line in reader.lines() {
        let line = line?;
        let line_nums = get_line(&line);
        ans.push(line_nums);
    }
    Ok(ans)
}

