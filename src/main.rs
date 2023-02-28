mod config_manager;
mod exit_manager;

use rand::{Rng, rngs::StdRng, SeedableRng};
use std::io::Write;
use std::thread;

static mut RUN: bool = true;

fn do_nothing() {
    // do nothing... shocker
}

fn main() {
    let stdout = std::io::stdout();

    let (frame_size, frame_delay, spawn_multiplier, filled_tile, empty_tile, starting_seed, use_seed, interleaved_frames, live_rule, grow_rule) = config_manager::load_config();
    let filled_tile = &filled_tile.to_string()[..];
    let empty_tile = &empty_tile.to_string()[..];

    // use seed if configured to
    let mut rng = if use_seed == true {
        StdRng::seed_from_u64(starting_seed.try_into().unwrap())
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

    print!("{}[2J", 27 as char);

    while unsafe { RUN } {
        for x in 1..frame_size-1 {
            for y in 1..frame_size-1 {
                let mut _neighbours = main_layer[x-1][y-1] + main_layer[x-1][y] + main_layer[x-1][y+1] + main_layer[x][y-1] + main_layer[x][y+1] + main_layer[x+1][y-1] + main_layer[x+1][y] + main_layer[x+1][y+1];

                match main_layer[x][y] {
                    1 => {
                        if live_rule.contains(_neighbours as u8 as char) {
                            possible_layer[x][y] = 1;
                        } else {
                            possible_layer[x][y] = 2;
                        }
                    },
                    0 => {
                        if grow_rule.contains(_neighbours as u8 as char) {
                            possible_layer[x][y] = 1;
                        }
                    },
                    _ => do_nothing()
                }
                drop(_neighbours);
            }
        }
        
        for x in 1..frame_size-1 {
            for y in 1..frame_size-1 {
                match possible_layer[x][y] {
                    1 => {
                        main_layer[x][y] = 1;
                        possible_layer[x][y] = 0;
                    },
                    2 => {
                        main_layer[x][y] = 0;
                        possible_layer[x][y] = 0;
                    },
                    _ => do_nothing()
                }
            }
        }
        
        if interleaved_frames == false || frame_count % 2 == 0 {
            // clear previous frame
            print!("{esc}[1;1H", esc = 27 as char);
            
            let mut lock = stdout.lock();
            for x in 1..frame_size-1 {
                for y in 1..frame_size-1 {
                    write!(lock, "{}", main_layer[x][y].to_string().replace("0", empty_tile).replace("1", filled_tile));
                }
                println!("");
            }
            drop(lock);

            thread::sleep(std::time::Duration::from_millis(frame_delay));
        }
        frame_count += 1;
    }

    if frame_delay <= 0 {
        println!("Quit simulation after {frame_count} frames\nTargeted fps was ∞\nInterleaved frames were {interleaved_frames_status}", frame_count = frame_count, interleaved_frames_status = interleaved_frames.to_string().replace("true", "on").replace("false", "off"));
    } else {
        println!("Quit simulation after {frame_count} frames\nTargeted fps was {fps}\nInterleaved frames were {interleaved_frames_status}", frame_count = frame_count, fps = (1000/frame_delay), interleaved_frames_status = interleaved_frames.to_string().replace("true", "on").replace("false", "off"));
    }

    thread::sleep(std::time::Duration::from_millis(1000));
    keybind_thread.join().unwrap();
    std::process::exit(0);
}