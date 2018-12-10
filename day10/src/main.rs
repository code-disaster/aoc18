#[macro_use]
extern crate text_io;

pub fn main() {
    let input = include_str!("input.txt");

    let mut points = Vec::new();
    for line in input.lines() {
        points.push(Point::new(line));
    }

    let mut last_width = 1000000;
    let mut last_height = 1000000;

    for sec in 9000..11000 {
        let (_aligned, width, height) = align(&points, sec);
        if width > last_width || height > last_height {
            let (last_aligned, _, _) = align(&points, sec - 1);
            print(&last_aligned, sec - 1);
            break;
        }
        last_width = width;
        last_height = height;
    }
}

struct Point {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

impl Point {
    fn new(line: &str) -> Self {
        let x: String;
        let y: String;
        let vx: String;
        let vy: String;
        scan!(line.bytes() => "position=<{}, {}> velocity=<{}, {}>", x, y, vx, vy);
        Point {
            x: x.trim().parse::<i32>().unwrap(),
            y: y.trim().parse::<i32>().unwrap(),
            vx: vx.trim().parse::<i32>().unwrap(),
            vy: vy.trim().parse::<i32>().unwrap(),
        }
    }
}

fn align(points: &Vec<Point>, after_seconds: i32) -> (Vec<Point>, i32, i32) {
    let mut result = Vec::new();

    let mut min_x = 99999;
    let mut min_y = 99999;
    let mut max_x = -99999;
    let mut max_y = -99999;

    for p in points {
        let ap = Point {
            x: p.x + p.vx * after_seconds,
            y: p.y + p.vy * after_seconds,
            vx: p.vx,
            vy: p.vy,
        };

        min_x = min_x.min(ap.x);
        min_y = min_y.min(ap.y);
        max_x = max_x.max(ap.x);
        max_y = max_y.max(ap.y);

        result.push(ap);
    }

    (result, max_x - min_x, max_y - min_y)
}

const WIDTH: i32 = 300;
const HEIGHT: i32 = 200;

fn print(points: &Vec<Point>, after_seconds: i32) {
    let mut map = [['.'; HEIGHT as usize]; WIDTH as usize];

    for p in points {
        if p.x >= 0 && p.y >= 0 && p.x < WIDTH && p.y < HEIGHT {
            map[p.x as usize][p.y as usize] = '#';
        }
    }

    println!("after {} seconds", after_seconds);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", map[x as usize][y as usize]);
        }
        print!("\n");
    }
}
