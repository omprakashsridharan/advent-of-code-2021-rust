use crate::utils;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let values: Vec<i32> = utils::get_lines("data.txt")
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut current = -1;
    let mut result = 0;
    for w in values.windows(3) {
        let first = w[0];
        let second = w[1];
        let third = w[2];
        let sum = first + second + third;
        if current == -1 {
            current = sum
        } else {
            if current < sum {
                result += 1;
            }
            current = sum;
        }
    }
    println!("{}", result);
    Ok(())
}
