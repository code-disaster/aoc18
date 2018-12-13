struct Map {
    w: i32,
    h: i32,
    grid: Vec<char>,
}

impl Map {
    fn new(input: &str) -> (Self, Vec<Cart>) {
        // first pass: size of grid
        let mut w: i32 = 0;
        let mut h: i32 = 0;

        for line in input.lines() {
            w = w.max(line.len() as i32);
            h += 1;
        }

        // second pass: fill grid
        let mut grid = Vec::with_capacity((w * h) as usize);

        for line in input.lines() {
            for ch in line.chars() {
                grid.push(ch);
            }
            for _ in line.len() as i32..w {
                grid.push(' ');
            }
        }

        let mut m = Map {
            w,
            h,
            grid,
        };

        let mut carts = Vec::new();

        // third pass: extract carts
        for y in 0..h {
            for x in 0..w {
                let t = m.get(x, y);
                match t {
                    '>' => {
                        carts.push(Cart::new(x, y, 0));
                        m.set(x, y, '-');
                    }
                    'v' => {
                        carts.push(Cart::new(x, y, 1));
                        m.set(x, y, '|');
                    }
                    '<' => {
                        carts.push(Cart::new(x, y, 2));
                        m.set(x, y, '-');
                    }
                    '^' => {
                        carts.push(Cart::new(x, y, 3));
                        m.set(x, y, '|');
                    }
                    _ => {}
                }
            }
        }

        (m, carts)
    }

    fn get(&self, x: i32, y: i32) -> char {
        if x < 0 || x >= self.w || y < 0 || y >= self.h {
            return ' ';
        }
        self.grid[(y * self.w + x) as usize]
    }

    fn set(&mut self, x: i32, y: i32, c: char) {
        self.grid[(y * self.w + x) as usize] = c;
    }

    fn tick(&mut self, carts: &mut Vec<Cart>) -> Vec<usize> {
        let w = self.w;
        carts.sort_by_key(|cart| cart.y * w + cart.x);
        let num_carts = carts.len();

        let mut crashes = Vec::new();

        for cart_idx in 0..num_carts {
            // check direction of cart, get position to move to
            let (x, y) = {
                let cart = &mut carts[cart_idx];

                let (dx, dy) = cart.dir();

                (cart.x + dx, cart.y + dy)
            };

            // check for collisions
            for other_idx in 0..num_carts {
                if cart_idx != other_idx {
                    let other_cart = &carts[other_idx];
                    if x == other_cart.x && y == other_cart.y {
                        crashes.push(cart_idx);
                        crashes.push(other_idx);
                    }
                }
            }

            // move
            {
                let cart = &mut carts[cart_idx];

                cart.x = x;
                cart.y = y;

                match self.get(x, y) {
                    '-' | '|' => { /* no change */ }
                    c => cart.turn(c)
                }
            }
        }

        crashes
    }
}

struct Cart {
    x: i32,
    y: i32,
    /// 0=right, 1=down, 2=left, 3=up
    dir: i8,
    /// 0=left, 1=straight, 2=right
    tick: i8,
}

impl Cart {
    fn new(x: i32, y: i32, dir: i8) -> Self {
        Cart {
            x,
            y,
            dir,
            tick: 0,
        }
    }

    fn dir(&self) -> (i32, i32) {
        match self.dir {
            0 => (1, 0),
            1 => (0, 1),
            2 => (-1, 0),
            3 => (0, -1),
            _ => unreachable!()
        }
    }

    fn turn(&mut self, c: char) {
        match c {
            '/' => {
                self.dir = match self.dir {
                    0 => 3,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => unreachable!()
                };
            }
            '\\' => {
                self.dir = match self.dir {
                    0 => 1,
                    1 => 0,
                    2 => 3,
                    3 => 2,
                    _ => unreachable!()
                };
            }
            '+' => {
                match self.tick {
                    0 => {
                        self.dir = (self.dir + 3) % 4;
                    }
                    1 => { /* nothing */ }
                    2 => {
                        self.dir = (self.dir + 1) % 4;
                    }
                    _ => unreachable!()
                };
                self.tick = (self.tick + 1) % 3;
            }
            _ => unreachable!()
        }
    }
}

pub fn main() {
    let input = include_str!("input.txt");

    let (mut map, mut carts) = Map::new(input);

    while carts.len() > 1 {
        let mut crashes = map.tick(&mut carts);
        for crash in &crashes {
            let cart = &carts[*crash];
            println!("crash #{} at {},{}", *crash, cart.x, cart.y);
        }
        crashes.sort();
        crashes.reverse();
        for crash in crashes {
            carts.remove(crash);
        }
    }

    let last = carts.first().unwrap();
    println!("last cart at {},{}", last.x, last.y);
}
