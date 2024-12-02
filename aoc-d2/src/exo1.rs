use std::fs::File;
use std::io::{BufRead, BufReader};

fn main()  -> std::io::Result<()> {
	let file = File::open("input.txt")?;	
	let buf_reader = BufReader::new(file);

	let mut sum_safe = 0;
	for line in buf_reader.lines().flatten() {
		let data: Vec<u32> = line.split_ascii_whitespace().map(|e| e.parse::<u32>().unwrap()).collect();
		let result = data.windows(2).all(|e| { e[0] > e[1] && (e[0] - e[1]) <= 3 }) || data.windows(2).all(|e| { e[0] < e[1] && (e[1] - e[0]) <= 3 });
		sum_safe += if result { 1 } else { 0 };
	}

	println!("Result: {sum_safe}");

	Ok(())
}
