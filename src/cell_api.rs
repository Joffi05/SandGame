use crate::*;
use rand::{thread_rng, Rng};

pub const MAP_HEIGHT: usize = 10;
pub const MAP_WIDTH: usize = 10;
pub const CELL_SIZE: usize = 4;

pub const EMPTY_CELL: Cell = Cell {
    species: Species::Empty,
    ra: 0,
    rb: 0,
    clock: 0,
};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Cell {
    pub species: Species,
    pub ra: u8,
    pub rb: u8,
    pub clock: u8,
}
impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
}

impl PartialEq for Species {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug)]
pub struct CellApi {
    pub map: [[Cell; MAP_WIDTH]; MAP_HEIGHT],
    pub x: usize,
    pub y: usize,
}

#[cfg(test)]
#[test]
fn test_get() {
    let api = CellApi {
        map: [[EMPTY_CELL; MAP_HEIGHT]; MAP_WIDTH],
        x: 0,
        y: 0,
    };

    assert_eq!(api.map[0][0], EMPTY_CELL);
}

impl CellApi {
    pub fn get(&self, x: i32, y: i32) -> Cell {
        self.map[(self.x as i32 + x) as usize][(self.y as i32 + y) as usize]
    }
    pub fn set(&mut self, x: i32, y: i32, cell: Cell) {
        self.map[(self.x as i32 + x) as usize][(self.y as i32 + y) as usize] = cell;
    }
    
    pub fn update_sand(&mut self) {
        println!("{}", 1);
        let dx = rand_dir();
        println!("rand dir: {}", dx);
        println!("{}", 2);
        
        let neighbor = self.get(0, 1); 
        println!("{}", 3);

        if neighbor.species == Species::Empty { 
            println!("{}", 4);
            self.set(0, 0, EMPTY_CELL); 
            println!("{}", 5);
            self.set(0, 1, self.map[self.x][self.y]); 
        } else if self.get(dx, 1).species == Species::Empty { 
            println!("{}", 6);
            self.set(0, 0, EMPTY_CELL); 
            println!("{}", 7);
            self.set(dx, 1, self.map[self.x][self.y]);  
        }
    }
}

pub fn rand_dir() -> i32 {
    let mut rng = thread_rng();
    let ret = rng.gen_range(-1..1);
    ret
}