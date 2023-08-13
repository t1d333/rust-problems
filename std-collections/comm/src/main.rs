#![forbid(unsafe_code)]
use std::{collections::HashSet, fs::File, io::BufRead, io::BufReader};

#[derive(Debug)]
pub struct Config {
    first_path: String,
    second_path: String,
}

fn main() {
    let (first_path, second_path) = read_config();

    let config = Config {
        first_path,
        second_path,
    };

    let strs = comm(&config);

	for st in strs {
		println!("{}", st);
	}
}

pub fn comm(config: &Config) -> Vec<String> {
    let mut set: HashSet<String> = HashSet::new();
    let mut result: Vec<String> = Vec::new();

    let first_file = File::open(&config.first_path).expect(&format!(
        "Failed to open file by path: {}",
        config.first_path
    ));
    let second_file = File::open(&config.second_path).expect(&format!(
        "Failed to open file by path: {}",
        config.second_path
    ));

    let mut reader = BufReader::new(first_file);

    for line in reader.lines() {
        if let Ok(s) = line {
            set.insert(s);
        } else {
            break;
        }
    }

    reader = BufReader::new(second_file);

    for line in reader.lines() {
        if let Ok(s) = line {
            if set.contains(&s) {
                set.remove(&s);
                result.push(s);
            }
        } else {
            break;
        }
    }

    result
}

pub fn read_config() -> (String, String) {
    let args = std::env::args().collect::<Vec<String>>();
    let first_path = args
        .get(1)
        .expect("Usage: ./comm <path to first file> <path to second file>");
    let second_path = args
        .get(2)
        .expect("Usage: ./comm <path to first file> <path to second file>");

    (first_path.to_string(), second_path.to_string())
}
