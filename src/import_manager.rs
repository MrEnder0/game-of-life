use std::env;

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
    //read file and do stuff soon
}