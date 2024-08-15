use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Symbol {
	pos_x: u64,
	pos_y: u64
}

#[derive(Debug, Clone)]
struct Number {
	pos_x_beg: u64,
	pos_x_end: u64,
	pos_y: u64,
	value: u64
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

	let mut gear_list: Vec<Symbol> = Vec::new();
	let mut number_list: Vec<Number> = Vec::new();

	for (i, line) in contents.lines().enumerate() {
		let line_str = line.to_string();
		let mut tmp_val = String::new();
		let mut x_beg = 0;
		let mut x_end = 0;
		let mut flag_num = false;

		for (j, c) in line_str.chars().enumerate() {
			if c.is_ascii_digit() {
				tmp_val.push(c);
				if !flag_num { x_beg = j; flag_num = true; }
				x_end = j;
			} else {
				if flag_num {
					let pos = Number { pos_x_beg: x_beg as u64, pos_x_end: x_end as u64, pos_y: i as u64, value: tmp_val.parse::<u64>().unwrap() };
					number_list.push(pos);
					x_beg = 0;
					flag_num = false;
					tmp_val = String::new();
				}

				if c == '*' {
					let pos = Symbol { pos_x: j as u64, pos_y: i as u64 };
					gear_list.push(pos);
				}
			}

			if (j+1) >= line_str.len() && flag_num {
				let pos = Number { pos_x_beg: x_beg as u64, pos_x_end: x_end as u64, pos_y: i as u64, value: tmp_val.parse::<u64>().unwrap() };
				number_list.push(pos);
				x_beg = 0;
				flag_num = false;
				tmp_val = String::new();
			}
		}
	}


	let mut total = 0;

	for gear in &gear_list {
		let mut nb_valid: Vec<Number> = Vec::new();

		for number in &number_list {
			let mut x_is_ok = false;
			let mut y_is_ok = false;

			if number.pos_x_beg > 0 {
				if gear.pos_x == (number.pos_x_beg - 1) {
					x_is_ok = true;
				}
			}

			if gear.pos_x <= (number.pos_x_end + 1) && gear.pos_x >= number.pos_x_beg {
				x_is_ok = true;
			}

			if number.pos_y > 0 {
				if gear.pos_y == (number.pos_y - 1) {
					y_is_ok = true;
				}
			}

			if gear.pos_y <= (number.pos_y + 1) && gear.pos_y >= number.pos_y {
				y_is_ok = true;
			}

			if x_is_ok && y_is_ok {
				nb_valid.push(number.clone());
			}
		}

		if nb_valid.len() == 2 {
			total += nb_valid[0].value * nb_valid[1].value;
		}
	}
	
	println!("Total: {:?}", total);

	Ok(())
}
