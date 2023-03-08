use std::env;
use std::thread;

pub(crate) fn check_imports() -> bool {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        if filename.ends_with(".life") {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub(crate) fn parse_import() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = std::fs::read_to_string(filename).unwrap();
    let lines = file.lines();
    
    let mut iter = lines.into_iter();
    while let Some(line) = iter.next() {
        if line.starts_with('%') && &line[1..4] == "VER" {
            match &line[5..] {
                "ELFv1" => {
                    let frame = iter.next().unwrap().split('|').collect::<Vec<_>>();
                    for row in frame {
                        println!("{}", row);
                    }
                }
                "ELFv2" => {
                    let mut itter_clone = iter.clone();
                    itter_clone.next();
                    let frame: Vec<String> = itter_clone.map(|s| s.to_string()).collect();
                    for row in &frame {
                        println!("{}", row.replace(".", "0").replace("#", "1"));
                    }
                }
                _ => println!("Loaded unknown life format found only supports 'ELFv1', and 'ELFv2'..."),
            }
        }
    }
    
    thread::sleep(std::time::Duration::from_millis(3000));
}