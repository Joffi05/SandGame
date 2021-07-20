use crate::*;
use rand::{thread_rng, Rng};

pub fn update_sand(api: &mut CellApi) {
    let cell = api.get(0, 0);
    let dx = rand_dir();
    
    let neighbor = api.get(0, 1); 

    if neighbor.species.density() < cell.species.density() { 
        //her wird oft kopiert iwie ändern
        api.set(0, 0, neighbor); 
        api.set(0, 1, cell); 
    } 
    else if api.get(dx, 1).species.density() < cell.species.density() { 
        api.set(0, 0, api.get(dx, 1)); 
        api.set(dx, 1, cell);  
    }
}

pub fn update_water(api: &mut CellApi) {
    let cell = api.get(0, 0);
    let dx = rand_dir();
    let neighbor = api.get(0, 1); 

    if neighbor.species.density() < cell.species.density() { 
        //her wird oft kopiert iwie ändern
        api.set(0, 0, neighbor); 
        api.set(0, 1, cell); 
    } 
    else if api.get(dx, 1).species.density() < cell.species.density() { 
        api.set(0, 0, api.get(dx, 1)); 
        api.set(dx, 1, cell);  
    }
    else if api.get(dx, 0).species.density() < cell.species.density() {
        api.set(0, 0, api.get(dx, 0)); 
        api.set(dx, 0, cell); 
    }       
}

pub fn update_wood(api: &mut CellApi) {
    let cell = api.get(0, 0);

    if cell.ra == 0 {
        api.set(0, 0, EMPTY_CELL);

        for i in api.get_adjacent() {
            let cell = api.get(i.0, i.1);
            if cell.species.is_burnable() {
                if api.get(i.0, i.1).ra >= 1 {
                    api.map[CellApi::get_index(api.x + i.0, api.y + i.1)].ra -= 1;
                }
            }
        } 
    }
    else if cell.ra != 30 {
        let mut rng = thread_rng();
        if rng.gen_bool(1.0 / 10.0) {
            api.map[CellApi::get_index(api.x, api.y)].ra -= 1;
        }  
        
        for i in api.get_adjacent() {
            let cell = api.get(i.0, i.1);
            if cell.species == Species::Empty {
                if rng.gen_bool(1.0 / 3.0) {
                    api.set(i.0, i.1, Cell::new(Species::Fire));
                }
            }
        } 
    }
}

pub fn update_fire(api: &mut CellApi) {
    let mut rng = thread_rng();
    let cell = api.get(0, 0);
    let dx = rand_dir();
    let is_cold = cell.ra == 0;
    let rand_val = rng.gen_bool(1.0 / 4.0);

    let neighbor = api.get(0, -1);
    if is_cold {
        api.set(0, 0, EMPTY_CELL);
    }
    else {
        if rand_val {
            api.map[CellApi::get_index(api.x, api.y)].ra -= 1;
        }
        //hier noch density check implementieren grad zu faul
        if neighbor == EMPTY_CELL { 
            //her wird oft kopiert iwie ändern 
            if !rand_val {
                api.set(0, 0, EMPTY_CELL);
                api.set(dx, -1, cell);
            }
            else {
                api.set(0, 0, EMPTY_CELL);
                api.set(0, -1, cell);
            }
        } 

        for i in api.get_adjacent() {
            let cell = api.get(i.0, i.1);
            if cell.species.is_burnable() {
                if rng.gen_bool(1.0 / 2.0) {
                    api.set(0, 0, EMPTY_CELL);
                    if api.map[CellApi::get_index(api.x + i.0, api.y + i.1)].ra > 1 {
                        api.map[CellApi::get_index(api.x + i.0, api.y + i.1)].ra -= 1;
                    }
                }
            }
        } 
        //old burning is also quite good
        /*else if neighbor.is_burnable() {
            if rng.gen_bool(1.0 / 2.0) {
                api.set(0, 0, EMPTY_CELL);
                if api.map[CellApi::get_index(api.x, api.y - 1)].ra > 1 {
                    api.map[CellApi::get_index(api.x, api.y - 1)].ra -= 1;
                }
            }
        }*/
    }
}

pub fn update_oil(api: &mut CellApi) {
    let cell = api.get(0, 0);
    let dx = rand_dir();
    let neighbor = api.get(0, 1); 

    if cell.ra == 0 {
        api.set(0, 0, EMPTY_CELL);

        for i in api.get_adjacent() {
            let cell = api.get(i.0, i.1);
            if cell.species.is_burnable() {
                if cell.ra >= 1 {
                    api.map[CellApi::get_index(api.x + i.0, api.y + i.1)].ra -= 1;
                }
            }
        } 
    }
    else if cell.ra != 5 {
        let mut rng = thread_rng();
        if rng.gen_bool(1.0 / 10.0) {
            if cell.ra >= 1 {
                api.map[CellApi::get_index(api.x, api.y)].ra -= 1;
            }
        }  
        
        for i in api.get_adjacent() {
            let cell = api.get(i.0, i.1);
            if cell.species == Species::Empty {
                if rng.gen_bool(1.0 / 3.0) {
                    api.set(i.0, i.1, Cell::new(Species::Fire));
                }
            }
        } 
    }

    if neighbor.species.density() < cell.species.density() { 
        //her wird oft kopiert iwie ändern
        api.set(0, 0, neighbor); 
        api.set(0, 1, cell); 
    } 
    else if api.get(dx, 1).species.density() < cell.species.density() { 
        api.set(0, 0, api.get(dx, 1)); 
        api.set(dx, 1, cell);  
    }
    else if api.get(dx, 0).species.density() < cell.species.density() {
        api.set(0, 0, api.get(dx, 0)); 
        api.set(dx, 0, cell); 
    }  
}