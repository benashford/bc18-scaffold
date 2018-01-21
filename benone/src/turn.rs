use fnv::FnvHashMap;

use rand;

use bc::controller::GameController;
use bc::location::Direction;
use bc::map::PlanetMap;
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
            UnitType::Rocket => self.factories.push(unit),
        }
    }
}

#[derive(Debug)]
pub(crate) struct KnownKarbonite {
    // All the locations that are known to have karbonite
    karbonite_locations: FnvHashMap<(i32, i32), u32>,
    // whether the gravity map needs updating on the next turn
    update_map: bool,
    pub(crate) gravity_map: GravityMap,
}

impl KnownKarbonite {
    fn new(planet: &PlanetMap) -> KnownKarbonite {
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

        let map = GravityMap::new(planet);

        KnownKarbonite {
            karbonite_locations: locs,
            update_map: true,
            gravity_map: map,
        }
    }

    fn update(&mut self) {
        if !self.update_map {
            return;
        }
        println!(" updating map");
        self.gravity_map
            .update(self.karbonite_locations.keys().map(|x| *x).collect());
        self.update_map = false;
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
            self.update_map = true;
            self.karbonite_locations.remove(&(x, y));
        }
    }
}

#[derive(Debug)]
pub(crate) struct Turn {
    pub(crate) rng: rand::ThreadRng,

    pub(crate) directions: Vec<Direction>,

    pub(crate) known_karbonite: KnownKarbonite,

    pub(crate) my_units: KnownUnits,
    pub(crate) enemy_units: KnownUnits,
}

impl Turn {
    pub(crate) fn new(gc: &GameController) -> Self {
        let planet = gc.planet();
        let starting_map = gc.starting_map(planet);

        let mut turn = Turn {
            rng: rand::thread_rng(),
            directions: Direction::all(),
            known_karbonite: KnownKarbonite::new(&starting_map),
            my_units: Default::default(),
            enemy_units: Default::default(),
        };
        turn.update(gc);
        turn
    }

    pub(crate) fn update(&mut self, gc: &GameController) {
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

        self.known_karbonite.update();
    }
}
