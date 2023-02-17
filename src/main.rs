use std::thread;
use rand::Rng;

fn main() {
    const SIZE: usize = 75;

    let mut rng = rand::thread_rng();
    let mut main_layer = [[0u8; SIZE]; SIZE];
    let mut possible_layer = [[0u8; SIZE]; SIZE];

    for _i in 0..SIZE*20 {
        let init_pos_x = rng.gen_range(1..SIZE-1);
        let init_pos_y = rng.gen_range(1..SIZE-1);
        main_layer[init_pos_x][init_pos_y] = 1;
    }

    while 1==1 {
        for x in 1..SIZE-1 {
            for y in 1..SIZE-1 {
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
        
        for x in 1..SIZE-1 {
            for y in 1..SIZE-1 {
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
        for x in 1..SIZE-1 {
            for y in 1..SIZE-1 {
                print!("{}", main_layer[x][y].to_string().replace("0", " ").replace("1", "█"));
            }
            println!("");
        }
        thread::sleep(std::time::Duration::from_millis(150));
    }
}
