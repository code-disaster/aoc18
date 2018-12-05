extern crate regex;

use regex::Regex;

pub fn main() {
    let input = include_str!("input.txt");

    // part 1
    let result = react(input);
    println!("remains: len={}", result.len());

    // part 2
    let mut shortest: usize = result.len();
    for letter in 0..26 {
        let l = 'a' as i32 + letter;
        let u = 'A' as i32 + letter;

        let removed = react(&remove(&result, l, u));
        shortest = shortest.min(removed.len());
    }
    println!("shortest: len={}", shortest);
}

fn react(input: &str) -> String {
    let mut polymer = String::from(input);
    let mut mutated = true;

    while mutated {
        mutated = false;
        for letter in 0..26 {
            let l = 'a' as i32 + letter;
            let u = 'A' as i32 + letter;

            let result = reduce(&polymer, l, u);
            mutated |= result.0;
            polymer = result.1;

            let result = reduce(&polymer, u, l);
            mutated |= result.0;
            polymer = result.1;
        }
    }

    polymer
}

fn reduce(polymer: &String, l: i32, u: i32) -> (bool, String) {
    let pa = format!("{}{}", l as u8 as char, u as u8 as char);
    let re = Regex::new(pa.as_str()).unwrap();
    let rs = re.replace_all(polymer.as_str(), "");

    let mutation = String::from(rs.into_owned());
    let shrink = polymer.len() - mutation.len();

    (shrink != 0, mutation)
}

fn remove(polymer: &String, l: i32, u: i32) -> String {
    let pa = format!("[{}{}]", l as u8 as char, u as u8 as char);
    let re = Regex::new(pa.as_str()).unwrap();
    let rs = re.replace_all(polymer.as_str(), "");

    String::from(rs.into_owned())
}
