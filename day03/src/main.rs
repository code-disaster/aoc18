#[macro_use]
extern crate text_io;

pub fn main() {
    let input = include_str!("input.txt");
    let mut claims = Vec::new();
    let mut map_w = 0;
    let mut map_h = 0;

    // parse
    {
        for line in input.lines() {
            let id: usize;
            let x: usize;
            let y: usize;
            let w: usize;
            let h: usize;
            scan!(line.bytes() => "#{} @ {},{}: {}x{}", id, x, y, w, h);
            claims.push(Claim {
                id,
                x,
                y,
                w,
                h,
            });
            map_w = map_w.max(x + w);
            map_h = map_h.max(y + h);
        }
    }

    let mut map: Vec<i32> = Vec::with_capacity(map_w * map_h);
    for _xy in 0..map_w * map_h {
        map.push(0);
    }

    // part 1
    {
        for c in &claims {
            for y in c.y..c.y + c.h {
                for x in c.x..c.x + c.w {
                    let xy = y * map_w + x;
                    map[xy] += 1;
                }
            }
        }

        let mut two_or_more = 0;
        for xy in 0..map_w * map_h {
            if map[xy] > 1 {
                two_or_more += 1;
            }
        }
        println!("two or more claims: {}", two_or_more);
    }

    // part 2
    {
        for c in &claims {
            let mut no_overlap = true;
            'outer: for y in c.y..c.y + c.h {
                for x in c.x..c.x + c.w {
                    let xy = y * map_w + x;
                    if map[xy] > 1 {
                        no_overlap = false;
                        break 'outer;
                    }
                }
            }
            if no_overlap {
                println!("claim w/o overlap: #{}", c.id);
                break;
            }
        }
    }
}

#[derive(Debug)]
struct Claim {
    id: usize,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}