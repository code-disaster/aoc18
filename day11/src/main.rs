const SERIAL_NUMBER: i32 = 4842;

pub fn main() {
    // part 1
    let (best_cell_x_3, best_cell_y_3, _) = largest(3);
    println!("largest total power 3x3: {},{}", best_cell_x_3, best_cell_y_3);

    // part 2
    let mut best_size = 0;
    let mut best_power_n = 0;
    let mut best_cell_x_n = best_cell_x_3;
    let mut best_cell_y_n = best_cell_y_3;

    for n in 1..300 {
        let (x, y, p) = largest(n);
        if best_power_n < p {
            best_size = n;
            best_power_n = p;
            best_cell_x_n = x;
            best_cell_y_n = y;
        }
        if n % 10 == 0 {
            println!(".");
        }
    }

    println!("largest total power NxN: {},{},{}", best_cell_x_n, best_cell_y_n, best_size);
}

fn largest(size: i32) -> (i32, i32, i32) {
    let mut best_cell_x = 0;
    let mut best_cell_y = 0;
    let mut best_power = 0;

    for y in 0..300 - size {
        for x in 0..300 - size {
            let mut p = 0;

            for yy in 0..size {
                for xx in 0..size {
                    p += power_level(x + xx, y + yy);
                }
            }

            if p > best_power {
                best_cell_x = x;
                best_cell_y = y;
                best_power = p;
            }
        }
    }

    (best_cell_x + 1, best_cell_y + 1, best_power)
}

fn power_level(x: i32, y: i32) -> i32 {
    let x = x + 1;
    let y = y + 1;
    let rack_id = x + 10;
    let power = (rack_id * y + SERIAL_NUMBER) * rack_id;
    ((power / 100) % 10) - 5
}
