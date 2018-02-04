extern crate battlecode_engine as bc;
extern crate failure;
extern crate fnv;
extern crate rand;
extern crate time;

mod map;
mod order;
mod turn;

use bc::controller::GameController;
use bc::error::GameError;
use bc::location::{Direction, Location};
use bc::unit::{Unit, UnitType};

use failure::Error;

use map::DIRECTIONS;
use turn::{KnownKarbonite, Turn};

fn harvest_karbonite(
    gc: &mut GameController,
    karbonite: &mut KnownKarbonite,
    worker: &Unit,
    direction: Direction,
) -> Result<bool, GameError> {
    let location = match worker.location() {
        Location::OnMap(location) => location,
        _ => panic!("Only on-map units should call this function"),
    };
    let target_location = location.add(direction);
    let known_karbonite = karbonite.get(target_location.x, target_location.y);
    if known_karbonite > 0 {
        let worker_id = worker.id();
        let actual_karbonite = gc.karbonite_at(target_location)?;
        karbonite.set(location.x, location.y, actual_karbonite);
        if actual_karbonite > 0 && gc.can_harvest(worker_id, direction) {
            gc.harvest(worker_id, direction)?;
            return Ok(true);
        }
    }
    Ok(false)
}

fn harvest_nearest_karbonite(
    gc: &mut GameController,
    karbonite: &mut KnownKarbonite,
    worker: &Unit,
) -> Result<bool, GameError> {
    if harvest_karbonite(gc, karbonite, worker, Direction::Center)? {
        return Ok(true);
    }
    for dir in DIRECTIONS {
        if harvest_karbonite(gc, karbonite, worker, *dir)? {
            return Ok(true);
        }
    }
    Ok(false)
}

fn do_workers(gc: &mut GameController, turn: &mut Turn) -> Result<(), Error> {
    let num_workers = turn.my_units.workers.len();
    for worker in turn.my_units.workers.iter() {
        let worker_id = worker.id();
        let rand_direction = **rand::seq::sample_iter(&mut turn.rng, DIRECTIONS, 1)
            .unwrap()
            .get(0)
            .unwrap();
        if num_workers < 6 && gc.can_replicate(worker_id, rand_direction) {
            gc.replicate(worker_id, rand_direction)?;
            continue;
        }
        let location = match worker.location() {
            Location::OnMap(location) => location,
            _ => continue, // Probably in-space, ignore it
        };
        if harvest_nearest_karbonite(gc, &mut turn.known_karbonite, worker)? {
            continue;
        }
        if gc.is_move_ready(worker_id) {
            if let Some(direction) = turn.known_karbonite
                .gravity_map
                .get(location.x, location.y)
                .direction
            {
                if gc.can_move(worker_id, direction) {
                    gc.move_robot(worker_id, direction)?;
                }
            }
        }
    }
    Ok(())
}

fn do_turn(gc: &mut GameController, turn: &mut Turn) -> Result<(), Error> {
    do_workers(gc, turn)?;

    Ok(())
}

fn main() {
    println!("Starting Benone");
    let mut gc = GameController::new_player_env().expect("GameController");
    let mut turn_start = time::precise_time_ns();
    let mut turn = Turn::new(&gc);

    // TODO - more refined research
    gc.queue_research(UnitType::Worker);
    gc.queue_research(UnitType::Knight);
    gc.queue_research(UnitType::Rocket);
    gc.queue_research(UnitType::Factory);
    gc.queue_research(UnitType::Ranger);
    gc.queue_research(UnitType::Mage);
    gc.queue_research(UnitType::Healer);

    loop {
        let round = gc.round();
        let karbonite = gc.karbonite();
        println!("START OF TURN {}, karbonite: {}", round, karbonite);
        turn.update(&gc);
        do_turn(&mut gc, &mut turn).expect("Turn failed");
        let turn_end = time::precise_time_ns();
        println!(
            "END OF TURN {}, took: {:.3}ms",
            round,
            (turn_end - turn_start) as f64 / 1000000.0f64
        );
        gc.next_turn().expect("Cannot start next turn");
        turn_start = time::precise_time_ns();
    }

    println!("Finishing, goodbye");
}
