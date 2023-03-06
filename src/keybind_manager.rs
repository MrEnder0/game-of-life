use crate::RUN;
use crate::PAUSE;
use crate::DEV;

use std::sync::atomic::Ordering;
use console::Term;

fn do_nothing() {
    // Does nothing... shocker
}

pub(crate) fn init_keybinds() {
    while RUN.load(Ordering::SeqCst) {
        let stdout = Term::buffered_stdout();
        if let Ok(character) = stdout.read_char() {
            match character {
                'q' => {
                    RUN.store(false, Ordering::Relaxed);
                },
                'p' => {
                    if PAUSE.load(Ordering::SeqCst) == true {
                        PAUSE.store(false, Ordering::Relaxed);
                    } else {
                        PAUSE.store(true, Ordering::Relaxed);
                    }
                },
                'd' => {
                    if DEV.load(Ordering::SeqCst) == true {
                        DEV.store(false, Ordering::Relaxed);
                    } else {
                        DEV.store(true, Ordering::Relaxed);
                    }
                },
                _ => do_nothing()
        }
    }
    }
}