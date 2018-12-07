#[macro_use]
extern crate text_io;

pub fn main() {
    let input = include_str!("input.txt");

    let mut steps = Vec::new();
    let mut instructions = Vec::new();

    for line in input.lines() {
        let req: char;
        let step: char;
        scan!(line.bytes() => "Step {} must be finished before step {} can begin.", req, step);

        if !steps.contains(&req) {
            steps.push(req);
        }
        if !steps.contains(&step) {
            steps.push(step);
        }

        instructions.push(Instr { step, req });
    }

    steps.sort_by_key(|s| *s);

    // part 1
    let mut drain_s = steps.to_vec();
    let mut drain_i = instructions.to_vec();
    let mut order = Vec::new();
    while !drain_s.is_empty() {
        let mut next = 0;
        for (idx, step) in drain_s.iter().enumerate() {
            let mut has_deps = false;
            for instr in &drain_i {
                if *step == instr.step {
                    has_deps = true;
                    break;
                }
            }
            if !has_deps {
                next = idx;
                break;
            }
        }
        let s = drain_s.remove(next);
        order.push(s);
        drain_i.retain(|i| i.req != s);
    }

    print!("order of instructions: ");
    for s in &order {
        print!("{}", s);
    }
    print!("\n");
}

#[derive(Copy, Clone)]
struct Instr {
    step: char,
    req: char,
}
