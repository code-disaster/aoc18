#[macro_use]
extern crate text_io;

const INITIAL_STATE: &str = "##.##.##..#..#.#.#.#...#...#####.###...#####.##..#####.#..#.##..#..#.#...#...##.##...#.##......####.";

pub fn main() {
    let input = include_str!("input.txt");

    let mut notes = Vec::new();
    for line in input.lines() {
        notes.push(Note::new(line));
    }

    //let mut state = format!("..{}..", INITIAL_STATE);
    let mut state = String::from(INITIAL_STATE);
    let mut start_index = 0;

    for _ in 0..20 {
        let (s, di) = grow(state, &notes);
        state = s;
        start_index += di;
    }

    let mut sum: i64 = 0;

    for (idx, ch) in state.chars().enumerate() {
        if ch == '#' {
            sum += start_index + idx as i64;
        }
    }

    println!("sum of pot numbers: {}", sum);
}

fn grow(state: String, notes: &Vec<Note>) -> (String, i64) {
    let state = format!("..{}..", state);
    let mut target = state.clone().replace("#", ".");

    for pot in 0..state.len() - 4 {
        let pattern = &state[pot..pot + 5];
        for note in notes {
            if pattern == note.pattern {
                target.replace_range(pot + 2..pot + 3, note.target.as_str());
                break;
            }
        }
    }

    (target, -2)
}

#[derive(Debug)]
struct Note {
    pattern: String,
    target: String,
}

impl Note {
    fn new(line: &str) -> Self {
        let p: String;
        let t: String;
        scan!(line.bytes() => "{} => {}", p, t);
        Note {
            pattern: p,
            target: t,
        }
    }
}
