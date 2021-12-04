use std::collections::{BTreeSet, HashMap};
use std::mem::replace;

use crate::utils;
#[derive(Debug)]
struct BingoBoard {
    board: Vec<i32>,
    position_map: HashMap<i32, i32>,
}

impl BingoBoard {
    fn new(board: Vec<i32>) -> Self {
        let mut position_map: HashMap<i32, i32> = HashMap::new();
        for (index, val) in board.clone().into_iter().enumerate() {
            position_map.insert(val, index as i32);
        }
        Self {
            board,
            position_map,
        }
    }

    fn mark_complete(&mut self, number: i32) {
        let index_opt = self.position_map.get(&number);
        if let Some(&index) = index_opt {
            let _got = replace(&mut self.board[index as usize], -1);
        };
    }

    fn get_sum(&mut self) -> i32 {
        return self.board.iter().filter(|x| *x > &0).sum();
    }

    fn is_complete(&self) -> bool {
        for row in self.board.chunks(5) {
            let sum: i32 = row.into_iter().sum();
            if sum == -5 {
                return true;
            }
        }
        for i in 0..5 {
            let i1 = *self.board.get(i as usize).unwrap();
            let i2 = *self.board.get(i + 5 as usize).unwrap();
            let i3 = *self.board.get(i + 10 as usize).unwrap();
            let i4 = *self.board.get(i + 15 as usize).unwrap();
            let i5 = *self.board.get(i + 20 as usize).unwrap();
            let sum = i1 + i2 + i3 + i4 + i5;
            if sum == -5 {
                return true;
            }
        }
        return false;
    }
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let lines: Vec<String> = utils::get_lines("day04")
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();
    let mut numbers: Vec<i32> = vec![];
    for line in lines[0..1].into_iter() {
        numbers = line
            .clone()
            .split(",")
            .map(|num_str| num_str.parse().unwrap())
            .collect()
    }
    let mut bingo_boards: Vec<BingoBoard> = vec![];
    for line in lines[1..].chunks(5) {
        let result: Vec<i32> = line
            .iter()
            .flat_map(|l| {
                return l
                    .trim()
                    .split_ascii_whitespace()
                    .into_iter()
                    .map(|s| s.parse::<i32>().unwrap());
            })
            .collect();
        bingo_boards.push(BingoBoard::new(result));
    }
    let mut finished_board_score: Vec<i32> = vec![];
    let mut board_set: BTreeSet<i32> = BTreeSet::new();
    let number_of_boards = bingo_boards.len();
    'outer: for num in numbers {
        for (index, board) in bingo_boards.iter_mut().enumerate() {
            board.mark_complete(num);
            if board.is_complete() {
                let sum = board.get_sum();
                board_set.insert(index as i32);
                // println!("score {} num is {} index is {}", sum * num, num, index);
                if board_set.len() == number_of_boards {
                    println!("score {} num is {} index is {}", sum * num, num, index);
                    break 'outer;
                }
                finished_board_score.push(num * sum);
            }
        }
    }
    Ok(())
}
