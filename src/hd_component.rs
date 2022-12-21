use std::{
  fs
};

pub fn get_sorce_code (filepath: &str) -> String {
  fs::read_to_string(filepath).unwrap()
}