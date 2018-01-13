use rand;

use bc::controller::GameController;
use bc::location::Direction;
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
pub(crate) struct Turn {
    pub(crate) rng: rand::ThreadRng,

    pub(crate) directions: Vec<Direction>,

    pub(crate) known_karbonite: Vec<Vec<u32>>,

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
            known_karbonite: starting_map.initial_karbonite.clone(),
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
