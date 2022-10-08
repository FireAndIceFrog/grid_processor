mod grid;

use std::io::{self, Write};

use grid::models::grid_point::GridPoint;
use quadtree_rs::point::Point;
use serde_json::from_str;

use crate::grid::models::board::Board;

fn print_items(items: &Vec<GridPoint>) {
    
    println!("Total items: {:?}", items.iter().fold(0, |i,_|i+1));
        
    for item in items.iter() {
        println!("Item: {:?}", item);
    }
}

fn get_input(prompt: &str) -> String{
    print!("{}",prompt);
    let mut out = io::stdout();
    out.flush().expect("Failed to flush stdout");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

fn main() {
    let file_path = "A:\\Github\\grid_processor\\data\\five_item_grid.json";
    let mut board = Board::new(file_path);
    loop {
        let x: i64 = from_str::<i64>(get_input("X: ").as_str()).expect("Failed to convert to int");
        let y: i64 = from_str::<i64>(get_input("Y: ").as_str()).expect("Failed to convert to int");
        
        board.move_player(Point {
            x,
            y
        });

        
        let items = board.get_items();
        print_items(&items);

    }
}
