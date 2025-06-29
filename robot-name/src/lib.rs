use rand::prelude::*;

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

static USED_NAMES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn get_used_names() -> &'static Mutex<HashSet<String>> {
    USED_NAMES.get_or_init(|| Mutex::new(HashSet::new()))
}

fn add_name(name: &str) {
    let names = get_used_names();
    let mut set = names.lock().unwrap();

    set.insert(name.to_string());
}

fn check_name(name: &str) -> bool {
    let names = get_used_names();
    let set = names.lock().unwrap();

    set.contains(name)
}

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        let name = Self::get_name_and_insert_global();
        Robot { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::get_name_and_insert_global();
    }

    fn get_name_and_insert_global() -> String {
        let mut name = Self::generate_name();
        while check_name(&name) {
            name = Self::generate_name();
        }
        add_name(&name);
        name
    }

    // format of two uppercase letters followed by three digits, such as RX837 or BC811
    fn generate_name() -> String {
        let mut rng = rand::rng();
        let mut name = String::new();

        for n in 0..5 {
            if n < 2 {
                let letter = rng.random_range('A'..='Z');
                name.push(letter);
            } else {
                let num: u8 = rng.random_range(0..9);
                name.push_str(&num.to_string());
            }
        }

        name
    }
}
