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
                "LIFEv1.05" => {
                    let mut itter_clone = iter.clone();
                    itter_clone.next();
                    let frame: Vec<String> = itter_clone.map(|s| s.to_string()).collect();
                    for row in &frame {
                        println!("{}", row.replace(".", "0").replace("*", "1"));
                    }
                }
                "LIFEv1.06" => {
                    let mut itter_clone = iter.clone();
                    let mut width = 0;
                    let mut height = 0;
                    
                    // skip the first line of the input
                    itter_clone.next();
                    
                    // read in the coordinates and update the width and height
                    for line in itter_clone {
                        let coords: Vec<i32> = line
                            .split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect();
                        let x = coords[0];
                        let y = coords[1];
                    
                        if x.abs() > width {
                            width = x.abs();
                        }
                        if y.abs() > height {
                            height = y.abs();
                        }
                    }
                    
                    // create the grid and set the cells based on the coordinates
                    let mut grid = vec![vec![0; (width * 2 + 1) as usize]; (height * 2 + 1) as usize];
                    let iter_clone = iter.clone();
                    for line in iter_clone {
                        let coords: Vec<i32> = line
                            .split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect();
                        let x = coords[0];
                        let y = coords[1];
                        grid[(y + height) as usize][(x + width) as usize] = 1;
                    }
                    
                    // print out the grid
                    for row in &grid {
                        for cell in row {
                            print!("{}", cell);
                        }
                        println!();
                    }   
                }
                _ => println!("Loaded unknown life format found only supports 'ELFv1', 'ELFv2', 'LIFEv1.05', and 'LIFEv1.06'...")
            }
        }
    }
    
    thread::sleep(std::time::Duration::from_millis(3000));
}