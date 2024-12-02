use std::fs::File;
use std::io::{BufRead, BufReader};

fn main()  -> std::io::Result<()> {
	let file = File::open("input.txt")?;	
	let buf_reader = BufReader::new(file);

	let mut sum_safe = 0;
	for line in buf_reader.lines().flatten() {
		let data: Vec<u32> = line.split_ascii_whitespace().map(|e| e.parse::<u32>().unwrap()).collect();		
		let result = data.windows(2).all(|e| { e[0] > e[1] && (e[0] - e[1]) <= 3 }) || data.windows(2).all(|e| { e[0] < e[1] && (e[1] - e[0]) <= 3 });
		if result {
			sum_safe += if result { 1 } else { 0 };
		} else {
			for i in 0..data.len() {
				let mut tmp_data = data.clone();
				tmp_data.remove(i);
				let result = tmp_data.windows(2).all(|e| { e[0] > e[1] && (e[0] - e[1]) <= 3 }) || tmp_data.windows(2).all(|e| { e[0] < e[1] && (e[1] - e[0]) <= 3 });
				if result {
					sum_safe += if result { 1 } else { 0 };
					break;
				}
			}
		}
	}

	println!("Result: {sum_safe}");

	Ok(())
}
