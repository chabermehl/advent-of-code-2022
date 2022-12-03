use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file not found");
    let lines: Vec<&str> = contents.lines().map(|l| l).collect();
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();

    let mut total_prio = 0;

    for chunk in lines.chunks(3) {
        if let [elf_1, elf_2, elf_3] = *chunk {
            let elf_1_chars: Vec<char> = elf_1.chars().map(|c| c).collect();
            let elf_2_chars: Vec<char> = elf_2.chars().map(|c| c).collect();
            let elf_3_chars: Vec<char> = elf_3.chars().map(|c| c).collect();

            let partial_intersections = determine_intersections(&elf_1_chars, &elf_2_chars);
            let complete_intersections =
                determine_intersections(&partial_intersections, &elf_3_chars);

            let priority = alphabet
                .iter()
                .position(|c| *c == complete_intersections[0])
                .unwrap();

            total_prio += priority + 1;
        };
    }

    println!("{total_prio}")
}

fn determine_intersections(a: &Vec<char>, b: &Vec<char>) -> Vec<char> {
    a.into_iter()
        .cloned()
        .filter(|x| b.contains(x))
        .map(|f| f)
        .collect()
}
