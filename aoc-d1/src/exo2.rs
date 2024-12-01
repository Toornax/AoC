use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
	// Input parsing
	let mut data1: Vec<u32> = Vec::new();
	let mut data2: Vec<u32> = Vec::new();
	let file = File::open("input.txt")?;	
    let buf_reader = BufReader::new(file);

	for line in buf_reader.lines().flatten() {
		let first_number = line.split_ascii_whitespace().next().unwrap().parse::<u32>().unwrap();
		let second_number = line.split_ascii_whitespace().last().unwrap().parse::<u32>().unwrap();
		data1.push(first_number);
		data2.push(second_number);
	}

	// Data processing
	let mut num_map: HashMap<u32, u32> = HashMap::new();
	let mut sum = 0;

	for val in data2 {
		if let Some(iter) = num_map.get_mut(&val) {
			*iter += 1;
		} else {
			num_map.insert(val, 1);
		}
	}

	for val in data1 {
		if let Some(iter) = num_map.get(&val) {
			sum += val * iter
		} 
	}

	println!("Result is {sum}");

	Ok(())
}
