use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn read_coords(file_path: &str) -> HashMap<String, Vec<f32>> {
        
    let file = File::open(file_path).unwrap();
    
    let reader = BufReader::new(file);

    let mut coords = HashMap::new();

    let mut index:u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        let name = values[0].to_string();
        let x = values[1].parse().unwrap();  //x coordinate
        let y: f32 = values[2].parse().unwrap(); //y coordinate
        let w: f32 = values[3].parse().unwrap(); //weight
        let vector = coords.entry(name).or_insert(vec![]); // add to coords
        vector.push(x);
        vector.push(y);
        vector.push(index as f32);
        vector.push(w as f32);
        index += 1;
    }

    coords
}

pub fn read_names(file_path: &str) -> Vec<String> {
        
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut names = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();
        let name = values[0].to_string();
        names.push(name);
    }

    names
}
