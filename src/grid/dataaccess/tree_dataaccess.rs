

use crate::grid;
use grid::models::grid_point::GridPoint;
use std::fs;


pub fn get_items(file_path: String) -> Vec<GridPoint> {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut deserialized_items: Vec<GridPoint> = serde_json::from_str(contents.as_str()).unwrap();
    deserialized_items.sort();
    return deserialized_items;
}