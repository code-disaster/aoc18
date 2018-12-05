extern crate regex;

use regex::Regex;
use std::borrow::Cow;

pub fn main() {
    let input = include_str!("input.txt");

    // part 1
    {
        let mut polymer = String::from(input);
        let mut mutated = true;
        let mut iterations = 0;

        while mutated {
            mutated = false;
            iterations += 1;
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

        println!("remains: len={} after {} iterations", polymer.len(), iterations);
    }
}

fn reduce(polymer: &String, l: i32, u: i32) -> (bool, String) {
    let pa = format!("{}{}", l as u8 as char, u as u8 as char);
    let re = Regex::new(pa.as_str()).unwrap();
    let rs = re.replace(polymer.as_str(), "");

    let mutation = String::from(rs.into_owned());
    let shrink = polymer.len() - mutation.len();

    (shrink != 0, mutation)
}
