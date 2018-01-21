use std::collections::VecDeque;

use fnv::FnvHashSet;

use bc::location::{Direction, MapLocation};
use bc::map::PlanetMap;

#[derive(Debug, Default)]
pub(crate) struct GravityMapCell {
    pub(crate) direction: Option<Direction>,
    distance: usize,
}

#[derive(Debug)]
pub(crate) struct GravityMap {
    directions: Vec<Direction>,
    planet: PlanetMap,
    map: Vec<Vec<GravityMapCell>>,
}

impl GravityMap {
    pub(crate) fn new(planet: &PlanetMap) -> GravityMap {
        GravityMap {
            directions: Direction::all(),
            planet: planet.clone(),
            map: (0..planet.height)
                .map(|_| (0..planet.width).map(|_| Default::default()).collect())
                .collect(),
        }
    }

    pub(crate) fn get(&self, x: i32, y: i32) -> &GravityMapCell {
        &self.map[y as usize][x as usize]
    }

    fn initialize(&mut self) {
        for y in 0..self.planet.height {
            for x in 0..self.planet.width {
                let cell = &mut self.map[y][x];
                cell.direction = None;
                cell.distance = 0;
            }
        }
    }

    pub(crate) fn update(
        &mut self,
        locations: Vec<(i32, i32)>,
        obstacles: &FnvHashSet<MapLocation>,
    ) {
        self.initialize();
        let mut visit_queue = VecDeque::with_capacity(locations.len());
        for (x, y) in locations {
            self.map[y as usize][x as usize].direction = Some(Direction::Center);
            visit_queue.push_back((x, y));
        }

        let height = self.planet.height as i32;
        let width = self.planet.width as i32;

        while !visit_queue.is_empty() {
            let (x, y) = visit_queue.pop_front().expect("Queue is empty");
            let ndist = self.map[y as usize][x as usize].distance + 1;
            for direction in self.directions.iter() {
                let nx = x + direction.dx();
                let ny = y + direction.dy();

                if ny < 0 || ny >= height || nx < 0 || nx >= width {
                    continue;
                }

                let cell = &mut self.map[ny as usize][nx as usize];
                let map_location = MapLocation::new(self.planet.planet, nx, ny);
                if cell.direction.is_none()
                    && self.planet
                        .is_passable_terrain_at(map_location)
                        .expect("Not a boolean result")
                    && !obstacles.contains(&map_location)
                {
                    visit_queue.push_back((nx, ny));
                    cell.direction = Some(direction.opposite());
                    cell.distance = ndist;
                }
            }
        }
    }
}
