pub fn rightbottom_corner(api: &mut CellApi) {
    let dx = rand_dir();
    
    let neighbor = api.get(0, 1); 

    if neighbor == EMPTY_CELL { 
        api.set(0, 1, api.map[CellApi::get_index(api.x, api.y)]); 
        api.set(0, 0, EMPTY_CELL); 
    } else if api.get(1, dx) == EMPTY_CELL { 
        api.set(1, dx, api.map[CellApi::get_index(api.x, api.y)]);  
        api.set(0, 0, EMPTY_CELL); 
    }
}

pub fn tropfsteine(api: &mut CellApi) {
    let cell = api.map[CellApi::get_index(api.x, api.y)];
    let dx = rand_dir();
    
    let neighbor = api.get(0, -1); 

    if neighbor == EMPTY_CELL { 
        //her wird oft kopiert iwie ändern
        api.set(0, 0, EMPTY_CELL); 
        api.set(0, -1, cell); 
    } 
}

pub fn laser(api: &mut CellApi) {
    let cell = api.get(0, 0);
    let dx = rand_dir();
    
    for i in 0..10 {
        api.x = api.x + i;
        api.y = api.y + i;

        let neighbor = api.get(0, 1); 

        if neighbor == EMPTY_CELL { 
            //her wird oft kopiert iwie ändern
            api.set(0, 0, EMPTY_CELL); 
            api.set(0, 1, cell); 
        } 
        else if api.get(dx, 1) == EMPTY_CELL { 
            api.set(0, 0, EMPTY_CELL); 
            api.set(dx, 1, cell);  
        }
        else if api.get(dx, 0) == EMPTY_CELL {
            api.set(0, 0, EMPTY_CELL); 
            api.set(dx, 0, cell); 
        }
    } 
}

pub fn old_wood(api: &mut CellApi) {
    let cell = api.get(0, 0);
    let mut rng = thread_rng();

    if cell.ra == 0 {
        api.set(0, 0, EMPTY_CELL);
        return;
    }
    else if cell.ra != 30 {
        api.map[CellApi::get_index(api.x, api.y)].ra -= 1;

        for i in api.get_adjacent() {
            let cell = api.get(i.0, i.1);
            if cell.species == Species::Wood && cell.ra == 30 {
                if rng.gen_bool(1.0 / 60.0) {
                    api.map[CellApi::get_index(api.x + i.0, api.y + i.1)].ra -= 1;
                }
            }
        }        
    }
}