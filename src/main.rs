use macroquad::prelude::*;
mod map;

#[macroquad::main("BasicShapes")]
async fn main() {
    let map = map::Map::new(100, 100, None);

    loop {
        clear_background(BLACK);

        for y in 0..map.height {
            for x in 0..map.width {
                let cell = &map.cells[y][x];
                let color = match cell.cell_types.last().unwrap() {
                    map::CellType::Resource(map::ResourceType::Water) => BLUE,
                    map::CellType::Resource(map::ResourceType::Field) => GREEN,
                    map::CellType::Resource(map::ResourceType::Tree) => BROWN,
                    map::CellType::Resource(map::ResourceType::Stone) => GRAY,
                    _ => RED,
                };

                draw_rectangle((x as f32) * 50.0, (y as f32) * 50.0, 50.0, 50.0, color);
            }
        }

        next_frame().await;
    }
}
