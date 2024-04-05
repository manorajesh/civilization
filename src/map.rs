use noise::{ NoiseFn, Perlin, Seedable };
use rand::Rng;

// -----------------------------
// Type Definitions
// -----------------------------

#[derive(Debug, Default)]
pub enum ResourceType {
    #[default]
    Field,
    Water,
    Tree,
    Stone,
}

#[derive(Debug)]
pub enum InfrastructureType {
    Road,
    Farm,
    Mine,
    LumberMill,
    Quarry,
    House,
}

#[derive(Debug, Default)]
pub enum CellType {
    #[default]
    Empty,
    Resource(ResourceType),
    Infrastructure(InfrastructureType),
    Unit, // character
}

#[derive(Debug)]
pub struct MapCell {
    x: isize,
    y: isize,
    pub cell_types: Vec<CellType>,
}

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<MapCell>,
}

// -----------------------------
// Implementations
// -----------------------------

trait FromNoise {
    fn from_noise(noise: f64) -> Self;
}

impl FromNoise for ResourceType {
    fn from_noise(noise: f64) -> Self {
        if noise < 0.2 {
            ResourceType::Water
        } else if noise < 0.35 {
            ResourceType::Field
        } else if noise < 0.65 {
            ResourceType::Tree
        } else {
            ResourceType::Stone
        }
    }
}

impl FromNoise for CellType {
    fn from_noise(noise: f64) -> Self {
        CellType::Resource(ResourceType::from_noise(noise))
    }
}

impl Map {
    pub fn new(width: usize, height: usize, seed: Option<u32>) -> Self {
        let mut cells = Vec::with_capacity(width * height);
        let perlin = Perlin::new(seed.unwrap_or(rand::thread_rng().gen()));
        const PERLIN_NOISE_SCALE: f64 = 10.0;

        for y in 0..height {
            for x in 0..width {
                let value = perlin.get([
                    (x as f64) / PERLIN_NOISE_SCALE,
                    (y as f64) / PERLIN_NOISE_SCALE,
                    0.0,
                ]);

                let cell_type = CellType::from_noise(value);

                cells.push(MapCell {
                    x: x as isize,
                    y: y as isize,
                    cell_types: vec![cell_type],
                });
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get_cell(&self, x: isize, y: isize) -> Option<&MapCell> {
        self.cells.iter().find(|cell| cell.x == x && cell.y == y)
    }

    pub fn get_cell_mut(&mut self, x: isize, y: isize) -> Option<&mut MapCell> {
        self.cells.iter_mut().find(|cell| cell.x == x && cell.y == y)
    }
}
