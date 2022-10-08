use serde::{Serialize, Deserialize};
use std::cmp::Ordering;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Clone)]
pub struct GridPoint {
    pub x: i64,
    pub y: i64,
}

impl Ord for GridPoint {
    fn cmp(&self, other: &Self) -> Ordering{ 
        let x_cmp = self.x.cmp(&other.x);
        let y_cmp = match x_cmp {
            std::cmp::Ordering::Equal => self.y.cmp(&other.y),
            _ => x_cmp,
        };
        return y_cmp;
    }
}