use std::{collections::HashMap, fmt::format};

use crate::utils;

fn get_pair(s: String) -> Vec<i32> {
    return s.split(",").map(|x| x.parse().unwrap()).collect();
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = utils::get_lines("day05");
    let mut map: HashMap<String, i32> = HashMap::new();
    for line in lines {
        let coordinates: Vec<String> = line.split(" -> ").map(|s| String::from(s)).collect();
        let pair1: Vec<i32> = get_pair(coordinates.get(0).unwrap().clone());
        let pair2: Vec<i32> = get_pair(coordinates.get(1).unwrap().clone());

        let x1: i32 = *pair1.get(0).unwrap();
        let y1: i32 = *pair1.get(1).unwrap();
        let x2: i32 = *pair2.get(0).unwrap();
        let y2: i32 = *pair2.get(1).unwrap();

        if x1 == x2 {
            if y1 < y2 {
                for y in y1..(y2 + 1) {
                    let key: String = String::from(format!("{}#{}", x1, y));
                    if map.contains_key(&key) {
                        *map.get_mut(&key).unwrap() += 1;
                    } else {
                        map.insert(key, 1);
                    }
                }
            } else {
                for y in y2..(y1 + 1) {
                    let key: String = String::from(format!("{}#{}", x1, y));
                    if map.contains_key(&key) {
                        *map.get_mut(&key).unwrap() += 1;
                    } else {
                        map.insert(key, 1);
                    }
                }
            }
        } else if y1 == y2 {
            if x1 < x2 {
                for x in x1..(x2 + 1) {
                    let key: String = String::from(format!("{}#{}", x, y1));
                    if map.contains_key(&key) {
                        *map.get_mut(&key).unwrap() += 1;
                    } else {
                        map.insert(key, 1);
                    }
                }
            } else {
                for x in x2..(x1 + 1) {
                    let key: String = String::from(format!("{}#{}", x, y1));
                    if map.contains_key(&key) {
                        *map.get_mut(&key).unwrap() += 1;
                    } else {
                        map.insert(key, 1);
                    }
                }
            }
        } else {
            // println!("Diagonal ignoring {:?} {:?}", pair1, pair2);
        }
    }
    let vals: Vec<i32> = map.values().filter(|&&x| x > 1).cloned().collect();
    println!("{:?}", vals.len());
    Ok(())
}
