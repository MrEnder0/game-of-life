use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut main_layer = [[0u8; 10]; 10];
    let mut possible_layer = [[0u8; 10]; 10];

    main_layer[0][rng.gen_range(0..10)] = 1;

    //prints out the main layer in a more readable format
    for i in 0..10 {
        for j in 0..10 {
            print!("{}", main_layer[i][j]);
        }
        println!();
    }
}
