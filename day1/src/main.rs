use std::env;
use std::fs;
use std::io::{Error};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let contents = fs::read_to_string(path)?;

    let columns = separate_columns(&contents);
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();
    for (col1, col2) in columns {
        v1.push(col1);
        v2.push(col2);
    }
    v1.sort();
    v2.sort();

    let mut sum = 0;
    for (val1, val2) in v1.iter().zip(v2.iter()) {
        sum += (val1-val2).abs();
    }
    println! ("result:{}", sum);
    Ok(())
}

fn separate_columns(contents: &str) -> Vec<(i32, i32)> {
    contents.lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                Some((parts[0].parse().unwrap(), parts[1].parse().unwrap()))
            } else {
                None
            }
        })
        .collect()
}
