extern crate battlecode_engine as bc;
extern crate time;

mod turn;

use bc::controller::GameController;

use turn::Turn;

fn do_turn(gc: &mut GameController, turn: &Turn) {}

fn main() {
    println!("Starting Benone");
    let mut gc = GameController::new_player_env().expect("GameController");
    let mut turn_start = time::precise_time_ns();
    let mut turn = Turn::new(&gc);

    loop {
        turn.update(&gc);
        do_turn(&mut gc, &turn);
        let turn_end = time::precise_time_ns();
        println!(
            "END OF TURN {}, took: {:.3}ns",
            gc.round(),
            turn_end - turn_start
        );
        gc.next_turn();
        turn_start = time::precise_time_ns();
    }

    println!("Finishing, goodbye");
}
