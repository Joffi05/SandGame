use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::Duration;

mod cell_api;
use cell_api::*;


fn reset_canvas(canvas: &mut WindowCanvas, color: Color) {
    canvas.set_draw_color(color);
    canvas.clear();
    canvas.present();
}

fn draw_map(canvas: &mut WindowCanvas, map: &[[Cell; MAP_HEIGHT]; MAP_WIDTH]) {
    canvas.set_draw_color(Color::RGB(1, 1, 1));
    canvas.draw_rect(sdl2::rect::Rect::new(100 - 1, 100 - 1, (MAP_HEIGHT * CELL_SIZE) as u32 + 2, (MAP_WIDTH * CELL_SIZE) as u32 + 2)).expect("didnt correctly draw rect");
    for (x, x_item) in map.iter().enumerate() {
        for (y, y_item) in x_item.iter().enumerate() {
            match &y_item.species {
                Species::Empty => canvas.set_draw_color(Color::RGB(184, 168, 144)),
                Species::Wall => canvas.set_draw_color(Color::RGB(180, 176, 168)),
                Species::Sand => canvas.set_draw_color(Color::RGB(200, 208, 21)),
                Species::Water => canvas.set_draw_color(Color::RGB(55, 47, 234)),
                _ => {},
            }
            canvas.fill_rect(sdl2::rect::Rect::new((x * CELL_SIZE) as i32 + 100, (y * CELL_SIZE) as i32 + 100, CELL_SIZE as u32, CELL_SIZE as u32)).expect("Couldnt draw cell");
        }
    }
    canvas.present();
}

fn init_empty_map() -> [[Cell; MAP_HEIGHT]; MAP_WIDTH] {
    [[Cell 
    {
        species: Species::Empty,
        ra: 0,
        rb: 0,
        clock: 0,
    }; MAP_HEIGHT]; MAP_WIDTH]
}

fn update_map(api: &mut CellApi) {
    for x in 0..api.map.len() {
        for y in 0..api.map[x].len() {
            api.x = x;
            api.y = y;

            println!("{:?}", api.map[x][y]);

            match api.map[x][y].species {
                Species::Empty => {},
                Species::Wall => {},
                Species::Sand => api.update_sand(),
                Species::Water => {},
                _ => {},
            }
        }
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("SDL Rust Demo", 1400, 820)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");
        
    reset_canvas(&mut canvas, Color::RGB(184, 168, 144));

    rand_dir();

    let mut api = CellApi {
        map: init_empty_map(),
        x: 0,
        y: 0,
    };

    api.map[5][5] = Cell {
        species: Species::Sand,
        ra: 0,
        rb: 0,
        clock: 0,
    };

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        //handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        //Update

        update_map(&mut api);

        //Render

        draw_map(&mut canvas, &api.map);

        //Time management!
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

