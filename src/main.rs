use std::thread;
use rand::Rng;
use ini::Ini;

fn main() {
    let settings = Ini::load_from_file("settings.ini").unwrap();
    let size = settings.get_from(Some("settings"), "size").unwrap().parse::<usize>().unwrap();
    let filled = settings.get_from(Some("settings"), "filled").unwrap();
    let empty = settings.get_from(Some("settings"), "empty").unwrap();
    let delay = settings.get_from(Some("settings"), "delay").unwrap().parse::<u64>().unwrap();

    let mut rng = rand::thread_rng();
    let mut main_layer = vec![vec![0; size]; size];
    let mut possible_layer = vec![vec![0; size]; size];

    for _i in 0..size*20 {
        let init_pos_x = rng.gen_range(1..size-1);
        let init_pos_y = rng.gen_range(1..size-1);
        main_layer[init_pos_x][init_pos_y] = 1;
    }

    while 1==1 {
        for x in 1..size-1 {
            for y in 1..size-1 {
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
        
        for x in 1..size-1 {
            for y in 1..size-1 {
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
        for x in 1..size-1 {
            for y in 1..size-1 {
                print!("{}", main_layer[x][y].to_string().replace("0", empty).replace("1", filled));
            }
            println!("");
        }
        thread::sleep(std::time::Duration::from_millis(delay));
    }
}
