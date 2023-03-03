use crate::RUN;
use crate::PAUSE;
use crate::DEV;

use console::Term;

fn do_nothing() {
    // Does nothing... shocker
}

pub(crate) fn init_keybinds() {
    while unsafe { RUN } == true {
        let stdout = Term::buffered_stdout();
        if let Ok(character) = stdout.read_char() {
            match character {
                'q' => {
                    unsafe { RUN = false; }
                },
                'p' => {
                    if unsafe { PAUSE == true } {
                        unsafe { PAUSE = false; }
                    } else {
                        unsafe { PAUSE = true; }
                    }
                },
                'd' => {
                    if unsafe { DEV == true } {
                        unsafe { DEV = false; }
                    } else {
                        unsafe { DEV = true; }
                    }
                },
                _ => do_nothing()
        }
    }
    }
}