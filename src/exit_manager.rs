use crate::RUN;

use console::Term;

fn do_nothing() {
    // do nothing... shocker
}

pub(crate) fn exit_keybind() {
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