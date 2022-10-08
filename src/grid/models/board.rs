


use std::str::FromStr;

use quadtree_rs::{point::Point, Quadtree};
use crate::grid::dataaccess::tree_dataaccess;
use crate::grid::services::tree_service;
use crate::grid::models::{grid_screen::GridScreen, grid_point::GridPoint};


#[derive(Debug)]
pub struct Board {
    screen_dim: GridScreen,
    player_position: Point<i64>,
    quadtree: Quadtree::<i64, GridPoint>
}


impl Board {
    pub fn new(file_path: &str) -> Board {
        let deserialized_items = tree_dataaccess::get_items(String::from_str(file_path).expect("Failed to serialize str"));
        let quadtree = tree_service::build_tree(deserialized_items);
    
    
        //init on the top left
        let screen_dim = GridScreen::new(3,3);
        
        let player_position = Point{
            x: 0,
            y: 0
        };

        Board {
            screen_dim,
            player_position,
            quadtree
        }
    }

    pub fn get_items(&self) -> Vec<GridPoint>{
        let screen_area = self.screen_dim.into_area(self.player_position);
    
        
        let found_items = self.quadtree.query(screen_area);
        let mut items: Vec<GridPoint> = vec![];
        items.extend(found_items.map(|i|(*i.value_ref()).clone()));
        
    
        return items;
    }

    pub fn move_player(&mut self, point: Point<i64>) {
        self.player_position = point;
    }


}
