use std::collections::HashMap;

use crate::utils;

pub fn calculate(numbers: Vec<i128>, days: i128) -> i128 {
    let mut day = 1;
    let mut map: HashMap<i128, i128> = HashMap::new();
    for num in numbers {
        if map.contains_key(&num) {
            *map.get_mut(&num).unwrap() += 1;
        } else {
            map.insert(num, 1);
        }
    }
    while day <= days {
        let mut copy_map: HashMap<i128, i128> = HashMap::new();
        for (n, count) in map {
            if n == 0 {
                if copy_map.contains_key(&6) {
                    *copy_map.get_mut(&6).unwrap() += count
                } else {
                    copy_map.insert(6, count);
                }
                if copy_map.contains_key(&8) {
                    *copy_map.get_mut(&8).unwrap() += count
                } else {
                    copy_map.insert(8, count);
                }
            } else {
                if copy_map.contains_key(&(n - 1)) {
                    *copy_map.get_mut(&(n - 1)).unwrap() += count
                } else {
                    copy_map.insert(n - 1, count);
                }
            }
        }

        map = copy_map;
        day += 1;
    }
    map.values().sum()
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = utils::get_lines("day06");
    let numbers: Vec<i128> = lines
        .get(0)
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let result = calculate(numbers, 256);
    println!("Result {}", result);
    Ok(())
}
