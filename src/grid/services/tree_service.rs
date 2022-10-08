

use crate::grid;

use quadtree_rs::{point::Point, Quadtree};
use grid::models::grid_point::GridPoint;

fn get_max_pow_2_depth(x:i64) -> u64 {
    if x == 0 {
        return 0;
    }
    let max_int = 1_u64<<63;
    if (x as u64) < max_int {
        let mut curr_val = 1;
        for depth in 1_u64..64_u64 {
            if curr_val >= x {
                return depth;
            }

            curr_val = curr_val << 1;
        }
    }

    let xfloat: f64 = x as f64;
    let logx = xfloat.log10();
    let cielx = (logx/2.0_f64.log2()).ceil();
    let uintx: u64;
    unsafe {
        uintx = cielx.to_int_unchecked::<u64>()
    }
    return uintx;
}

pub fn build_tree(items: Vec<GridPoint>) -> Quadtree::<i64, GridPoint> {
    
    let depth = get_max_pow_2_depth(items.len() as i64) as usize;
    let mut qt = Quadtree::<i64, GridPoint>::new(depth);
    
    for item in items {
        qt.insert_pt(Point {x: item.x, y: item.y}, item);
    }

    return qt;
}
