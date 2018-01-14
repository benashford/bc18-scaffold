use fnv::FnvHashMap;

use rand;

use bc::controller::GameController;
use bc::location::Direction;
use bc::map::PlanetMap;
use bc::unit::{Unit, UnitID, UnitType};
use bc::world::Team;

#[derive(Debug, Default)]
pub(crate) struct KnownUnits {
    pub(crate) workers: Vec<UnitID>,
    pub(crate) knights: Vec<UnitID>,
    pub(crate) rangers: Vec<UnitID>,
    pub(crate) mages: Vec<UnitID>,
    pub(crate) healers: Vec<UnitID>,
    pub(crate) factories: Vec<UnitID>,
    pub(crate) rockets: Vec<UnitID>,
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

    fn add(&mut self, unit: &Unit) {
        let unit_id = unit.id();
        match unit.unit_type() {
            UnitType::Worker => self.workers.push(unit_id),
            UnitType::Knight => self.knights.push(unit_id),
            UnitType::Ranger => self.rangers.push(unit_id),
            UnitType::Mage => self.mages.push(unit_id),
            UnitType::Healer => self.healers.push(unit_id),
            UnitType::Factory => self.factories.push(unit_id),
            UnitType::Rocket => self.factories.push(unit_id),
        }
    }
}

#[derive(Debug)]
pub(crate) struct KnownKarbonite {
    // All the locations that are known to have karbonite
    karbonite_locations: FnvHashMap<(i32, i32), u32>,
    // whether the gravity map needs updating on the next turn
    update_map: bool,
}

impl KnownKarbonite {
    fn new(planet: &PlanetMap) -> KnownKarbonite {
        let width = planet.width;
        let height = planet.height;
        let original_locs = &planet.initial_karbonite;

        let mut locs = FnvHashMap::default();

        for x in 0..width {
            for y in 0..height {
                let karbonite = original_locs[y][x];
                if karbonite > 0 {
                    locs.insert((y as i32, x as i32), karbonite);
                }
            }
        }

        KnownKarbonite {
            karbonite_locations: locs,
            update_map: true,
        }
    }

    pub(crate) fn get(&self, y: i32, x: i32) -> u32 {
        match self.karbonite_locations.get(&(y, x)) {
            Some(&amt) => amt,
            None => 0u32,
        }
    }

    pub(crate) fn set(&mut self, y: i32, x: i32, karbonite: u32) {
        if karbonite > 0 {
            self.karbonite_locations.insert((y, x), karbonite);
        } else {
            self.karbonite_locations.remove(&(y, x));
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

        for unit_ref in gc.units_ref() {
            if unit_ref.team() == my_team {
                self.my_units.add(unit_ref);
            } else {
                self.enemy_units.add(unit_ref);
            }
        }
    }
}
