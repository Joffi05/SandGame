use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use sdl2::mouse::MouseState;
use std::time::Duration;
use rand::{Rng, thread_rng};

mod cell_api;
mod update_funcs;
mod utils;
use cell_api::*;
use update_funcs::*;
use utils::*;

fn is_colliding(x1: i32, y1: i32, width1: i32, heigth1: i32, x2: i32, y2: i32, width2: i32, heigth2: i32) -> bool {
    x1 < x2 + width2 &&
    x1 + width1 > x2 &&
    y1 < y2 + heigth2 &&
    y1 + heigth1 > y2
}

fn reset_canvas(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

fn draw_map(canvas: &mut WindowCanvas, api: &CellApi) {
    canvas.set_draw_color(Color::RGB(1, 1, 1));
    canvas.draw_rect(sdl2::rect::Rect::new(100 - 1, 100 - 1, (MAP_WIDTH * CELL_SIZE) as u32 + 2, (MAP_HEIGHT * CELL_SIZE) as u32 + 2)).expect("didnt correctly draw rect");
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let cell = api.map[CellApi::get_index(x, y)];
            match cell.species {
                Species::Empty => canvas.set_draw_color(Color::RGB(184, 168, 144)),
                Species::Wall => canvas.set_draw_color(Color::RGB(180, 176, 168)),
                Species::Sand => canvas.set_draw_color(Color::RGB(200, 208, 21)),
                Species::Water => canvas.set_draw_color(Color::RGB(55, 47, 234)),
                Species::Wood => { if cell.ra == 30 {canvas.set_draw_color(Color::RGB(101, 67, 33))} else {canvas.set_draw_color(Color::RGB(255, 100, 50))}},
                Species::Fire => canvas.set_draw_color(Color::RGB(255 - cell.ra * 10, 100 - cell.ra * 6, 50 - cell.ra * 3)),
                Species::Oil => canvas.set_draw_color(Color::RGB(5, 5, 5)),
                Species::Steam => canvas.set_draw_color(Color::RGB(150, 150, 234)),
            }
            canvas.fill_rect(sdl2::rect::Rect::new((x * CELL_SIZE) as i32 + 100, (y * CELL_SIZE) as i32 + 100, CELL_SIZE as u32, CELL_SIZE as u32)).expect("Couldnt draw cell");
        }
    }
    canvas.present();
}

fn update_map(api: &mut CellApi) {

    api.clock = !api.clock;

    for y in 0..MAP_HEIGHT {
        for x in (0..MAP_WIDTH).rev() {
            api.x = x;
            api.y = y;

            if api.map[CellApi::get_index(x, y)].clock == api.clock {
                api.map[CellApi::get_index(x, y)].clock = !api.map[CellApi::get_index(x, y)].clock;

                match api.map[CellApi::get_index(x, y)].species {
                    Species::Empty => {},
                    Species::Wall => {},
                    Species::Sand => update_sand(api),
                    Species::Water => update_water(api),
                    Species::Wood => update_wood(api),
                    Species::Fire => update_fire(api),
                    Species::Oil => update_oil(api),
                    Species::Steam => update_steam(api),
                }
            }   
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("SDL Rust Demo", 1000, 800)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
    
    let mut api = CellApi {
        map: vec![EMPTY_CELL; (MAP_WIDTH * MAP_HEIGHT) as usize],
        clock: true,
        x: 0,
        y: 0,
    };

    reset_canvas(&mut canvas, Color::RGB(184, 168, 144));

    let mut selected_species: Species = Species::Sand;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        //handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    selected_species = selected_species.next();
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    selected_species = selected_species.last();
                },
                Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                    for y in 0..MAP_HEIGHT {
                        for x in 0..MAP_WIDTH {
                            api.map[CellApi::get_index(x, y)] = EMPTY_CELL;
                        }
                    }
                },
                _ => {}
            }
        }

        let mouse = MouseState::new(&event_pump);

        if mouse.left() {
            let mouse_x = (mouse.x() - 100 - (mouse.x() % CELL_SIZE)) / CELL_SIZE;
            let mouse_y = (mouse.y() - 100 - (mouse.y() % CELL_SIZE)) / CELL_SIZE;
            if mouse.x() > 100 && 
                mouse.x() < 100 + MAP_WIDTH * CELL_SIZE &&
                mouse.y() > 100 &&
                mouse.y() < 100 + MAP_HEIGHT * CELL_SIZE {

                    //warum durch 2 ¯\_(ツ)_/¯ 
                    api.spawn(mouse_x / 2, mouse_y / 2, 5, selected_species);
                }
        }

        //Update

        update_map(&mut api);

        //Render

        draw_map(&mut canvas, &api);

        //Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000u32 / 60));
    }

    Ok(())
}