use std::env;
use std::fs;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !(args.len() > 2) {
        return println!("not enough args specified");
    }
    let needle = &args[1];
    let haystack_name = &args[2];

    let haystack = fs::read_to_string(haystack_name).expect("Error loading file");

    if haystack.to_lowercase().contains(&needle.to_lowercase()) {
        println!(
            "{}",
            format!(
                "{}\n{}\n{}",
                "-".repeat(25),
                format!("{} contains \"{}\"", haystack_name, needle),
                "-".repeat(25)
            )
        );
        let mut occurences = Vec::new();
        let mut index: i32 = 0;
        let start = Instant::now();
        for line in haystack.lines() {
            index += 1;
            if line.to_lowercase().contains(&needle.to_lowercase()) {
                occurences.push(format!("Line {}: {}", index, line));
            }
        }
        for appearance in occurences {
            println!("{}", appearance);
        }
        println!(
            "\nFound all occurences in {} seconds.",
            start.elapsed().as_secs()
        );
    }
}
