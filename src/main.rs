mod config_manager;
mod exit_manager;

use rand::{Rng, rngs::StdRng, SeedableRng};
use std::thread;

static mut RUN: bool = true;

fn main() {
    let (frame_size, frame_delay, spawn_multiplier, filled_tile, empty_tile, starting_seed, use_seed) = config_manager::load_config();

    // use seed if configured to
    let mut rng = if use_seed == true {
        StdRng::seed_from_u64(starting_seed)
    } else {
        StdRng::from_entropy()
    };

    let mut main_layer = vec![vec![0; frame_size]; frame_size];
    let mut possible_layer = vec![vec![0; frame_size]; frame_size];
    let mut frame_count = 0;

    for _i in 0..frame_size*spawn_multiplier {
        let init_pos_x = rng.gen_range(1..frame_size-1);
        let init_pos_y = rng.gen_range(1..frame_size-1);
        main_layer[init_pos_x][init_pos_y] = 1;
    }

    let keybind_thread = thread::spawn(move || {
        exit_manager::exit_keybind();
    });

    while unsafe{ RUN } == true {
        for x in 1..frame_size-1 {
            for y in 1..frame_size-1 {
                let mut _neighbours = 0;

                _neighbours = main_layer[x-1][y-1] + main_layer[x-1][y] + main_layer[x-1][y+1] + main_layer[x][y-1] + main_layer[x][y+1] + main_layer[x+1][y-1] + main_layer[x+1][y] + main_layer[x+1][y+1];

                if main_layer[x][y] == 1 {
                    // Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
                    if _neighbours < 2 {
                        possible_layer[x][y] = 2;
                    }
                    // Any live cell with two or three live neighbours lives on to the next generation.
                    if _neighbours == 2 || _neighbours == 3 {
                        possible_layer[x][y] = 1;
                    }
                    // Any live cell with more than three live neighbours dies, as if by overpopulation.
                    if _neighbours > 3 {
                        possible_layer[x][y] = 2;
                    }
                }
                if main_layer[x][y] == 0 {
                    // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
                    if _neighbours == 3 {
                        possible_layer[x][y] = 1;
                    }
                }
                drop(_neighbours);
            }
        }
        
        for x in 1..frame_size-1 {
            for y in 1..frame_size-1 {
                if possible_layer[x][y] == 1 {
                    main_layer[x][y] = 1;
                    possible_layer[x][y] = 0;
                }
                if possible_layer[x][y] == 2 {
                    main_layer[x][y] = 0;
                    possible_layer[x][y] = 0;
                }
            }
        }
        
        // clear previous frame
        print!("{}[2J", 27 as char);
        
        for x in 1..frame_size-1 {
            for y in 1..frame_size-1 {
                print!("{}", main_layer[x][y].to_string().replace("0", &empty_tile.to_string()[..]).replace("1", &filled_tile.to_string()[..]));
            }
            println!("");
        }

        thread::sleep(std::time::Duration::from_millis(frame_delay));
        frame_count += 1;
    }

    println!("Quit simulation after {} frames while targeting at {} fps", frame_count, (1000/frame_delay));
    keybind_thread.join().unwrap();
}
