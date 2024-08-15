use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

	let mut total: u64 = 0;
	for mut line in contents.lines().map(|l| String::from(l)) {

		print!("Line: {} | ", line);
		line = line.replace("one", "on1e")
				   .replace("two", "tw2o")
				   .replace("three", "thr3ee")
				   .replace("four", "fo4ur")
				   .replace("five", "fi5ve")
				   .replace("six", "si6x")
				   .replace("seven", "se7ven")
				   .replace("eight", "eig8ht")
				   .replace("nine", "ni9ne");

		print!("{} => ", line);

		line.retain(|c| c.is_ascii_digit());
		
		let mut nb_str = line.chars().next().unwrap().to_string();
		nb_str.push_str(&line.chars().last().unwrap().to_string());
		println!("{}", nb_str);
		let nb = nb_str.parse::<u64>().unwrap();
		total += nb;
	}

	println!("Total: {}", total);

    Ok(())
}