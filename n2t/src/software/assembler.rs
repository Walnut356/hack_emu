use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

pub fn to_machine_code(path: &Path) {
    let file = File::open(path).unwrap();
    // let mut output = File::create(path.parent().unwrap()).unwrap();
    let mut stream = BufReader::new(file);

    let mut buffer = String::new();

    stream.read_to_string(&mut buffer);

    let mut symbol_table: HashMap<String, usize> = HashMap::new();

    let mut i = 0usize;
    while i < 16 {
        symbol_table.insert(format!("R{i:?}"), i);
        i += 1;
    }
    symbol_table.insert("SP".to_string(), 0);
    symbol_table.insert("LCL".to_string(), 1);
    symbol_table.insert("ARG".to_string(), 2);
    symbol_table.insert("THIS".to_string(), 3);
    symbol_table.insert("THAT".to_string(), 4);
    symbol_table.insert("SCREEN".to_string(), 16384);
    symbol_table.insert("KBD".to_string(), 24576);

    let mut first_pass: Vec<String> = Vec::new();

    for line in buffer.lines() {
        // I could probably use regex but it seems a bit excessive for something so constrained

        let mut trimmed = line.trim();
        trimmed = trimmed.split_whitespace().next().unwrap_or(trimmed);
        if trimmed.starts_with("//") | trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('(') {
            symbol_table.insert(trimmed[1..trimmed.len() - 1].to_string(), first_pass.len());
            continue;
        }
        if trimmed.starts_with('@') {
            first_pass.push(trimmed.to_string());
            if let Ok(num) = trimmed[1..].parse::<usize>() {
                continue;
            } else {
                symbol_table.insert(trimmed[1..].trim().to_string(), first_pass.len());
            }
            continue;
        }
        first_pass.push(trimmed.to_string());
    }

    #[cfg(debug_assertions)]
    println!("{:?}\n", first_pass);
    #[cfg(debug_assertions)]
    println!("{:?}", symbol_table);

    let mut out_path = path.clone().file_stem().unwrap().to_os_string();
    out_path.push(path.file_name().unwrap());
    out_path.push(".hack");

    let mut out_file = File::create(out_path);

    for instr in first_pass {}
}
