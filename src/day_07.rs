use std::collections::HashMap;

use crate::utils;

pub fn calculate(numbers: Vec<i128>) -> i128 {
    let mut map: HashMap<i128, i128> = HashMap::new();
    for num in numbers.clone() {
        if map.contains_key(&num) {
            *map.get_mut(&num).unwrap() += 1;
        } else {
            map.insert(num, 1);
        }
    }
    let max = *numbers.iter().max().unwrap();

    let mut min_fuel: i128 = i128::MAX;
    for k in 1..(max + 1) {
        let mut fuel: i128 = 0;
        for num in numbers.clone() {
            let n = (num - k).abs();
            let fuel_rate = (n * (n + 1)) / 2;
            fuel += fuel_rate
        }
        println!("Fuel for pos {} is {}", k, fuel);
        if min_fuel > fuel {
            min_fuel = fuel;
        }
    }

    min_fuel
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = utils::get_lines("day07");
    let numbers: Vec<i128> = lines
        .get(0)
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();
    let result = calculate(numbers);
    println!("Result {}", result);
    Ok(())
}
