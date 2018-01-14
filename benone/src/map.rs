use std::collections::VecDeque;

use bc::location::{Direction, MapLocation};
use bc::map::PlanetMap;

#[derive(Debug, Default)]
pub(crate) struct GravityMapCell {
    pub(crate) direction: Option<Direction>,
    distance: usize,
}

pub(crate) type GravityMap = Vec<Vec<GravityMapCell>>;

pub(crate) fn gravity_map(planet: &PlanetMap, locations: Vec<(i32, i32)>) -> GravityMap {
    let directions = Direction::all();

    let width = planet.width as i32;
    let height = planet.height as i32;

    let mut gravity_map: GravityMap = (0..width)
        .map(|_| (0..height).map(|_| Default::default()).collect())
        .collect();

    let mut visit_queue = VecDeque::with_capacity(locations.len());
    for (y, x) in locations {
        gravity_map[y as usize][x as usize].direction = Some(Direction::Center);
        visit_queue.push_back((y, x));
    }

    while !visit_queue.is_empty() {
        let (y, x) = visit_queue.pop_front().expect("Queue is empty");
        let ndist = gravity_map[y as usize][x as usize].distance + 1;
        for direction in directions.iter() {
            let ny = y + direction.dy();
            let nx = x + direction.dx();
            if ny < 0 || ny >= height || nx < 0 || nx >= width {
                continue;
            }
            let cell = &mut gravity_map[ny as usize][nx as usize];
            if cell.direction.is_none()
                && planet
                    .is_passable_terrain_at(MapLocation::new(planet.planet, nx, ny))
                    .expect("Not a boolean result")
            {
                visit_queue.push_back((ny, nx));
                cell.direction = Some(direction.opposite());
                cell.distance = ndist;
            }
        }
    }

    gravity_map
}
