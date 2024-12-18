use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

// pub fn get_int_from_path(path: &Path) -> std::io::Result<Vec<i32>> {
//     let file = File::open(&path)?;
//     let reader = io::BufReader::new(file);
//     let mut ans = vec!();
//     for line in reader.lines() {
//         let line = line?;
//         match line.trim().parse::<i32>() {
//             Ok(num) => ans.push(num),
//             Err(e) => println!("failed to parse number, got error {e}"),
//         }
//     }
//     Ok(ans)
// }
pub fn get_pair_int_from_path(path: &Path) -> std::io::Result<(Vec<i32>, Vec<i32>)> {
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    let mut ans1 = vec!();
    let mut ans2 = vec!();

    for line in reader.lines() {
        let line = line?;
        let parts:Vec<&str> = line.trim().split_whitespace().collect();
        assert!(parts.len() == 2);
        match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            (Ok(num1), Ok(num2)) => {
                ans1.push(num1); 
                ans2.push(num2);
            },
            _ => println!("errored with parsing"),
            
        }
    }
    Ok((ans1, ans2))
}
