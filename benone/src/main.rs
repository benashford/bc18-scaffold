extern crate battlecode_engine as bc;
extern crate failure;
extern crate fnv;
extern crate rand;
extern crate time;

mod turn;

use bc::controller::GameController;
use bc::location::{Direction, Location};

use failure::Error;

use turn::Turn;

fn do_workers(gc: &mut GameController, turn: &mut Turn) -> Result<(), Error> {
    let num_workers = turn.my_units.workers.len();
    for &worker_id in turn.my_units.workers.iter() {
        let rand_direction = **rand::seq::sample_iter(&mut turn.rng, &turn.directions, 1)
            .unwrap()
            .get(0)
            .unwrap();
        if num_workers < 6 && gc.can_replicate(worker_id, rand_direction) {
            gc.replicate(worker_id, rand_direction)?;
            continue;
        }
        let location = match gc.unit_ref(worker_id)?.location() {
            Location::OnMap(location) => location,
            _ => continue,
        };
        // TODO - replace with "find nearest karbonite"
        let known_karbonite = turn.known_karbonite.get(location.y, location.x);
        if known_karbonite > 0 {
            let actual_karbonite = gc.karbonite_at(location)?;
            turn.known_karbonite
                .set(location.y, location.x, actual_karbonite);
            if actual_karbonite > 0 && gc.can_harvest(worker_id, Direction::Center) {
                gc.harvest(worker_id, Direction::Center)?;
                continue;
            }
        } else {
            if gc.is_move_ready(worker_id) {
                if gc.can_move(worker_id, rand_direction) {
                    gc.move_robot(worker_id, rand_direction)?;
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
