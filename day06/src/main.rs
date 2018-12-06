#[macro_use]
extern crate text_io;

pub fn main() {
    let input = include_str!("input.txt");
    let mut coords = Vec::new();

    for line in input.lines() {
        let x: i32;
        let y: i32;
        scan!(line.bytes() => "{}, {}", x, y);
        coords.push(Coord { x, y });
    }

    let mut map = Map::new(&coords);
    let total_size = map.populate(&coords);

    let max_area = map.largest(&coords);
    println!("size of largest area: {}", max_area);

    println!("size of total distance region: {}", total_size);
}

struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn manhattan_dist(&self, x: i32, y: i32) -> i32 {
        let dx = self.x - x;
        let dy = self.y - y;
        dx.abs() + dy.abs()
    }
}

struct Map {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    nearest: Vec<i8>,
}

impl Map {
    fn new(coords: &Vec<Coord>) -> Self {
        let mut min_x = coords[0].x;
        let mut max_x = min_x;
        let mut min_y = coords[0].y;
        let mut max_y = min_y;

        for xy in coords {
            min_x = min_x.min(xy.x);
            max_x = max_x.max(xy.x);
            min_y = min_y.min(xy.y);
            max_y = max_y.max(xy.y);
        }

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        Map {
            x: min_x,
            y: min_y,
            width,
            height,
            nearest: vec![0; (width * height) as usize],
        }
    }

    fn populate(&mut self, coords: &Vec<Coord>) -> i32 {
        let mut total_size = 0;

        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                let mut nearest: i32 = 0;
                let mut nearest_count: i32 = 1;
                let mut nearest_dist = coords[0].manhattan_dist(x, y);
                let mut total = nearest_dist;

                for idx in 1..coords.len() {
                    let dist = coords[idx].manhattan_dist(x, y);
                    total += dist;
                    if dist == nearest_dist {
                        nearest_count += 1;
                    } else if dist < nearest_dist {
                        nearest = idx as i32;
                        nearest_count = 1;
                        nearest_dist = dist;
                    }
                }

                if nearest_count > 1 {
                    nearest = -1;
                }

                if total < 10000 {
                    total_size += 1;
                }

                let mx = x - self.x;
                let my = y - self.y;

                self.nearest[(my * self.width + mx) as usize] = nearest as i8;
            }
        }

        total_size
    }

    fn largest(&self, coords: &Vec<Coord>) -> i32 {
        let mut area = vec![0; coords.len()];

        for y in self.y..self.y + self.height {
            for x in self.x..self.x + self.width {
                let mx = x - self.x;
                let my = y - self.y;

                let idx = self.nearest[(my * self.width + mx) as usize];
                if idx >= 0 {
                    area[idx as usize] += 1;
                }
            }
        }

        for x in 0..self.width {
            let x0 = self.nearest[x as usize];
            if x0 >= 0 {
                area[x0 as usize] = 0;
            }

            let x1 = self.nearest[(self.width * self.height - 1 - x) as usize];
            if x1 >= 0 {
                area[x1 as usize] = 0;
            }
        }

        for y in 0..self.height {
            let y0 = self.nearest[(y * self.width) as usize];
            if y0 >= 0 {
                area[y0 as usize] = 0;
            }

            let y1 = self.nearest[(y * self.width + self.width - 1) as usize];
            if y1 >= 0 {
                area[y1 as usize] = 0;
            }
        }

        let mut max_area = area[0];
        for idx in 1..coords.len() {
            if area[idx] > max_area {
                max_area = area[idx];
            }
        }

        max_area
    }
}
