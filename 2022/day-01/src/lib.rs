use std::fs;
use anyhow::Result;

pub fn load_file(filename: &str) -> Result<String> {
    let bytes = fs::read(filename)?;

    let s = String::from_utf8(bytes)?;

    Ok(s)
}

pub fn get_calorie_counts(s: &str) -> Vec<u32> {
    let mut sums = s.split("\n\n").map(|load| {
        load.lines().map(|load_str| load_str.parse::<u32>().unwrap()).sum()
    }).collect::<Vec<u32>>();

    sums.sort_by(|a, b| b.cmp(a));

    sums
}

