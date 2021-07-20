use rand::{Rng, thread_rng};    

pub fn bresenham_midpoint(center_x: i32, center_y: i32, radius: i32) -> Vec<(i32, i32)> {
    let mut x = radius;
    let mut y = 0;
    
    let mut ret_vals: Vec<(i32, i32)> = Vec::new();

    for i in -x + center_x..x + center_x {
        ret_vals.push((i, y + center_y));
    }

    //leerer Kreis
    ret_vals.push((x + center_x, y + center_y));
    ret_vals.push((-x + center_x, y + center_y));

    if radius > 0 {
        ret_vals.push((x + center_x, -y + center_y));
        ret_vals.push((-y + center_x, -x + center_y));
        ret_vals.push((-y + center_x, x + center_y));
    }

    let mut p = 1 - radius;
    while x > y {
        y += 1;

        if p <= 0 {
            p = p + 2 * y + 1;
        }
        else {
            x -= 1;
            p = p + 2 * y - 2 * x + 1;
        }

        if x < y {
            break;
        }

        //streckenweise oke implementierung für kreis füllen
        for i in -x + center_x..x + center_x {
            ret_vals.push((i, y + center_y));
        }
        for i in -x + center_x..x + center_x {
            ret_vals.push((i, -y + center_y));
        }

        //leerer kreis
        ret_vals.push((x + center_x, y + center_y));
        ret_vals.push((-x + center_x, y + center_y));
        ret_vals.push((x + center_x, -y + center_y));
        ret_vals.push((-x + center_x, -y + center_y));

        if x != y {
            //streckenweise oke implementierung für kreis füllen
            for i in -y + center_x..y + center_x {
                ret_vals.push((i, x + center_y));
            }
            for i in -y + center_x..y + center_x {
                ret_vals.push((i, -x + center_y));
            }

            //leerer kreis
            ret_vals.push((y + center_x, x + center_y));
            ret_vals.push((-y + center_x, x + center_y));
            ret_vals.push((y + center_x, -x + center_y));
            ret_vals.push((-y + center_x, -x + center_y));
        }
    }

    ret_vals
}

pub fn rand_dir() -> i32 {
    let mut rng = thread_rng();
    let i = rng.gen_range(0..1000);
    (i % 3) - 1
}