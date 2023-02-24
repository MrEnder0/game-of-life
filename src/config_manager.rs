use std::thread;
use rand::Rng;
use ini::Ini;

pub(crate) fn load_config() -> (usize, u64, usize, char, char, u64, bool) {
    // generate settings file if it doesn't exist
    if !std::path::Path::new("settings.ini").exists() {
        let mut create_settings = Ini::new();
        create_settings.with_section(Some("settings"))
            .set("frame_size", "60")
            .set("frame_delay", "150")
            .set("spawn_multiplier", "25")
            .set("filled_tile", "🟩")
            .set("empty_tile", "🟥")
            .set("starting_seed", rand::thread_rng().gen_range(-2_147_483_647..2_147_483_647).to_string().as_str())
            .set("use_seed", "false");
        create_settings.write_to_file("settings.ini").unwrap();
    }

    // loads settings
    let settings = Ini::load_from_file("settings.ini").unwrap();
    let frame_size = settings.get_from(Some("settings"), "frame_size").unwrap().parse::<usize>().unwrap();
    let frame_delay = settings.get_from(Some("settings"), "frame_delay").unwrap().parse::<u64>().unwrap();
    let spawn_multiplier = settings.get_from(Some("settings"), "spawn_multiplier").unwrap().parse::<usize>().unwrap();
    let filled_tile = settings.get_from(Some("settings"), "filled_tile").unwrap().parse::<char>().unwrap();
    let empty_tile = settings.get_from(Some("settings"), "empty_tile").unwrap().parse::<char>().unwrap();
    let starting_seed = settings.get_from(Some("settings"), "starting_seed").unwrap().parse::<u64>().unwrap();
    let use_seed = settings.get_from(Some("settings"), "use_seed").unwrap().parse::<bool>().unwrap();

    if empty_tile == filled_tile {
        println!("Filled tile and empty tile cannot be the same; please change them in settings.ini; Exiting in 5 seconds...");
        thread::sleep(std::time::Duration::from_millis(5000));
        std::process::exit(0);
    }

    return (frame_size, frame_delay, spawn_multiplier, filled_tile, empty_tile, starting_seed, use_seed);
}