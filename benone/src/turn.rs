use std::iter::Iterator;

use fnv::{FnvHashMap, FnvHashSet};

use rand;

use bc::controller::GameController;
use bc::location::{Location, MapLocation};
use bc::map::{AsteroidPattern, PlanetMap};
use bc::unit::{Unit, UnitType};

use map::GravityMap;

#[derive(Debug, Default)]
pub(crate) struct KnownUnits {
    pub(crate) workers: Vec<Unit>,
    pub(crate) knights: Vec<Unit>,
    pub(crate) rangers: Vec<Unit>,
    pub(crate) mages: Vec<Unit>,
    pub(crate) healers: Vec<Unit>,
    pub(crate) factories: Vec<Unit>,
    pub(crate) rockets: Vec<Unit>,
}

// TODO - this may make more sense being separated into SelfInfo and EnemyInfo
impl KnownUnits {
    fn reset(&mut self) {
        self.workers.clear();
        self.knights.clear();
        self.rangers.clear();
        self.mages.clear();
        self.healers.clear();
        self.factories.clear();
        self.rockets.clear();
    }

    fn add(&mut self, unit: Unit) {
        match unit.unit_type() {
            UnitType::Worker => self.workers.push(unit),
            UnitType::Knight => self.knights.push(unit),
            UnitType::Ranger => self.rangers.push(unit),
            UnitType::Mage => self.mages.push(unit),
            UnitType::Healer => self.healers.push(unit),
            UnitType::Factory => self.factories.push(unit),
            UnitType::Rocket => self.rockets.push(unit),
        }
    }

    fn iter<'a>(&'a self) -> Box<Iterator<Item = &'a Unit> + 'a> {
        Box::new(
            self.workers
                .iter()
                .chain(self.knights.iter())
                .chain(self.rangers.iter())
                .chain(self.mages.iter())
                .chain(self.healers.iter())
                .chain(self.factories.iter())
                .chain(self.rockets.iter()),
        )
    }
}

#[derive(Debug)]
pub(crate) struct KnownKarbonite {
    // All the locations that are known to have karbonite
    karbonite_locations: FnvHashMap<(i32, i32), u32>,
    future_karbonite: Vec<(u32, i32, i32, u32)>, // round, x, y, amt
    pub(crate) gravity_map: GravityMap,
}

impl KnownKarbonite {
    fn new(planet: &PlanetMap, asteroids: &AsteroidPattern) -> KnownKarbonite {
        let width = planet.width;
        let height = planet.height;
        let original_locs = &planet.initial_karbonite;

        let mut locs = FnvHashMap::default();

        for y in 0..height {
            for x in 0..width {
                let karbonite = original_locs[y][x];
                if karbonite > 0 {
                    locs.insert((x as i32, y as i32), karbonite);
                }
            }
        }

        let asteroid_map = asteroids.asteroid_map();
        let mut future_karbonite: Vec<(u32, i32, i32, u32)> = asteroid_map
            .iter()
            .map(|(round, strike)| {
                let location = strike.location;
                (*round, location.x, location.y, strike.karbonite)
            })
            .collect();
        future_karbonite.sort_by(|&(x, _, _, _), &(y, _, _, _)| y.cmp(&x));

        let map = GravityMap::new(planet);

        KnownKarbonite {
            karbonite_locations: locs,
            future_karbonite: future_karbonite,
            gravity_map: map,
        }
    }

    fn update(&mut self, round_num: u32, obstacles: &FnvHashSet<MapLocation>) {
        let fut_karb_len = self.future_karbonite.len();
        if fut_karb_len > 0 {
            let &(round, _, _, _) = &self.future_karbonite[fut_karb_len - 1];
            if round <= round_num {
                let (_, x, y, amt) = self.future_karbonite
                    .pop()
                    .expect("Last vector element has gone missing");
                self.karbonite_locations.insert((x, y), amt);
            }
        }
        let known_locations = self.karbonite_locations.keys().map(|&(x, y)| (x, y, 0));
        let still_future_karb = self.future_karbonite
            .iter()
            .rev()
            .map(|&(round, x, y, _)| (x, y, round - round_num))
            .take_while(|&(_, _, rounds_until)| rounds_until < 100);
        self.gravity_map.update(
            known_locations.chain(still_future_karb).collect(),
            obstacles,
        );
    }

    pub(crate) fn get(&self, x: i32, y: i32) -> u32 {
        match self.karbonite_locations.get(&(x, y)) {
            Some(&amt) => amt,
            None => 0u32,
        }
    }

    pub(crate) fn set(&mut self, x: i32, y: i32, karbonite: u32) {
        if karbonite > 0 {
            self.karbonite_locations.insert((x, y), karbonite);
        } else {
            self.karbonite_locations.remove(&(x, y));
        }
    }
}

#[derive(Debug)]
pub(crate) struct Turn {
    pub(crate) rng: rand::ThreadRng,

    pub(crate) known_karbonite: KnownKarbonite,

    pub(crate) my_units: KnownUnits,
    pub(crate) enemy_units: KnownUnits,
}

impl Turn {
    pub(crate) fn new(gc: &GameController) -> Self {
        let planet = gc.planet();
        let starting_map = gc.starting_map(planet);
        let asteroid_pattern = gc.asteroid_pattern();

        let mut turn = Turn {
            rng: rand::thread_rng(),
            known_karbonite: KnownKarbonite::new(&starting_map, &asteroid_pattern),
            my_units: Default::default(),
            enemy_units: Default::default(),
        };
        turn.update(gc);
        turn
    }

    pub(crate) fn update(&mut self, gc: &GameController) {
        let round_num = gc.round();
        let my_team = gc.team();

        self.my_units.reset();
        self.enemy_units.reset();

        for unit in gc.units() {
            if unit.team() == my_team {
                self.my_units.add(unit);
            } else {
                self.enemy_units.add(unit);
            }
        }

        let obstacles = self.enemy_units
            .iter()
            .fold(FnvHashSet::default(), |mut h, u| {
                if let Location::OnMap(map_loc) = u.location() {
                    h.insert(map_loc);
                }
                h
            });

        self.known_karbonite.update(round_num, &obstacles);
    }
}
