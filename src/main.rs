use console::Term;
use std::thread;
use rand::{Rng, rngs::StdRng, SeedableRng};
use ini::Ini;

static mut RUN: bool = true;

fn do_nothing() {
    // do nothing... shocker
}

fn keybinds() {
    let stdout = Term::buffered_stdout();
    if let Ok(character) = stdout.read_char() {
        match character {
            'q' => {
                unsafe { RUN = false; }
            },
            _ => do_nothing()
        }
    }
}

fn main() {
    // generate settings file if it doesn't exist
    if !std::path::Path::new("settings.ini").exists() {
        let mut create_settings = Ini::new();
        create_settings.with_section(Some("settings"))
            .set("frame_size", "60")
            .set("frame_delay", "150")
            .set("spawn_multiplier", "25")
            .set("filled_tile", "🟩")
            .set("empty_tile", "🟥")
            .set("starting_seed", "0")
            .set("use_seed", "false");
        create_settings.write_to_file("settings.ini").unwrap();
    }

    // loads settings
    let settings = Ini::load_from_file("settings.ini").unwrap();
    let frame_size = settings.get_from(Some("settings"), "frame_size").unwrap().parse::<usize>().unwrap();
    let frame_delay = settings.get_from(Some("settings"), "frame_delay").unwrap().parse::<u64>().unwrap();
    let spawn_multiplier = settings.get_from(Some("settings"), "spawn_multiplier").unwrap().parse::<usize>().unwrap();
    let filled_tile = settings.get_from(Some("settings"), "filled_tile").unwrap();
    let empty_tile = settings.get_from(Some("settings"), "empty_tile").unwrap();
    let starting_seed = settings.get_from(Some("settings"), "starting_seed").unwrap().parse::<u64>().unwrap();
    let use_seed = settings.get_from(Some("settings"), "use_seed").unwrap().parse::<bool>().unwrap();

    let mut rng = if use_seed == true {
        StdRng::seed_from_u64(starting_seed)
    } else {
        StdRng::from_entropy()
    };
    let mut main_layer = vec![vec![0; frame_size]; frame_size];
    let mut possible_layer = vec![vec![0; frame_size]; frame_size];

    for _i in 0..frame_size*spawn_multiplier {
        let init_pos_x = rng.gen_range(1..frame_size-1);
        let init_pos_y = rng.gen_range(1..frame_size-1);
        main_layer[init_pos_x][init_pos_y] = 1;
    }

    let keybind_thread = thread::spawn(move || {
        keybinds();
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
                print!("{}", main_layer[x][y].to_string().replace("0", empty_tile).replace("1", filled_tile));
            }
            println!("");
        }

        thread::sleep(std::time::Duration::from_millis(frame_delay));
    }

    println!("Quitting...");
    keybind_thread.join().unwrap();
}
