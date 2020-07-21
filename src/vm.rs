
use crate::vm_data::{VError, Atom};
use crate::input::{Input, InputError};

pub fn run_vm(file_name : &str) {
    let file = std::fs::read_to_string(file_name)
        .unwrap()
        .char_indices()
        .collect::<Vec<(usize, char)>>();
    let mut input = Input::new( &file );

    let mut stack : Vec<Atom> = vec![];
    let mut dict : Vec<Atom> = vec![];

    while input.more() {
        let sym = input.parse_symbol().unwrap();

    }
}


fn read_file(file_name : &str) -> Result<Atom, VError> {
    match std::fs::read_to_string(file_name) {
        Ok(f) => Ok(Atom::String(f)),
        Err(_) => Err(VError::Todo),
    }
}

