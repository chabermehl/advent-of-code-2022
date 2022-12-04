use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file not found");
    let lines: Vec<&str> = contents.lines().collect();

    calculate_complete_overlaps(&lines);
    calculate_partial_overlaps(&lines);
}

fn calculate_complete_overlaps(elves: &Vec<&str>) {
    let mut complete_overlaps = 0;

    for elf_pair in elves {
        let ranges: Vec<&str> = elf_pair.split(",").collect();

        let elf_1: Vec<i32> = ranges[0].split("-").map(|f| f.parse().unwrap()).collect();
        let elf_2: Vec<i32> = ranges[1].split("-").map(|f| f.parse().unwrap()).collect();

        if (elf_1[0] <= elf_2[0] && elf_1[1] >= elf_2[1])
            || (elf_1[0] >= elf_2[0] && elf_1[1] <= elf_2[1])
        {
            complete_overlaps += 1;
        }
    }

    println!("Complete Overlaps: {complete_overlaps}");
}

fn calculate_partial_overlaps(elves: &Vec<&str>) {
    let mut partial_overlaps = 0;

    for elf_pair in elves {
        let ranges: Vec<&str> = elf_pair.split(",").collect();

        let elf_1: Vec<i32> = ranges[0].split("-").map(|f| f.parse().unwrap()).collect();
        let elf_2: Vec<i32> = ranges[1].split("-").map(|f| f.parse().unwrap()).collect();

        if (elf_1[0] <= elf_2[0] && elf_2[0] <= elf_1[1])
            || (elf_2[0] <= elf_1[0] && elf_1[0] <= elf_2[1])
        {
            partial_overlaps += 1;
        }
    }

    println!("Partial Overlaps: {partial_overlaps}");
}
