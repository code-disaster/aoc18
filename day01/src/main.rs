use std::collections::HashSet;

pub fn main() {
    let input = include_str!("input.txt");

    // part 1
    {
        let mut sum = 0;

        for line in input.lines() {
            let n = line.parse::<i32>().unwrap();
            sum += n;
        }

        println!("sum = {}", sum);
    }

    // part 2
    {
        let mut sum = 0;
        let mut freqs = HashSet::new();
        let mut found: Option<i32> = None;

        while found.is_none() {
            for line in input.lines() {
                let n = line.parse::<i32>().unwrap();
                sum += n;

                if freqs.contains(&sum) {
                    found = Some(sum);
                    break;
                }

                freqs.insert(sum);
            }
        }

        println!("freq = {}", found.unwrap());
    }
}
