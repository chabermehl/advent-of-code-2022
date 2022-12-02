use std::fs;

// A = Rock, 1
// B = Paper, 2
// C = Scissors, 3

// X = Lose
// Y = Draw
// Z = Win

// Lose 0
// Draw 3
// Win 6

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file not found");
    let lines = contents.lines();

    let mut score = 0;

    for line in lines {
        let l: Vec<&str> = line.split(" ").collect();

        match l[0] {
            "A" => match l[1] {
                "X" => score += 3 + 0,
                "Y" => score += 1 + 3,
                "Z" => score += 2 + 6,
                _ => panic!("Unrecognized character"),
            },
            "B" => match l[1] {
                "X" => score += 1 + 0,
                "Y" => score += 2 + 3,
                "Z" => score += 3 + 6,
                _ => panic!("Unrecognized character"),
            },
            "C" => match l[1] {
                "X" => score += 2 + 0,
                "Y" => score += 3 + 3,
                "Z" => score += 1 + 6,
                _ => panic!("Unrecognized character"),
            },
            _ => panic!("Unrecognized character"),
        }
    }
    println!("{score}");
}
