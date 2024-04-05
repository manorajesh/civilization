mod map;

fn main() {
    let map = map::Map::new(100, 100, None);

    for i in 0..map.height {
        for j in 0..map.width {
            match &map.cells[i * map.width + j].cell_types[0] {
                map::CellType::Empty => print!("."), // White background for empty
                map::CellType::Resource(resource_type) => {
                    match resource_type {
                        map::ResourceType::Water => print!("\x1B[44m~\x1B[0m"), // Blue background for water
                        map::ResourceType::Tree => print!("\x1B[42mT\x1B[0m"), // Green background for trees
                        map::ResourceType::Stone => print!("\x1B[40mS\x1B[0m"), // Black background for stone
                        map::ResourceType::Field => print!("\x1B[43mF\x1B[0m"), // Yellow background for fields
                    }
                }
                _ => print!("\x1B[41m?\x1B[0m"), // Red background for unknown
            }
        }
        println!();
    }
}
