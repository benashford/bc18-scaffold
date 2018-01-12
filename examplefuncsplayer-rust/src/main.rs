extern crate battlecode_engine as bc;
extern crate failure;
extern crate rand;

use std::env;
use std::collections::HashMap;

use bc::controller::*;
use bc::world::*;
use bc::unit::*;
use bc::map::*;
use bc::location::*;

use Location::*;
use Team::*;
use Direction::*;
use UnitType::*;

use failure::Error;

use rand::{Rng, SeedableRng, ChaChaRng};

fn examplefuncsplayer(gc: &mut GameController) -> Result<(), Error> {
    let mut rng = ChaChaRng::from_seed(&[2284860895, 1790736221, 1190208258, 3279695007, 2888369390, 2233370644, 3161697024, 2177838068]);
    let alld = Direction::all();
    gc.queue_research(Rocket);
    gc.queue_research(Worker);
    gc.queue_research(Knight);

    'unit: for unit in gc.my_units() {
        if unit.unit_type() == Factory {
            let d = *rng.choose(&alld[..]).unwrap();

            let garrison = unit.structure_garrison()?;
            if garrison.len() > 0 {
                if gc.can_unload(unit.id(), d) {
                    println!("unloaded a knight!");
                    gc.unload(unit.id(), d)?;
                }
            } else if gc.can_produce_robot(unit.id(), Knight) {
                gc.produce_robot(unit.id(), Knight)?;
                println!("produced a knight!");
            }
        }

        if let OnMap(loc) = unit.location() {
            assert_eq!(gc.sense_unit_at_location_opt(loc)?.unwrap().id(), unit.id());
            let sense_range = match unit.unit_type() {
                Worker => 2,
                Knight => 50,
                _ => 0
            };
            let nearby = gc.sense_nearby_units(loc, sense_range);
            for other in nearby {
                if unit.unit_type() == Knight && other.team() != unit.team() {
                    if gc.is_attack_ready(unit.id()) && gc.can_attack(unit.id(), other.id()) {
                        gc.attack(unit.id(), other.id());
                        println!("attack {} {}", unit.id(), other.id());
                    } else if gc.is_move_ready(unit.id()) {
                        let dir = unit.location().map_location()?
                            .direction_to(other.location().map_location()?)?;
                        
                        if gc.can_move(unit.id(), dir) {
                            gc.move_robot(unit.id(), dir);
                            println!("honing {} {:?}", unit.id(), dir);
                        }

                    }
                }
                if unit.unit_type() == UnitType::Worker && gc.can_build(unit.id(), other.id()) {
                    gc.build(unit.id(), other.id())?;
                    println!("building {} {}", unit.id(), other.id());
                }
            }
        }
        for _ in 0..4 {
            let dir = *rng.choose(&alld[..]).unwrap();
            if rng.gen::<u8>() < 128 && gc.karbonite() > UnitType::Factory.blueprint_cost()? &&
                gc.can_blueprint(unit.id(), UnitType::Factory, dir) &&
                unit.unit_type() == Worker {
                println!("blueprinting (i am {:?}) {:?} {:?}", unit.unit_type(), unit.location(), dir);
                gc.blueprint(unit.id(), UnitType::Factory, dir)?;
                break;
            } else if gc.is_move_ready(unit.id()) && gc.can_move(unit.id(), dir) {
                println!("moving {:?} {:?}", unit.location(), dir);
                gc.move_robot(unit.id(), dir)?;
                break;
            }
        }
    }
    Ok(())
}

fn main() {
    let mut gc = GameController::new_player_env().expect("GameController");
    while true {
        examplefuncsplayer(&mut gc).expect("Error running example");
        gc.next_turn().expect("Next turn");
    }
}
