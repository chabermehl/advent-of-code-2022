use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file not found");
    let lines = contents.lines();

    let calories_arr: Vec<i32> = lines.map(|line| line.parse().unwrap_or(0)).collect();

    let mut elves = Vec::new();
    let mut calories = 0;
    for c in calories_arr {
        match c {
            0 => {
                elves.push(calories);
                calories = 0;
            }
            _ => calories += c,
        }
    }

    elves.sort();
    elves.reverse();
    let top3 = &elves[..3];
    let top_sum = top3.iter().sum::<i32>();

    println!("{} {:?}", elves[0], top_sum)
}
