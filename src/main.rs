mod keybind_manager;
mod config_manager;

use rand::{Rng, rngs::StdRng, SeedableRng};
use std::io::Write;
use std::thread;

static mut RUN: bool = true;
static mut PAUSE: bool = false;
static mut DEV: bool = false;

fn do_nothing() {
    // Does nothing... shocker
}

fn main() {
    let stdout = std::io::stdout();

    // Loads config
    let (frame_size, frame_delay, spawn_multiplier, filled_tile, empty_tile, starting_seed, use_seed, interleaved_frames, live_rule, grow_rule) = config_manager::load_config();
    let filled_tile = &filled_tile.to_string()[..];
    let empty_tile = &empty_tile.to_string()[..];

    let mut live_rule_lookup = vec![0; 10];
    let mut grow_rule_lookup = vec![0; 10];
    for i in 0..10 {
        if live_rule.contains(i.to_string().as_str()) {
            live_rule_lookup[i] = 1;
        }
        if grow_rule.contains(i.to_string().as_str()) {
            grow_rule_lookup[i] = 1;
        }
    }

    // Use seed if configured to
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
        keybind_manager::init_keybinds();
    });

    print!("{}[2J", 27 as char);

    while unsafe { RUN } {
        for x in 1..frame_size-1 {
            for y in 1..frame_size-1 {
                let neighbours = main_layer[x-1][y-1] + main_layer[x-1][y] + main_layer[x-1][y+1] + main_layer[x][y-1] + main_layer[x][y+1] + main_layer[x+1][y-1] + main_layer[x+1][y] + main_layer[x+1][y+1];

                match main_layer[x][y] {
                    1 => {
                        if live_rule_lookup[neighbours] == 1 {
                            possible_layer[x][y] = 1;
                        } else {
                            possible_layer[x][y] = 2;
                        }
                    },
                    0 => {
                        if grow_rule_lookup[neighbours as usize] == 1 {
                            possible_layer[x][y] = 1;
                        }
                    },
                    _ => do_nothing()
                }
                drop(neighbours);
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
            // Clears previous frame
            print!("{esc}[1;1H", esc = 27 as char);
            
            let mut lock = stdout.lock();
            for x in 1..frame_size-1 {
                for y in 1..frame_size-1 {
                    write!(lock, "{}", main_layer[x][y].to_string().replace("0", empty_tile).replace("1", filled_tile));
                }
                print!("{}\n", " ".repeat(20));
            }
            drop(lock);

            // Dev info
            if unsafe { DEV } == true {
                if frame_delay <= 0 {
                    println!("Frame {frame_count} while targeting ∞ fps {frame_size}x{frame_size} with {spawn_multiplier}x spawn multiplier with interleaved frames {interleaved_frames_status}", frame_count = frame_count, frame_size = frame_size, spawn_multiplier = spawn_multiplier, interleaved_frames_status = interleaved_frames.to_string().replace("true", "on").replace("false", "off"));
                } else {
                    println!("Frame {frame_count} while targeting {fps} fps {frame_size}x{frame_size} with {spawn_multiplier}x spawn multiplier with interleaved frames {interleaved_frames_status}", frame_count = frame_count, frame_size = frame_size, spawn_multiplier = spawn_multiplier, fps = (1000/frame_delay), interleaved_frames_status = interleaved_frames.to_string().replace("true", "on").replace("false", "off"));
                }
            } else {
                print!("{}", " ".repeat(110));
            }

            thread::sleep(std::time::Duration::from_millis(frame_delay.try_into().unwrap()));

            // Pauses sim
            while unsafe { PAUSE } == true {
                thread::sleep(std::time::Duration::from_millis(10));
            }
        }
        frame_count += 1;
    }

    if frame_delay <= 0 {
        println!("\rQuit simulation after {frame_count} frames\nTargeted fps was \nInterleaved frames were {interleaved_frames_status}", frame_count = frame_count, interleaved_frames_status = interleaved_frames.to_string().replace("true", "on").replace("false", "off"));
    } else {
        println!("\rQuit simulation after {frame_count} frames\nTargeted fps was {fps}\nInterleaved frames were {interleaved_frames_status}", frame_count = frame_count, fps = (1000/frame_delay), interleaved_frames_status = interleaved_frames.to_string().replace("true", "on").replace("false", "off"));
    }

    thread::sleep(std::time::Duration::from_millis(1000));

    unsafe { RUN = false; }
    keybind_thread.join().unwrap();
    std::process::exit(0);
}
