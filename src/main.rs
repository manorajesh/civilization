use macroquad::prelude::*;
mod map;

const TILE_WIDTH: f32 = 5.0;
const TILE_HEIGHT: f32 = 2.5;
const INITIAL_ZOOM: f32 = 0.01;
const MIN_ZOOM: f32 = 0.0001;
const MAX_ZOOM: f32 = 100.0;

#[macroquad::main("civilization")]
async fn main() {
    let map = map::Map::new(100, 100, None);
    let mut camera = Camera2D {
        zoom: vec2(INITIAL_ZOOM, INITIAL_ZOOM),
        target: vec2(
            ((map.width as f32) * TILE_WIDTH) / 2.0,
            ((map.height as f32) * TILE_HEIGHT) / 2.0
        ),
        ..Default::default()
    };
    let mut last_mouse_position: Option<Vec2> = None;

    loop {
        clear_background(BLACK);

        set_camera(&camera);

        // if is_mouse_button_down(MouseButton::Left) {
        //     let mouse_position = mouse_position();
        //     let x = (mouse_position.0 / TILE_WIDTH) as usize;
        //     let y = (mouse_position.1 / TILE_HEIGHT) as usize;

        //     let cell = &map.cells[y][x];
        //     println!("Cell at ({}, {}): {:?}", x, y, cell);
        // }

        if is_mouse_button_down(MouseButton::Middle) {
            if let Some(last_pos) = last_mouse_position {
                let mouse_pos = vec2(mouse_position().0, mouse_position().1);
                let delta = mouse_pos - last_pos;
                camera.target -= delta / camera.zoom / 200.0;
                last_mouse_position = Some(mouse_pos);
            } else {
                last_mouse_position = Some(vec2(mouse_position().0, mouse_position().1));
            }
        } else {
            last_mouse_position = None;
        }

        if mouse_wheel().1 != 0.0 {
            camera.zoom += (vec2(mouse_wheel().1, mouse_wheel().1) * camera.zoom) / 1000.0;
            camera.zoom = camera.zoom.clamp(vec2(MIN_ZOOM, MIN_ZOOM), vec2(MAX_ZOOM, MAX_ZOOM));
        }

        for y in 0..map.height {
            for x in 0..map.width {
                let cell = &map.cells[y][x];
                let color = match cell.cell_types.last().unwrap() {
                    map::CellType::Resource(map::ResourceType::Water) => BLUE,
                    map::CellType::Resource(map::ResourceType::Field) => GREEN,
                    map::CellType::Resource(map::ResourceType::Tree) => DARKGREEN,
                    map::CellType::Resource(map::ResourceType::Stone) => GRAY,
                    _ => RED,
                };

                // Calculate the isometric position for the tile
                let iso_x = (((x as f32) - (y as f32)) * TILE_WIDTH) / 2.0;
                let iso_y = (((x as f32) + (y as f32)) * TILE_HEIGHT) / 2.0;

                // Draw the isometric tile
                draw_poly(iso_x, iso_y, 4, TILE_WIDTH / 2.0, 0.0, color);
            }
        }

        // draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 30.0, WHITE);

        set_default_camera();
        next_frame().await;
    }
}
