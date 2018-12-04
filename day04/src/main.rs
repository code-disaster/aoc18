#[macro_use]
extern crate text_io;

#[derive(Debug)]
struct Record {
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    text: String,
}

impl Record {
    fn timestamp(&self) -> i32 {
        (((((self.month << 5) + self.day) << 5) + self.hour) << 6) + self.minute
    }
}

#[derive(Debug, Copy, Clone)]
struct Nap {
    month: i32,
    day: i32,
    guard: i32,
    from: i32,
    to: i32,
}

pub fn main() {
    let input = include_str!("input.txt");
    let mut records = Vec::new();

    // parse & sort
    {
        for line in input.lines() {
            let month: i32;
            let mut day: i32;
            let mut hour: i32;
            let minute: i32;
            let _ignore_text: String;
            let event_text: String;
            scan!(line.bytes() => "[1518-{}-{} {}:{}] {} {}", month, day, hour, minute, _ignore_text, event_text);

            // hack: shift 23:xx times to "-01:xx" on the next day;
            // only works because time stamp is just used for sorting
            if hour == 23 {
                day += 1;
                hour = -1;
            }

            records.push(Record {
                month,
                day,
                hour,
                minute,
                text: event_text,
            });
        }

        records.sort_by_key(|r| r.timestamp());
    }

    // convert
    let mut naps = Vec::new();

    {
        let mut guard: i32 = 0;
        let mut from: i32 = 0;

        for r in &records {
            if r.text.contains("asleep") {
                from = r.minute;
            } else if r.text.contains("up") {
                naps.push(Nap {
                    month: r.month,
                    day: r.day,
                    guard,
                    from,
                    to: r.minute,
                });
            } else {
                scan!(r.text.bytes() => "#{}", guard);
            }
        }
    }

    // part 1&2
    {
        let mut guard_naps = Vec::new();
        guard_naps.clone_from(&naps);
        guard_naps.sort_by_key(|n| n.guard);

        let mut guard = 0;
        let mut sleep: [i32; 60] = [0; 60];

        let mut top_guard = 0;
        let mut top_minute = 0;
        let mut top_score = 0;

        let mut top2_guard = 0;
        let mut top2_minute = 0;
        let mut top2_score = 0;

        for n in guard_naps {
            if n.guard != guard {
                let mut score = 0;
                let mut best_minute = 0;
                for m in 0..60 {
                    score += sleep[m];
                    if sleep[m] > sleep[best_minute] {
                        best_minute = m;
                    }
                }

                if score > top_score {
                    top_guard = guard;
                    top_minute = best_minute;
                    top_score = score;
                }

                if sleep[best_minute] > top2_score {
                    top2_guard = guard;
                    top2_minute = best_minute;
                    top2_score = sleep[best_minute];
                }

                sleep = [0; 60];
                guard = n.guard;
            }

            for m in n.from..n.to {
                sleep[m as usize] += 1;
            }
        }

        let result1 = top_guard * top_minute as i32;
        println!("result #1 = {} (guard #{}, minute {}, score {})", result1, top_guard, top_minute, top_score);

        let result2 = top2_guard * top2_minute as i32;
        println!("result #2 = {} (guard #{}, minute {}, score {})", result2, top2_guard, top2_minute, top2_score);
    }
}
