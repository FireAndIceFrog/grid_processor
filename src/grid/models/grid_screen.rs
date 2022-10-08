
use quadtree_rs::{area::{AreaBuilder, Area}, point::Point};


#[derive(Debug, Clone)]
pub struct GridScreen {
    pub width: i64,
    pub height: i64,
}

impl GridScreen {
    pub fn new(width: i64, height: i64) -> GridScreen {
        GridScreen {
            width,
            height,
        }   
    }

    pub fn into_tuple(&self) -> (i64, i64) {
        return (self.width, self.height)
    }

    pub fn into_area(&self, point: Point<i64>) -> Area<i64> {
        let area:  Area<i64> = AreaBuilder::<i64>::default()
            .anchor(point)
            .dimensions(self.into_tuple())
            .build()
            .expect("Failed to create area for grid");

        return area;
    }
}