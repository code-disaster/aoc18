pub fn main() {
    let input = include_str!("input.txt");

    // part 1
    {
        let mut twos = 0;
        let mut threes = 0;

        for line in input.lines() {
            let mut map: [i32; 26] = [0; 26];
            count(&mut map, line);
            if map.contains(&2) { twos += 1; }
            if map.contains(&3) { threes += 1; }
        }

        let checksum = twos * threes;
        println!("checksum = {}", checksum);
    }

    // part 2
    {
        let mut lines = Vec::new();
        for l in input.lines() { lines.push(l); }
        let len = lines.len();

        'outer: for idxa in 0..len {
            let la = lines[idxa];
            for idxb in idxa + 1..len {
                let lb = lines[idxb];

                let mut chars_a = la.chars();
                let mut chars_b = lb.chars();
                let c_len = la.len().min(lb.len());

                for idxc in 0..c_len {
                    let ca = chars_a.next().unwrap();
                    let cb = chars_b.next().unwrap();
                    if ca != cb {
                        if la[idxc + 1..] == lb[idxc + 1..] {
                            println!("common = {}{}", &la[..idxc], &la[idxc + 1..]);
                            break 'outer;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
}

fn count(map: &mut [i32; 26], input: &str) {
    for c in input.bytes() {
        let idx = (c - 'a' as u8) as usize;
        map[idx] += 1;
    }
}
