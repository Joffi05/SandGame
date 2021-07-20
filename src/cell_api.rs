use crate::*;
use rand::{thread_rng, Rng};
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

pub const MAP_HEIGHT: i32 = 200;
pub const MAP_WIDTH: i32 = 200;
pub const CELL_SIZE: i32 = 2;

pub const EMPTY_CELL: Cell = Cell {
    species: Species::Empty,
    ra: 0,
    rb: 0,
    clock: true,
};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct Cell {
    pub species: Species,
    pub ra: u8,
    pub rb: i8,
    pub clock: bool,
}

impl Cell {
    pub fn new(species: Species) -> Cell {
        let mut rng = thread_rng();
        match species {
            Species::Wood => {
                Cell {
                    species,
                    ra: 30,
                    ..EMPTY_CELL
                }
            },
            Species::Fire => {
                Cell {
                    species,
                    ra: rng.gen_range(1..5),
                    ..EMPTY_CELL
                }
            },
            Species::Oil => {
                Cell {
                    species,
                    ra: 5,
                    ..EMPTY_CELL
                }
            },
            _ => {
                Cell {
                    species,
                    ..EMPTY_CELL
                }
            }
        }        
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum Species {
    Empty = 1,
    Wall, 
    Sand,
    Water,
    Wood,
    Fire,
    Oil,
}

impl Species {
    pub fn next(&self) -> Self {
        let x = FromPrimitive::from_u8(*self as u8 + 1);
        match x {
            Some(val) => val,
            None => *self,
        }
    }
    pub fn last(&self) -> Self {
        let x = FromPrimitive::from_u8(*self as u8 - 1);
        match x {
            Some(val) => val,
            None => *self,
        }
    }

    pub fn is_burnable(&self) -> bool {
        self == &Species::Wood ||
        self == &Species::Oil
    }

    pub fn density(&self) -> i32 {
        match self {
            Species::Empty => 0,
            Species::Fire => 1,
            Species::Oil => 20,
            Species::Water => 50,
            Species::Sand => 100,
            _ => 1_000_000,
        }
    }
}

#[derive(Debug)]
pub struct CellApi {
    pub map: Vec<Cell>,
    pub clock: bool,
    pub x: i32,
    pub y: i32,
}

#[cfg(test)]
#[test]
fn test_get() {
    let api = CellApi {
        map: vec![EMPTY_CELL; (MAP_WIDTH * MAP_HEIGHT) as usize],
        clock: true,
        x: 0,
        y: 0,
    };

    assert_eq!(api.map[CellApi::get_index(0, 0)], EMPTY_CELL);
}

impl CellApi {
    pub fn get(&self, x: i32, y: i32) -> Cell {
        if x > 2 || x < -2 || y > 2 || y < -2 {
            panic!("dx or dy bigger 2");
        }
        
        let dx = self.x + x;
        let dy = self.y + y;

        if dx < 0 || dx > MAP_WIDTH - 1 || dy < 0 || dy > MAP_HEIGHT - 1 {
            return Cell {
                species: Species::Wall,
                ra: 0,
                rb: 0,
                clock: true,
            };
        }
        else {
            return self.map[CellApi::get_index(dx, dy)];
        }
    }
    pub fn spawn(&mut self, x: i32, y: i32, radius: i32, species: Species) {
        let point_vec = bresenham_midpoint(x, y, radius);

        //hier aufpassen
        self.x = x;
        self.y = y;

        for point in point_vec {
            self.set(point.0, point.1, Cell::new(species));
        }
    }
    pub fn set(&mut self, x: i32, y: i32, cell: Cell) {
        let dx = self.x + x;
        let dy = self.y + y;

        if !(dx < 0 || dx > MAP_WIDTH - 1 || dy < 0 || dy > MAP_HEIGHT - 1) {
            self.map[CellApi::get_index(dx, dy)] = cell;   
        }
    }
    pub fn get_index(x: i32, y: i32) -> usize {
        (x * MAP_HEIGHT + y) as usize
    }
    pub fn get_adjacent(&self) -> [(i32, i32); 8] {
        [(-1, -1), (0, -1), (1, -1), 
         (-1, 0),           (1, 0),
         (-1, 1),  (0, 1),  (1, 1)]
    }
}