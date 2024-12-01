use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
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

	data1.sort();
	data2.sort();

	let mut sum = 0;
	for i in 0..data1.len() {
		sum += data1.get(i).unwrap().abs_diff(*data2.get(i).unwrap());
	}

	println!("Result is: {sum}");

	Ok(())
}
